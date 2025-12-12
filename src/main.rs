use chip_8::chip8::CPU;
use chip_8::rom;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::error::Error;
use std::time::Duration;

fn main() -> Result<(), Box<dyn Error>> {
    let mut cpu = CPU::new();
    let program = rom::load_rom();

    cpu.load_rom(&program);

    let sdl2_context = sdl2::init()?;
    let video_subsystem = sdl2_context.video()?;

    let window = video_subsystem
        .window("Chip-8", 640, 320)
        .position_centered()
        .build()?;

    let mut canvas = window.into_canvas().build()?;
    let mut event_pump = sdl2_context.event_pump()?;

    loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    return Ok(());
                }

                _ => {}
            }
        }

        //tick
        cpu.run();

        canvas.set_draw_color(Color::BLACK);
        canvas.clear();
        canvas.set_draw_color(Color::WHITE);

        for (i, pixel) in cpu.frame_buffer.iter().enumerate() {
            if *pixel {
                //get the x and y from the frame buffer
                let x = i % 64;
                let y = i / 64;

                let rect = Rect::new((x * 10) as i32, (y * 10) as i32, 10, 10);
                canvas.fill_rect(rect)?;
            }
        }

        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
