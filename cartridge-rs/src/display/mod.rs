use std::{error::Error, time::Duration};

use sdl2::{VideoSubsystem, event::Event, keyboard::Keycode, pixels::Color};

pub struct Display {
    video_subsystem: VideoSubsystem,
    sdl_content: sdl2::Sdl,
}

impl Display {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let sdl_content = sdl2::init()?;
        let video_subsystem = sdl_content.video()?;

        Ok(Self {
            video_subsystem,
            sdl_content,
        })
    }

    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        let window = self
            .video_subsystem
            .window("Chip-8 Emulator", 800, 600)
            .position_centered()
            .build()?;

        let mut canvas = window.into_canvas().build()?;
        canvas.set_draw_color(Color::RGB(0, 255, 255));
        canvas.clear();
        canvas.present();

        let mut event_pump = self.sdl_content.event_pump()?;

        loop {
            canvas.set_draw_color(Color::BLACK);
            canvas.clear();

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

            canvas.present();
            std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
