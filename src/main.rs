use chip_8::chip8::CPU;
use chip_8::rom;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut display = chip_8::display::Display::new(800, 600)?;

    let mut cpu = CPU::new();
    let program = rom::load_rom();

    cpu.load_rom(&program);

    display.run(&mut cpu)
}
