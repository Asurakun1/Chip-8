use rand::random_range;

pub struct CPU {
    register: Register,
    stack: [u16; 64],
    pub frame_buffer: [bool; 64 * 32],
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

impl Default for CPU {
    fn default() -> Self {
        self::CPU::new()
    }
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

    pub fn load_rom(&mut self, data: &[u8]) {
        let start = self.register.pc as usize;
        let end = start + data.len();

        if end > 4096 {
            panic!("ROM is too large to load into memory")
        }

        self.memory[start..end].copy_from_slice(data);
    }

    pub fn run(&mut self) {
        //fetch
        let pc = self.register.pc as usize;
        let first_byte = self.memory[pc] as u16;
        let second_byte = self.memory[pc + 1] as u16;

        let opcode = first_byte << 8 | second_byte;

        println!(
            "Instruction: {:04X} | Memory: {:04X}{:04X} | PC: {:04X} | SP: {:04X}",
            opcode, first_byte, second_byte, pc, self.register.stack_pointer
        );

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
                self.register.pc = nnn - 2;
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
                self.register.v_registers[x as usize] = vx.wrapping_add(kk);
            }

            /*
             * 8xy0-E instructions
             */
            (8, _, _, 0) => {
                //8xy0
                // Vx = Vy
                self.register.v_registers[x as usize] = self.register.v_registers[y as usize];
            }

            (8, _, _, 1) => {
                //8xy1
                // Vx = Vx OR Vy
                let vx = self.register.v_registers[x as usize];
                let vy = self.register.v_registers[y as usize];

                self.register.v_registers[x as usize] = vx | vy;
            }

            (8, _, _, 2) => {
                //8xy2
                // Vx = Vx AND Vy
                let vx = self.register.v_registers[x as usize];
                let vy = self.register.v_registers[y as usize];

                self.register.v_registers[x as usize] = vx & vy;
            }

            (8, _, _, 3) => {
                //8xy3
                // Vx = Vx XOR Vy
                let vx = self.register.v_registers[x as usize];
                let vy = self.register.v_registers[y as usize];

                self.register.v_registers[x as usize] = vx ^ vy;
            }

            (8, _, _, 4) => {
                //8xy4
                // Vx = Vx + Vy
                // Vf = carry
                let vx = self.register.v_registers[x as usize] as u16;
                let vy = self.register.v_registers[y as usize] as u16;
                if vx + vy > 255 {
                    self.register.v_registers[0xF] = 1;
                } else {
                    self.register.v_registers[0xF] = 0;
                }

                self.register.v_registers[x as usize] = (vx + vy) as u8;
            }

            (8, _, _, 5) => {
                //8xy5
                // Vx = Vx - Vy
                // VF = NOT Borrow
                let vx = self.register.v_registers[x as usize];
                let vy = self.register.v_registers[y as usize];

                if vx >= vy {
                    self.register.v_registers[0xF] = 1;
                } else {
                    self.register.v_registers[0xF] = 0;
                }

                self.register.v_registers[x as usize] = vx.wrapping_sub(vy);
            }

            (8, _, _, 6) => {
                //8xy6
                // Vx = Vx SHR 1
                let vx = self.register.v_registers[x as usize];
                if vx & 0x01 == 1 {
                    self.register.v_registers[0xF] = 1;
                } else {
                    self.register.v_registers[0xF] = 0;
                }
                self.register.v_registers[x as usize] = vx / 2;
            }

            (8, _, _, 7) => {
                //8xy7
                // Vx = Vy - Vx
                // Vf = NOT Borrow
                let vx = self.register.v_registers[x as usize];
                let vy = self.register.v_registers[y as usize];
                if vy >= vx {
                    self.register.v_registers[0xF] = 1;
                } else {
                    self.register.v_registers[0xF] = 0;
                }
                self.register.v_registers[x as usize] = vy.wrapping_sub(vx);
            }

            (8, _, _, 0xE) => {
                //8xyE
                // Vx = Vx SHL 1
                let vx = self.register.v_registers[x as usize];
                if (vx >> 7) & 1 == 1 {
                    self.register.v_registers[0xF] = 1;
                } else {
                    self.register.v_registers[0xF] = 0;
                }

                self.register.v_registers[x as usize] = vx * 2;
            }
            /*
             * End of 8xy0-E instructions
             */
            (9, _, _, 0) => {
                //9xy0
                // Vx != Vy
                let vx = self.register.v_registers[x as usize];
                let vy = self.register.v_registers[y as usize];

                if vx != vy {
                    self.register.pc += 2;
                }
            }

            (0xA, _, _, _) => {
                //Annn
                // I = nnn
                self.register.index_register = nnn;
            }
            (0xB, _, _, _) => {
                //Bnnn
                // Jump to location nnn + V0
                self.register.pc = (nnn + self.register.v_registers[0] as u16) - 2;
            }

            (0xC, _, _, _) => {
                //Cxkk
                // Vx = random byte AND kk
                let rand = random_range(0..=255) as u8;
                self.register.v_registers[x as usize] = rand & kk;
            }

            (0xD, _, _, _) => {
                //Dxyn
                //Display n-byte sprite starting at memory location I at (Vx, Vy)
                // VF = collision
                let vx = self.register.v_registers[x as usize];
                let vy = self.register.v_registers[y as usize];
                self.register.v_registers[0xF] = 0;

                for row in 0..n {
                    let addr = self.register.index_register + row;
                    let pixels = self.memory[addr as usize];

                    for col in 0..8 {
                        if (pixels >> (7 - col)) & 1 == 1 {
                            let x = (vx as u16 + col) % 64;
                            let y = (vy as u16 + row) % 32;

                            let index = (x + (y * 64)) as usize;

                            //XOR
                            if self.frame_buffer[index] {
                                self.frame_buffer[index] = false;
                                self.register.v_registers[0xF] = 1;
                            } else {
                                self.frame_buffer[index] = true;
                            }
                        }
                    }
                }
            }
            (0, _, _, _) => {
                //nop
            }
            _ => (),
        }
    }
}
