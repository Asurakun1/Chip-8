#[derive(Debug, Default)]
pub struct Debugger {
    debug: Propagate,
}

#[derive(Debug, PartialEq, Eq, Default)]
pub enum Propagate {
    Enable,
    #[default]
    Disable,
}
impl Debugger {
    pub fn new() -> Self {
        Self {
            debug: Propagate::Enable,
        }
    }

    pub fn propagate(&mut self, pc: u8, first_byte: u16, second_byte: u16, opcode: u16, sp: u8) {
        if self.debug == Propagate::Enable {
            let key = format!(
                "Instructions: {:04X} | Memory: {:04X}{:04X} | PC: {:04X} | SP: {:04X}",
                opcode, first_byte, second_byte, pc, sp
            );
            println!("{}", key)
        }
    }

    pub fn enable(&mut self) {
        self.debug = Propagate::Enable;
    }

    pub fn disable(&mut self) {
        self.debug = Propagate::Disable;
    }

    pub fn get_status(&self) -> &Propagate {
        &self.debug
    }
}
