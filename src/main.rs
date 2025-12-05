struct CPU {
    register: Register,
    stack: [u16; 64],
    frame_buffer: [bool; 64 * 32],
    memory: [u8; 4096],
}

struct Register {
    v_registers: [u8; 16],
    index_register: u16,
    pc: u16,
    delay_timer: u8,
    sound_timer: u8,
    stack_pointer: u8,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            register: Register {
                v_registers: [0; 16],
                index_register: 0,
                pc: 0x200, //starting point 512 bytes are always loaded into memory
                delay_timer: 0,
                sound_timer: 0,
                stack_pointer: 0,
            },
            stack: [0; 64],
            frame_buffer: [false; 64 * 32],
            memory: [0; 4096],
        }
    }

    pub fn run(&mut self) {
        //fetch
        let pc = self.register.pc as usize;
        let first_byte = self.memory[pc] as u16;
        let second_byte = self.memory[pc + 1] as u16;

        let opcode = first_byte << 8 | second_byte;

        //decode & execute
        self.execute(opcode);

        //increment pc
        self.register.pc += 2;
    }

    fn execute(&mut self, opcode: u16) {
        let digit = (opcode & 0xF000) >> 12;
        let x = (opcode & 0x0F00) >> 8;
        let y = (opcode & 0x00F0) >> 4;
        let n = opcode & 0x000F;
        let nnn = opcode & 0x0FFF;
        let kk = (opcode & 0x00FF) as u8;

        /*
         * CHIP-8 Instructions
         */
        match (digit, x, y, n) {
            (0, 0, 0xE, 0) => {
                //CLS
                self.frame_buffer = [false; 64 * 32]
            }
            (0, 0, 0xE, 0xE) => {
                //Return from subroutine
                self.register.stack_pointer -= 1;
                self.register.pc = self.stack[self.register.stack_pointer as usize];
            }

            (1, _, _, _) => {
                //Jump to location nnn
                self.register.pc = nnn;
            }

            (2, _, _, _) => {
                //2nnn
                //Call subroutine at nnn
                let sp = self.register.stack_pointer;
                self.stack[sp as usize] = self.register.pc;
                self.register.stack_pointer += 1;
                self.register.pc = nnn - 2;
            }
            (3, _, _, _) => {
                //3xkk
                // skip next instruction if Vx = kk;
                //
                let vx = self.register.v_registers[x as usize];
                if vx == kk {
                    self.register.pc += 2;
                }
            }
            (4, _, _, _) => {
                //4xkk
                // skip next instruction if Vx != kk
                let vx = self.register.v_registers[x as usize];
                if vx != kk {
                    self.register.pc += 2;
                }
            }

            (5, _, _, 0) => {
                //5xy0
                // Skip next instruction if Vx == Vy
                let vx = self.register.v_registers[x as usize];
                let vy = self.register.v_registers[y as usize];

                if vx == vy {
                    self.register.pc += 2;
                }
            }

            (6, _, _, _) => {
                //6xKK put value kk into register Vx
                self.register.v_registers[x as usize] = kk;
            }

            (7, _, _, _) => {
                //7xkk
                let vx = self.register.v_registers[x as usize];
                self.register.v_registers[x as usize] += vx.wrapping_add(kk);
            }

            /*
             * 8xy0-E instructions
             */
            (0, _, _, _) => {
                //nop
            }
            _ => (),
        }
    }
}

fn main() {
    let mut cpu = CPU::new();
    cpu.run();

    println!("Hello world!")
}
