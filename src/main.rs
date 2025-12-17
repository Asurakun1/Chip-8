use chip_8::chip8::CPU;
use chip_8::rom;
use sdl2::pixels::Color;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut cpu = CPU::new();
    let program = rom::load_rom();

    cpu.load_rom(&program);
    let mut display = chip_8::display::Display::new(1280, 640, Color::GREEN)?;

    display.run(&mut cpu)
}
