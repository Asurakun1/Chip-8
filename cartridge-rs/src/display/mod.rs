use std::{error::Error, time::Duration};

use sdl2::{
    VideoSubsystem, event::Event, keyboard::Keycode, pixels::Color, rect::Rect, surface::Surface,
};
use taffy::Layout;

pub struct Display {
    video_subsystem: VideoSubsystem,
    sdl_content: sdl2::Sdl,
    width: u32,
    height: u32,
}

impl Display {
    pub fn new(width: u32, height: u32) -> Result<Self, Box<dyn Error>> {
        let sdl_content = sdl2::init()?;
        let video_subsystem = sdl_content.video()?;

        Ok(Self {
            video_subsystem,
            sdl_content,
            width,
            height,
        })
    }

    pub fn run(&self, layout: Vec<&Layout>) -> Result<(), Box<dyn Error>> {
        let window = self
            .video_subsystem
            .window("Chip-8 Emulator", self.width, self.height)
            .position_centered()
            .build()?;

        let mut canvas = window.into_canvas().build()?;
        let mut event_pump = self.sdl_content.event_pump()?;

        //let texture_creator = canvas.texture_creator();
        // let texture = texture_creator.create_texture_from_surface(&surface)?;

        // let target = Rect::new(50, 50, surface.width(), surface.height());

        loop {
            canvas.set_draw_color(Color::BLACK);
            canvas.clear();
            canvas.set_draw_color(Color::WHITE);

            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::ESCAPE),
                        ..
                    } => return Ok(()),

                    _ => {}
                }
            }

            for layout in layout.iter() {
                let x = layout.content_box_x();
                let y = layout.content_box_y();
                let width = layout.content_box_width();
                let height = layout.content_box_height();
                let rect = Rect::new(x as i32, y as i32, width as u32, height as u32);
                //canvas.copy(&texture, None, Some(self.block.get_rect()))?;
                canvas.draw_rect(rect)?;
            }

            canvas.present();
            std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
