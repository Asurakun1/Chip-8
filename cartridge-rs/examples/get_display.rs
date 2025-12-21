use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::rwops::RWops;
use sdl2::ttf;
use std::error::Error;

use cartridge_rs::display::Display;
use taffy::prelude::*;

pub struct Block<'a> {
    node: &'a Layout,
    rect: Rect,
}

fn main() -> Result<(), Box<dyn Error>> {
    /*
     * Initialization
     * Apply text fonts and render them to the inner box
     */

    let ttf_context = ttf::init()?;
    let bytes_from_file = include_bytes!("../Fonts/Modeseven-L3n5.ttf");
    let rwops = RWops::from_bytes(bytes_from_file)?;
    let font = ttf_context.load_font_from_rwops(rwops, 20)?;
    let surface = font.render("Hello World!").blended(Color::GREEN)?;

    let mut taffy: TaffyTree<()> = TaffyTree::new();

    let width = 800;
    let height = 600;

    let child = taffy.new_leaf(Style {
        size: Size {
            width: Dimension::from_percent(0.50),
            height: Dimension::from_percent(0.7),
        },
        justify_content: Some(JustifyContent::Center),
        align_items: Some(AlignItems::Center),
        ..Default::default()
    })?;

    let node = taffy.new_with_children(
        Style {
            size: Size {
                width: Dimension::from_length(width as f32),
                height: Dimension::from_length(height as f32),
            },
            justify_content: Some(JustifyContent::Center),
            align_items: Some(AlignItems::Center),
            ..Default::default()
        },
        &[child],
    )?;
    taffy.compute_layout(
        node,
        Size {
            width: AvailableSpace::Definite(width as f32),
            height: AvailableSpace::Definite(height as f32),
        },
    )?;

    let layout = taffy.layout(node)?;
    let child = taffy.layout(child)?;

    let sdlrect = Rect::new(
        child.content_box_x() as i32 * 2,
        child.content_box_y() as i32 * 2,
        child.content_box_width() as u32 / 2,
        child.content_box_height() as u32 / 2,
    );

    let block = Block {
        node: child,
        rect: sdlrect,
    };

    let display = Display::new(width, height)?;
    display.run(vec![layout, child])?;

    Ok(())
}
