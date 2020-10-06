use rand::Rng;
use std::fs;

pub struct Chip8 {
    memory: [u8; 4096],
    V: [u8; 16],
    stack: [u16; 16],
    gfx: [u8; 64 * 32],
    I: u16,
    sp: u16,
    pc: u16,
    cur_opcode: u16,
}

impl Chip8 {
    pub fn init(&mut self) {
        self.pc = 0x200;
    }

    pub fn load_program(&mut self) {
        let mut mem_ptr = 0x200;
        let buff = fs::read("prog").unwrap();
        for data in buff.iter() {
            self.memory[mem_ptr] = *data;
            mem_ptr += 1;
        }
    }

    pub fn cycle(&mut self) {
        let upper_opcode_byte: u16 = self.memory[self.pc as usize].into();
        let upper_opcode_byte = upper_opcode_byte << 8;
        let lower_opcode_byte: u16 = self.memory[(self.pc + 1) as usize].into();
        self.cur_opcode = upper_opcode_byte + lower_opcode_byte;
        println!("current opcode: {:#X}", self.cur_opcode);
        println!("program counter: {:#X}", self.pc);
        let reg_x = ((self.cur_opcode & 0x0F00) >> 8) as usize;
        let reg_y = ((self.cur_opcode & 0x00F0) >> 4) as usize;
        match self.cur_opcode & 0xF000 {
            0x0000 => {
                match self.cur_opcode & 0x00FF {
                    0x00E0 => {
                        // 0x00E0
                        self.gfx = [0; 64 * 32];
                    }
                    0x00EE => { // 0x00EE
                         //ret
                    }
                    _ => { // 0x0NNN
                         //call machine code routine
                    }
                }
            }
            0x1000 => {
                // 0x1NNN
                self.pc = self.cur_opcode & 0x0FFF;
            }
            0x2000 => { // 0x2NNN
                 // call subroutine
            }
            0x3000 => {
                // 0x3XNN
                if self.V[reg_x] == (self.cur_opcode & 0x00FF) as u8 {
                    self.pc += 2;
                }
            }
            0x4000 => {
                // 0x4XNN
                if self.V[reg_x] != (self.cur_opcode & 0x00FF) as u8 {
                    self.pc += 2;
                }
            }
            0x5000 => {
                //0x5XY0
                if self.V[reg_x] == self.V[reg_y] {
                    self.pc += 2;
                }
            }
            0x6000 => {
                // 0x6XNN
                self.V[reg_x] = (self.cur_opcode & 0x00FF) as u8;
            }
            0x7000 => {
                // 0x7XNN
                self.V[reg_x] += (self.cur_opcode & 0x00FF) as u8;
            }
            0x8000 => {
                match self.cur_opcode & 0x000F {
                    0x0000 => {
                        // 0x8XY0
                        self.V[reg_x] = self.V[reg_y];
                    }
                    0x0001 => {
                        // 0x8XY1
                        self.V[reg_x] = self.V[reg_x] | self.V[reg_y];
                    }
                    0x0002 => {
                        // 0x8XY2
                        self.V[reg_x] = self.V[reg_x] & self.V[reg_y];
                    }
                    0x0003 => {
                        // 0x8XY3
                        self.V[reg_x] = self.V[reg_x] ^ self.V[reg_y];
                    }
                    0x0004 => {
                        // 0x8XY4
                        self.V[reg_x] += self.V[reg_y];
                    }
                    0x0005 => {
                        // 0x8XY5
                        self.V[reg_x] -= self.V[reg_y];
                    }
                    0x0006 => {
                        // 0x8XY6
                        self.V[0xF] = self.V[reg_x] & 0xF0;
                        self.V[reg_x] = self.V[reg_x] << 1;
                    }
                    0x0007 => {
                        // 0x8XY7
                        self.V[reg_x] = self.V[reg_y] - self.V[reg_x];
                    }
                    0x000E => {
                        // 0x8XYE
                        self.V[0xF] = self.V[reg_x] & 0x0F;
                        self.V[reg_x] = self.V[reg_x] << 1;
                    }
                    _ => {
                        println!("unsupported opcode! {}", self.cur_opcode);
                    }
                }
            }
            0x9000 => {
                // 0x9XY0
                if self.V[reg_x] != self.V[reg_y] {
                    self.pc += 2;
                }
            }
            0xA000 => {
                // 0xANNN
                self.I = self.cur_opcode & 0x0FFF;
            }
            0xB000 => {
                // 0xBNNN
                self.pc = (self.V[0] as u16) + self.cur_opcode & 0x0FFF;
            }
            0xC000 => {
                //0xCXNN
                let con = (self.cur_opcode & 0x00FF) as u8;
                let mut rng = rand::thread_rng();
                let num: u8 = rng.gen();
                self.V[reg_x] = num & con;
            }
            0xD000 => { // 0xDXYN
                 // draw
            }
            0xE000 => {
                match self.cur_opcode & 0x00FF {
                    0x009E => { // 0xEX9E
                         //key
                    }
                    0x00A1 => { // 0xEXA1
                         //key
                    }
                    _ => {
                        println!("unsupported opcode {}", self.cur_opcode);
                    }
                }
            }
            0xF000 => {
                match self.cur_opcode & 0x00FF {
                    0x0007 => { // 0xFX07
                         // timer
                    }
                    0x000A => { // 0xFX0A
                         // key
                    }
                    0x0015 => { // 0xFX15
                         // timer
                    }
                    0x0018 => { // 0xFX18
                         // timer
                    }
                    0x001E => {
                        // 0xFX1E
                        self.I += (self.V[reg_x] as u16);
                    }
                    0x0029 => { // 0xFX29
                         // i loc sprite in VX
                    }
                    0x0033 => { // 0xFX33
                         // BDC of VX
                    }
                    0x0055 => {
                        // 0xFX55
                        // reg dump V0 - VX in mem starting at I
                        let mut idx = self.I;
                        for i in 0..reg_x {
                            self.memory[idx as usize] = self.V[i as usize];
                            idx += 1;
                        }
                    }
                    0x0065 => {
                        // 0xFX65
                        // reg fill V0 - VX from mem starting at I
                        let mut idx = self.I;
                        for i in 0..reg_x {
                            self.V[i as usize] = self.memory[idx as usize];
                            idx += 1;
                        }
                    }
                    _ => {
                        println!("unsupported op code {}", self.cur_opcode);
                    }
                }
            }
            _ => {
                println!("unsupported opcode! {}", self.cur_opcode);
            }
        }

        self.pc += 2;
    }
}

fn main() {
    let mut emulator = Chip8 {
        memory: [0; 4096],
        V: [0; 16],
        stack: [0; 16],
        gfx: [0; 64 * 32],
        I: 0 as u16,
        sp: 0 as u16,
        pc: 0 as u16,
        cur_opcode: 0 as u16,
    };
    emulator.init();
    emulator.load_program();

    loop {
        emulator.cycle();
    }
}
