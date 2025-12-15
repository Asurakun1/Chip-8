use crate::chip8::CPU;
use crate::chip8::debugger::Propagate;
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

    fn event(&self, event_pump: &mut EventPump, cpu: &mut CPU) {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    std::process::exit(0);
                }

                Event::KeyDown {
                    keycode: Some(Keycode::F1),
                    repeat: false,
                    ..
                } => {
                    /*
                     * Toggle the debug propagation for the cpu instructions
                     */
                    if *cpu.debug.get_status() == Propagate::Disable {
                        cpu.debug.enable();
                        println!("Debug Propagation Enabled");
                    } else {
                        cpu.debug.disable();
                        println!("Debug Propagation Disabled:");
                    }
                }

                /*
                 * Chip-8 Controls
                 * These controls go from 1-0 and A-F
                 * They are to be translated from SDL's input from
                 *     SDL2                CHIP-8
                 * [1][2][3][4]         [1][2][3][C]
                 * [Q][W][E][R]   =>    [4][5][6][D]
                 * [A][S][D][F]   =>    [7][8][9][E]
                 * [Z][X][C][V]         [A][0][B][F]
                 */
                Event::KeyDown {
                    keycode: Some(key), ..
                } => {
                    if let Some(key) = self.key2btn(key) {
                        cpu.keypad[key as usize] = true;
                        println!("Key: 0x{:X}", key)
                    }
                }

                Event::KeyUp {
                    keycode: Some(key), ..
                } => {
                    if let Some(key) = self.key2btn(key) {
                        cpu.keypad[key as usize] = false;
                    }
                }
                _ => {}
            }
        }
    }

    fn key2btn(&self, key: Keycode) -> Option<u16> {
        /*
         * Keys that are registered must be converted to their respective u16 for the Chip-8 CPU
         *
         * In the future these set of key bindings will be custom by user configuration
         */
        match key {
            Keycode::NUM_1 => Some(0x1),
            Keycode::NUM_2 => Some(0x2),
            Keycode::NUM_3 => Some(0x3),
            Keycode::NUM_4 => Some(0xC),
            Keycode::Q => Some(0x4),
            Keycode::W => Some(0x5),
            Keycode::E => Some(0x6),
            Keycode::R => Some(0xD),
            Keycode::A => Some(0x7),
            Keycode::S => Some(0x8),
            Keycode::D => Some(0x9),
            Keycode::F => Some(0xE),
            Keycode::Z => Some(0xA),
            Keycode::X => Some(0x0),
            Keycode::C => Some(0xB),
            Keycode::V => Some(0xF),
            _ => None,
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
            self.event(&mut event_pump, cpu);
            //tick
            cpu.run();

            self.render(&mut canvas, cpu)?;

            std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }
}
