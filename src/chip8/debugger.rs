#[derive(Debug, Default)]
pub struct Debugger {
    debug: Propagate,
    list: Vec<String>,
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
            list: vec![],
        }
    }

    pub fn propagate(&mut self, pc: u8, first_byte: u16, second_byte: u16, opcode: u16, sp: u8) {
        if self.debug == Propagate::Enable {
            let key = format!(
                "Instructions: {:04X} | Memory: {:04X}{:04X} | PC: {:04X} | SP: {:04X}",
                opcode, first_byte, second_byte, pc, sp
            );

            let length = self.list.len();
            let is_spam = length >= 3
                && self.list[length - 1] == key
                && self.list[length - 2] == key
                && self.list[length - 3] == key;

            if !is_spam {
                println!("{}", key)
            }

            if length >= 20 {
                self.list.remove(0);
            }

            self.list.push(key);
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
