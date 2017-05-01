use byteorder::{ByteOrder, LittleEndian};
use cpu::cpu::CPU;
use cpu::op::Opcode;
use cpu::gb::Gameboy;

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

    pub fn disassemble(&self, machine: &Gameboy) {
       // disassemble 
        let mut pc = machine.cpu.pc as usize;
        let mut line_count = 0;
        while line_count < 40 {
            // println!("0x{:02X}\t{}", pc, Opcode::parse(pc as u16, machine.cpu.memory.read_value_u8(pc)));
            match machine.cpu.memory.read_value_u8( pc ){
                0x00 => {
                    println!("0x{:02X}\tNOP", pc);
                    pc += 1;
                },
                0x02 => {
                    println!("0x{:02X}\tLD (BC), A", pc);
                    pc += 1;
                },
                0x03 => {
                    println!("0x{:02X}\tINC BC", pc);
                    pc += 1;
                },
                0x04 => {
                    println!("0x{:02X}\tINC B", pc);
                    pc += 1;
                }
                0x05 => {
                    println!("0x{:02X}\tDEC B", pc);
                    pc += 1;
                },
                0x0B => {
                    println!("0x{:02X}\tDEC BC", pc);
                    pc += 1;
                },
                0x0D => {
                    println!("0x{:02X}\tDEC C", pc);
                    pc += 1;
                },
                0x1D => {
                    println!("0x{:02X}\tDEC E", pc);
                    pc += 1;
                },
                0x06 => {
                    println!("0x{:02X}\tLD B, 0x{:02X}", pc, machine.cpu.memory.read_value_u8( pc + 1 ));
                    pc += 2;
                },
                0x0C => {
                    println!("0x{:02X}\tINC C", pc);
                    pc += 1;
                },
                0x0E => { 
                    println!("0x{:02X}\tLD C, 0x{:02X}", pc, machine.cpu.memory.read_value_u8( pc + 1 ));
                    pc += 2;
                },
                0x11 => {
                    println!("0x{:02X}\tLD DE, 0x{:02X}{:X}", pc, machine.cpu.memory.read_value_u8( pc + 2 ), machine.cpu.memory.read_value_u8( pc + 1 ));
                    pc += 3;
                },
                0x17 => {
                    println!("0x{:02X}\tRLA", pc);
                    pc += 1;
                }
                0x12 => {
                    println!("0x{:02X}\tLD (DE), A", pc);
                    pc += 1;
                }
                0x13 => {
                    println!("0x{:02X}\tINC DE", pc);
                    pc += 1;
                },
                0x14 => {
                    println!("0x{:02X}\tINC D", pc);
                    pc += 1;
                },
                0x15 => {
                    println!("0x{:02X}\tDEC D", pc);
                    pc += 1;
                },
                0x16 => {
                    println!("0x{:02X}\tLD D, 0x{:02X}", pc, machine.cpu.memory.read_value_u8( pc + 1 ));
                    pc += 2;
                },
                0x18 => {
                    let value =  machine.cpu.memory.read_value_u8( pc + 1 );
                    println!("0x{:02X}\tJR 0x{:02X} 0x{:02X}", pc, (pc + value as usize), value);
                    pc += 2;
                },
                0x1A => {
                    println!("0x{:02X}\tLD A, (DE)", pc);
                    pc += 1;
                },
                0x1C => {
                    println!("0x{:02X}\tINC E", pc);
                    pc += 1;
                }
                0x1E => {
                    println!("0x{:02X}\tLD E, d8 0x{:02X}", pc, machine.cpu.memory.read_value_u8(pc + 1));
                    pc += 2;
                },
                0x20 => {
                    // calculate jmp location
                    let mut location = LittleEndian::read_int(&[machine.cpu.memory.read_value_u8( (pc + 1) as usize )], 1);
                    location  = (((pc + 2) as i32) + (((location as u8) as i8) as i32)) as i64;
                    println!("0x{:02X}\tJR NZ, 0x{:02X}", pc, location);
                    pc += 2;
                },
                0x21 => {
                    println!("0x{:02X}\tLD HL, 0x{:02X}{:X}", pc, machine.cpu.memory.read_value_u8( (pc + 2) as usize ), machine.cpu.memory.read_value_u8( (pc + 1) as usize ));
                    pc += 3;
                },
                0x22 => {
                    println!("0x{:02X}\tLD (HL+), A", pc);
                    pc += 1;
                },
                0x23 => {
                    println!("0x{:02X}\tINC HL", pc);
                    pc += 1;
                },
                0x24 => {
                    println!("0x{:02X}\tINC H", pc);
                    pc += 1;
                },
                0x28 => {
                    let mut location = LittleEndian::read_int(&[machine.cpu.memory.read_value_u8( (pc + 1) as usize )], 1);
                    location  = (((pc + 2) as i32) + (((location as u8) as i8) as i32)) as i64;
                    println!("0x{:02X}\tJR Z, r8 0x{:04X}", pc, location);
                    pc += 2;
                },
                0x2A => {
                    println!("0x{:02X}\tLD A, (HL+)", pc);
                    pc += 1;
                }
                0x31 => {
                    println!("0x{:02X}\tLD SP, 0x{:02X}{:X}", pc, machine.cpu.memory.read_value_u8( (pc + 2) as usize ), machine.cpu.memory.read_value_u8( (pc + 1) as usize ));
                    pc += 3;
                },
                0x32 => {
                    println!("0x{:02X}\tLD (HL-),A\t", pc);
                    pc += 1;
                },
                0x3E => {
                    
                    println!("0x{:02X}\tLD A, 0x{:02X}", pc, machine.cpu.memory.read_value_u8(( pc + 1) as usize ));
                    pc += 2;
                },
                0x4F => {
                    println!("0x{:02X}\tLD C, A", pc);
                    pc += 1;
                },
                0x47 => {
                    println!("0x{:02X}\tLD B, A", pc);
                    pc += 1;
                },
                0x57 => {
                    println!("0x{:02X}\tLD D, A", pc);
                    pc += 1;
                },
                0x66 => {
                    println!("0x{:02X}\tLD H, (HL)", pc);
                    pc += 1;
                },
                0x7C => {
                    println!("0x{:02X}\tLD A, H", pc);
                    pc += 1;
                }
                0x77 => {
                    println!("0x{:02X}\tLD (HL), A", pc);
                    pc += 1;
                },
                0x78 => {
                    println!("0x{:02X}\tLD A, B", pc);
                    pc += 1;
                },
                0x7B => {
                    println!("0x{:02X}\tLD A, E", pc);
                    pc += 1;
                },
                0x90 => {
                    println!("0x{:02X}\tSUB B", pc);
                    pc += 1;
                }
                0xAF => {
                    println!("0x{:02X}\tXOR A", pc);
                    pc += 1;
                },
                0xBE => {
                    println!("0x{:02X}\tCP (HL)", pc);
                    pc += 1;
                },
                0xC0 => {
                    println!("0x{:02X}\tRET NZ", pc);
                    pc += 1;
                },
                0xC1 => {
                    println!("0x{:02X}\tPOP BC", pc);
                    pc += 1;
                },
                0xC3 => {
                    let location = machine.cpu.read_word();
                    println!("0x{:02X}\tJP 0x{:02X}", pc, location);
                    pc += 2;
                },
                0xC5 =>  {
                    println!("0x{:02X}\tPUSH BC", pc);
                    pc += 1;
                },
                0xCB => { // redirect 
                    match machine.cpu.memory.read_value_u8( pc + 1 ) {
                        0x11 => println!("0x{:02X}\tRL C", pc + 1),
                        0x7C => println!("0x{:02X}\tBIT 7, H", pc + 1),
                        _ =>  println!("Not disassembled redirect 0x{:02X}", pc + 1),
                    };
                    pc += 2;
                },
                0xCC => {
                    println!("0x{:02X}\tCALL Z, 0x{:02X}{:X}", pc, machine.cpu.memory.read_value_u8( (pc + 2) as usize ), machine.cpu.memory.read_value_u8( (pc + 1) as usize ));
                    pc += 3;
                },
                0xCD => {
                    println!("0x{:02X}\tCALL 0x{:02X}{:X}", pc, machine.cpu.memory.read_value_u8( (pc + 2) as usize ), machine.cpu.memory.read_value_u8( (pc + 1) as usize ));
                    pc += 3;
                },
                0xCE => {
                    println!("0x{:02X}\tADC A, 0x{:02X}", pc, machine.cpu.memory.read_value_u8((pc + 1) as usize));
                    pc += 2;
                },
                0xC9 => {
                    println!("0x{:02X}\tRET", pc);
                    pc += 1;
                }
                0xE0 => {
                    println!("0x{:02X}\tLDH ($FF00+0x{:02X}), A", pc, machine.cpu.memory.read_value_u8( pc + 1 ));
                    pc += 2;
                },
                0xF0 => {
                    println!("0x{:02X}\tLDH A, ($FF00+0x{:02X})", pc, machine.cpu.memory.read_value_u8( pc + 1 ));
                    pc += 2;
                },
                0xE2 => {
                    println!("0x{:02X}\tLD ($FF00+C), A", pc);
                    pc += 1;
                },
                0xEA => {
                    println!("0x{:02X}\tLD (a16), A", pc);
                    pc += 3;
                },
                0xFE => {
                    println!("0x{:02X}\tCP d8, (0x{:02X})", pc, machine.cpu.memory.read_value_u8(pc + 1));
                    pc += 2;
                    
                },
                _ => println!("Not disassembled 0x{:02X}, 0x{:02X}", pc, machine.cpu.memory.read_value_u8( pc )) 
            }
            line_count += 1;
        }
    }

    pub fn parse_input(&mut self, input: &str, machine: &Gameboy) -> Actions {

        let split: Vec<&str> = input.split(" ").collect();
        match split[0].as_ref() {
            ":q" => return Actions::EXIT,
            "quit" => return Actions::EXIT,
            "s" => return Actions::STEP,
            "step" => return Actions::STEP,
            "p" => {
                self.print_status(&machine.cpu);
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
                self.disassemble(&machine);
                return Actions::NOOP
            },
            _ => return Actions::NOOP,
        }
    }
    
    pub fn step(&self, machine: &mut Gameboy) {
        let res = machine.step();
    }
    
    pub fn print_status(&self, cpu: &CPU) {
        println!("DEBUGGER PC: {}", cpu.pc);
        println!("{:?}", cpu);
    }

}
