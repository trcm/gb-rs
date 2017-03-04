use byteorder::{ByteOrder, LittleEndian};
use cpu::cpu::CPU;

pub struct Debug {
    pub location: u16,
    pub breakpoints: Vec<u16>,
}

pub enum Actions  {
    BREAK,
    EXIT,
    STEP,
    NOOP
}

impl Debug {
    pub fn new() -> Debug {
        let debugger = Debug {
            location: 0,
            breakpoints: Vec::new(),
        };
        
        debugger
    }

    pub fn set_breakpoint(&mut self, location: u16) {
        self.breakpoints.push(location);
    }

    pub fn check_breakpoints(&self, location: u16) -> bool {
        if self.breakpoints.contains(&location) {
            return true;
        }
        false
    }

    pub fn disassemble(&self, cpu: &CPU) {
       // disassemble 
        let mut pc = cpu.pc as usize;
        let mut line_count = 0;
        while line_count < 20 {
            match cpu.memory.read_value_u8( pc ){
                0x05 => {
                    println!("0x{:02X}\tDEC B", pc);
                    pc += 1;
                }
                0x06 => {
                    println!("0x{:02X}\tLD B, 0x{:02X}", pc, cpu.memory.read_value_u8( pc + 1 ));
                    pc += 2;
                },
                0x0C => {
                    println!("0x{:02X}\tINC C", pc);
                    pc += 1;
                },
                0x0E => { 
                    println!("0x{:02X}\tLD C, 0x{:02X}", pc, cpu.memory.read_value_u8( pc + 1 ));
                    pc += 2;
                },
                0x11 => {
                    println!("0x{:02X}\tLD DE, 0x{:02X}{:X}", pc, cpu.memory.read_value_u8( pc + 2 ), cpu.memory.read_value_u8( pc + 1 ));
                    pc += 3;
                },
                0x17 => {
                    println!("0x{:02X}\tRLA", pc);
                    pc += 1;
                }
                0x1A => {
                    println!("0x{:02X}\tLD A, (DE)", pc);
                    pc += 1;
                },
                0x20 => {
                    // calculate jmp location
                    let mut location = LittleEndian::read_int(&[cpu.memory.read_value_u8( (pc + 1) as usize )], 1);
                    location  = (((pc + 2) as i32) + (((location as u8) as i8) as i32)) as i64;
                    println!("0x{:02X}\tJR NZ, 0x{:02X}", pc, location);
                    pc += 2;
                },
                0x21 => {
                    println!("0x{:02X}\tLD HL, 0x{:02X}{:X}", pc, cpu.memory.read_value_u8( (pc + 2) as usize ), cpu.memory.read_value_u8( (pc + 1) as usize ));
                    pc += 3;
                },
                0x22 => {
                    println!("0x{:02X}\tLD (HL-), A", pc);
                    pc += 1;
                },
                0x23 => {
                    println!("0x{:02X}\tINC HL", pc);
                    pc += 1;
                },
                0x31 => {
                    println!("0x{:02X}\tLD SP, 0x{:02X}{:X}", pc, cpu.memory.read_value_u8( (pc + 2) as usize ), cpu.memory.read_value_u8( (pc + 1) as usize ));
                    pc += 3;
                },
                0x32 => {
                    println!("0x{:02X}\tLD (HL-),A\t", pc);
                    pc += 1;
                },
                0x3E => {
                    println!("0x{:02X}\tLD A, 0x{:02X}", pc, cpu.memory.read_value_u8( pc + 1 ));
                    pc += 2;
                },
                0x4F => {
                    println!("0x{:02X}\tLD C, A", pc);
                    pc += 1;
                }
                0x77 => {
                    println!("0x{:02X}\tLD (HL), A", pc);
                    pc += 1;
                },
                0xAF => {
                    println!("0x{:02X}\tXOR A", pc);
                    pc += 1;
                },
                0xC1 => {
                    println!("0x{:02X}\tPOP BC", pc);
                    pc += 1;
                },
                0xC5 =>  {
                    println!("0x{:02X}\tPUSH BC", pc);
                    pc += 1;
                },
                0xCB => { // redirect 
                    match cpu.memory.read_value_u8( pc + 1 ) {
                        0x11 => println!("0x{:02X}\tRL C", pc + 1),
                        0x7C => println!("0x{:02X}\tBIT 7, H", pc + 1),
                        _ =>  println!("Not disassembled redirect 0x{:02X}", pc + 1),
                    };
                    pc += 2;
                },
                0xCD => {
                    println!("0x{:02X}\tCALL 0x{:02X}{:X}", pc, cpu.memory.read_value_u8( (pc + 2) as usize ), cpu.memory.read_value_u8( (pc + 1) as usize ));
                    pc += 3;
                },
                0xC9 => {
                    println!("0x{:02X}\tRET", pc);
                    pc += 1;
                }
                0xE0 => {
                    println!("0x{:02X}\tLDH ($FF00+0x{:02X}), A", pc, cpu.memory.read_value_u8( pc + 1 ));
                    pc += 2;
                },
                0xE2 => {
                    println!("0x{:02X}\tLD ($FF00+C), A", pc);
                    pc += 1;
                },
                _ => println!("Not disassembled 0x{:02X}, 0x{:02X}", pc, cpu.memory.read_value_u8( pc )) 
            }
            line_count += 1;
        }
    }

    pub fn parse_input(&mut self, input: &str, cpu: &CPU) -> Actions {

        let split: Vec<&str> = input.split(" ").collect();
        match split[0].as_ref() {
            ":q" => return Actions::EXIT,
            "quit" => return Actions::EXIT,
            "s" => return Actions::STEP,
            "step" => return Actions::STEP,
            "p" => {
                self.print_status(cpu);
                return Actions::NOOP
            },
            "c" => return Actions::BREAK,
            "continue" => return Actions::BREAK,
            "b" => {
                // set a breakpoint, assume numbers are in hex format
                if split.len() > 1 {
                    // let loc: u16 = split[1].parse().unwrap();
                    let loc: u16 = u16::from_str_radix(split[1], 16).unwrap();
                    self.set_breakpoint(loc);
                }
                return Actions::NOOP;
            }
            "bs" => {
                let mut i = 0;
                for b in &self.breakpoints {
                    println!("{}: 0x{:02X}", i, b);
                    i += 1;
                }
                return Actions::NOOP;
            },
            "bc" => {
                let loc: u16 = split[1].parse().unwrap();
                self.breakpoints.remove(loc as usize);
                return Actions::NOOP;
            },
            "l" => {
                // disassemble the next ten lines
                self.disassemble(cpu);
                return Actions::NOOP
            },
            _ => return Actions::NOOP,
        }
    }
    
    pub fn step(&self, cpu: &mut CPU) {
        cpu.cycle();
    }
    
    pub fn print_status(&self, cpu: &CPU) {
        println!("DEBUGGER PC: {}", cpu.pc);
        println!("{:?}", cpu);
    }

}
