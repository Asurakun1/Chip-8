use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let display = cartridge_rs::display::Display::new()?;
    display.run()?;
    Ok(())
}
