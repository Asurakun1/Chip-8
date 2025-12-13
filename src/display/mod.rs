use crate::chip8::CPU;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::{EventPump, Sdl, VideoSubsystem};
use std::error::Error;
use std::time::Duration;

pub struct Display {
    sdl2_context: Sdl,
    height: u32,
    width: u32,
    video_subsystem: VideoSubsystem,
}

impl Default for Display {
    fn default() -> Self {
        if let Ok(display) = Self::new(640, 320) {
            display
        } else {
            panic!("The display has failed to initialize")
        }
    }
}

impl Display {
    pub fn new(width: u32, height: u32) -> Result<Self, Box<dyn Error>> {
        let sdl2_context = sdl2::init()?;
        let video_subsystem = sdl2_context.video()?;

        Ok(Self {
            sdl2_context,
            height,
            width,
            video_subsystem,
        })
    }

    fn event(&self, event_pump: &mut EventPump) {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    std::process::exit(0);
                }

                _ => {}
            }
        }
    }

    fn render(&self, canvas: &mut Canvas<Window>, cpu: &CPU) -> Result<(), Box<dyn Error>> {
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();
        canvas.set_draw_color(Color::WHITE);

        for (i, pixel) in cpu.frame_buffer.iter().enumerate() {
            if *pixel {
                //get the x and y from the 1D array frame buffer
                let x = i % 64;
                let y = i / 64;

                let rect = Rect::new(x as i32, y as i32, 1, 1);
                canvas.fill_rect(rect)?;
            }
        }
        canvas.present();
        Ok(())
    }

    pub fn run(&mut self, cpu: &mut CPU) -> Result<(), Box<dyn Error>> {
        let window = self
            .video_subsystem
            .window("Chip-8 Emulator", self.width, self.height)
            .position_centered()
            .build()?;

        let mut canvas = window.into_canvas().build()?;

        canvas.set_logical_size(64, 32)?;

        let mut event_pump = self.sdl2_context.event_pump()?;
        loop {
            self.event(&mut event_pump);
            //tick
            cpu.run();

            self.render(&mut canvas, cpu)?;

            std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }
}
