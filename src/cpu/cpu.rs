use byteorder::{ByteOrder, LittleEndian};
use std::io::prelude::*;
use std::fs::File;
use std::fmt;
use cpu::mem::MMU;
use cpu::gpu::GPU;
use cpu::op::Opcode;
use cpu::interconnect::Interconnect;

#[allow(non_snake_case)]
pub struct CPU {
    pub f: u8, // flags
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub Z: u8, // set if math operation is zzero or two values match during CP
    pub N: u8, // set if subtraction was performed in the last math instruction
    pub H: u8, // set if a carry occurs from the lower nibble
    pub C: u8, // set if if carry occued in the last math op or if registare A is smaller in CP instruction
    sp: u16,
    pub pc: u16,
    opcode: u8,
    pub memory: MMU,
    pub gpu: GPU,
    m_clock: u32,
    t_clock: u32,
    pub boot: [u8; 256], // boot rom
    ime: u8,
}

impl fmt::Debug for CPU {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "A: {:#X}\nB: {:#X}\nC: {:#X}\nD: {:#X}\nE: {:#X}\nH: {:#X}\nL: {:#X}\nSP: {:#X} PC: {:#X}\nFlags: Z {:#X} N {:#X} H {:#X} C {:#X}\nt_clock: {}\tm_clock: {}",
               self.a, self.b, self.c, self.d, self.e, self.h, self.l, self.sp, self.pc, self.Z, self.N, self.H, self.C, self.t_clock, self.m_clock)
    }
}

impl CPU {
    pub fn new(boot_rom: File) -> CPU {
        let mut cpu = CPU {
            a: 0x0,
            f: 0x0,
            b: 0x0,
            c: 0x0,
            d: 0x0,
            e: 0x0,
            h: 0x0,
            l: 0x0,
            Z: 0x0,
            N: 0x0,
            H: 0x0,
            C: 0x0,
            sp: 0,
            pc: 0,
            m_clock: 0,
            t_clock: 0,
            opcode: 0x0,
            memory: MMU::new(),
            gpu: GPU::new(),
            boot: [0; 256],
            ime: 0,
        };

        let mut count = 0;
        for byte in boot_rom.bytes() {
            cpu.memory.load_value_u8(count, byte.unwrap());
            count += 1;
        }

        cpu
    }

    pub fn cycle(&mut self, interconnect: &mut Interconnect) -> u8 {
        self.get_opcode();
        self.parse_opcode(interconnect);
        return 0;
    }
    
    pub fn get_opcode(&mut self) {
        // println!("Mem loc: {:#X}", self.pc);
        // self.opcode = self.boot[self.pc as usize];
        self.opcode = self.memory.read_value_u8(self.pc as usize);
        // println!("Opcode {:02X}", self.opcode);
    }

    pub fn read_word(&self) -> u16 {
        let aa: u8 = self.memory.read_value_u8((self.pc + 2) as usize);
        let bb: u8 = self.memory.read_value_u8((self.pc + 1) as usize);
        let word: u16 = (aa as u16) << 8 | bb as u16;
        word
    }

    pub fn read_byte(&self, count: u16) -> u8 {
        self.memory.read_value_u8( (self.pc + count) as usize )
    }

    fn half_carry_add(&self, initial: u8, value: u8) -> bool {
        let a = initial & 0xF;
        let b = value & 0xF;
        return (a + b) & 0x10 == 0x10;
    }

    fn half_carry_sub(&self, initial: u8, value: u8) -> bool {
        let a = initial & 0xF;
        let b = value & 0xF;
        return (a.wrapping_sub(b)) & 0x10 == 0x10;
    }
    
    fn step(&mut self) { // move the timers forward
        // update the m clock one cycle
        // update t clock 4 cycles

        self.m_clock += 1;

        // for _ in 0..4 { // do something more here
        //     self.t_clock += 1;
        // }
        
        // update timers

        // update display
        self.render_screen();
    }
    
    pub fn update_timers(&mut self) {
    }

    // 0xFF40 - LCD Control Register
    // Bit 7 - LCD Power (0=Off, 1=On)
    // Bit 6 - Window Tile Map (0=9800h-9BFFh, 1=9C00h-9FFFh)
    // Bit 5 - Window Enable (0=Disabled, 1=Enabled)
    // Bit 4 - BG & Window Tileset (0=8800h-97FFh, 1=8000h-8FFFh)
    // Bit 3 - BG Tile Map (0=9800h-9BFFh, 1=9C00h-9FFFh)
    // Bit 2 - Sprite Size (0=8×8, 1=8×16)
    // Bit 1 - Sprites Enabled (0=Disabled, 1=Enabled)
    // Bit 0 - BG Enabled (in DMG) (0=Disabled, 1=Enabled)
    // 0xFF41 - LCD Status 
    // Bit 6 - LYC Check
    // Bit 5 - Mode 2 OAM Checj
    // Bit 4 - Mode 1 V Blank check 
    // Bit 3 - Mode 0 H Blank Check
    // Bit 2 - LYC Comp signal
    // Bit 1/0 - Screen mode
    // 0: H blank
    // 1: V blank
    // 2: Searching OAM
    // 3: Transfer data to lcd
    pub fn render_screen(&self) {
        // check 0xFF40
        if (self.memory.read_value_u8(0x0FF40) & 0b1000000) != 0 {
            println!("SCREEN ON!!!!!");
            panic!("SCREEN ON");
        }
    }
    pub fn interrupts(&mut self) {
        if self.ime != 0 {
            println!("INTERRUPT TRIGGERED!!!!!!");
        }
        // println!("interrups");
    }
    
    fn parse_opcode(&mut self, interconnect: &mut Interconnect) {
        match Opcode::parse(self.pc, self.opcode) {
        // match self.opcode {
            Opcode::NoOp => {
                self.pc += 1;
            }
            Opcode::IncB => {
                inc_reg!(self; b);
            },
            Opcode::DecA => {
                dec_reg!(self; a);
            }
            Opcode::DecB => {
                dec_reg!(self; b);
            },
            Opcode::DecC => {
                dec_reg!(self; c);
            },
            Opcode::IncD => {
                inc_reg!(self; d);
            },
            Opcode::DecD => {
                dec_reg!(self; d);
            },
            Opcode::DecE => {
                dec_reg!(self; e);
            },
            Opcode::IncH => {
                inc_reg!(self; h);
            },
            Opcode::LdB => {
            // 0x06 => { // LD b, d8
                let value = self.memory.read_value_u8((self.pc + 1) as usize);
                store_reg!(self; b; value);
                // self.b = value;
                self.pc += 2;
                self.step();
                self.step();
            },
            Opcode::LdD => { // 0x16 LD D, d8
                let value = self.memory.read_value_u8((self.pc + 1) as usize);
                store_reg!(self; d; value);
                self.pc += 2;
                self.step();
                self.step();
            }
            Opcode::IncC => { // INC C
                inc_reg!(self; c);
            },
            Opcode::IncDE => { // INC DE
                let value = inc_reg16!(self; d, e);
                self.d = (value >> 8) as u8;
                self.e = (value & 0xFF) as u8;
                self.pc += 1;
                self.step();
                self.step();
            },
            Opcode::LdDE => {
                // println!("LD DE, d16");
                self.d = self.read_byte(2);
                self.e = self.read_byte(1);
                self.pc += 3;
            },
            Opcode::RLA => { // RLA rotate A left through carry
                let old_value = self.a;
                self.a = self.rotate_left_carry(&old_value);
                self.Z = 0;
            },
            Opcode::LdADE => { // LD A, (DE)
                // println!("LD A, (DE)");
                let location = (self.d as u16) << 8 | self.e as u16;
                // self.a = self.memory.read_value_u8( location as usize );
                // println!("\n\n\n\n\n {:#X}", location);
                self.a = self.memory.read_value_u8( location as usize );
                self.pc += 1;
                // self.a = ;
            },
            Opcode::JRNZ => { // JR NZ 
                let mut location = LittleEndian::read_int(&[self.memory.read_value_u8( (self.pc + 1) as usize )], 1);
                if self.Z == 1 {
                    location  = (((self.pc + 2) as i32) + (((location as u8) as i8) as i32)) as i64;
                    self.pc = location as u16;
                    self.step();
                } else {
                    self.pc += 2;
                    self.step();
                    self.step();
                }
            },
					  Opcode::JRZ => {
                // println!("{:#X}", self.pc );
                // panic!();
                let mut location = LittleEndian::read_int(&[self.memory.read_value_u8( (self.pc + 1) as usize )], 1);
                if self.Z == 0 {

                    location  = (((self.pc + 2) as i32) + (((location as u8) as i8) as i32)) as i64;
                    self.pc = location as u16;
                    self.step();
                } else {
                    self.pc += 2;
                    self.step();
                    self.step();
                }
            },
            Opcode::Jr => {
                let mut location = LittleEndian::read_int(&[self.memory.read_value_u8( (self.pc + 1) as usize )], 1);
                location  = (((self.pc + 2) as i32) + (((location as u8) as i8) as i32)) as i64;
                self.pc = location as u16;
                
                self.step();
                self.step();
                self.step();
            },
            Opcode::LdHL => { // LD HL, $aabb
                store_reg16!(self; h, l; self.read_word());
                // self.h = self.read_byte(2);
                // self.l = self.read_byte(1);
                // println!("H {:X} L {:X}", self.h, self.l);
                self.pc += 3;
                self.step();
                self.step();
                self.step();
            },
            Opcode::LdSP => { // LD SP, $aabb
                let val: u16 = self.read_word();
                self.sp = val;
                self.pc += 3;
                self.step();
                self.step();
                self.step();
            },
            Opcode::LdHLDECA => { // LD (HL-), A
                // load the value in A into memory location HL, then decremement HL
                // construct memory loation from HL
                // println!("H {:X}, L {:X}", self.h, self.l);

                let location = (self.h as u16) << 8 | (self.l as u16);
                // println!("loocation {:X}", location);
                self.memory.load_value_u8(location as usize, self.a);
                interconnect.write_word(location as u8, location);
                // decrement HL
                let hl = location.wrapping_sub(1);
                // println!("HL {:X} decrement {:X}", HL, HL - 1);
                self.h = (hl >> 8) as u8;
                self.l = (hl & 0xFF) as u8;
                self.pc += 1;
                self.step();
                self.step();
            },
            Opcode::LdHLADDA => {

                let location = (self.h as u16) << 8 | (self.l as u16);
                // println!("loocation {:X}", location);
                self.memory.load_value_u8(location as usize, self.a);
                interconnect.write_word(self.a, location as u16);

                // decrement HL
                let hl = location.wrapping_add(1);
                // println!("HL {:X} decrement {:X}", HL, HL - 1);
                self.h = (hl >> 8) as u8;
                self.l = (hl & 0xFF) as u8;
                self.pc += 1;
                self.step();
                self.step();

            },
            Opcode::IncHL => {
                let location = (self.h as u16) << 8 | (self.l as u16);
                let hl = location.wrapping_add(1);
                self.h = (hl >> 8) as u8;
                self.l = (hl & 0xFF) as u8;
                self.pc += 1;
                self.step();
                self.step();
            },
            Opcode::IncBC => {
                let mut bc = get_reg16!(self; b, c);
                bc = bc.wrapping_add(1);
                self.b = (bc >> 8) as u8;
                self.c = (bc & 0xFF) as u8;
                self.pc += 1;
                self.step();
                self.step();
            },
            Opcode::DecBC => {
                let mut bc = get_reg16!(self; b, c);
                bc = bc.wrapping_sub(1);
                self.b = (bc >> 8) as u8;
                self.c = (bc & 0xFF) as u8;
                self.pc += 1;
                self.step();
                self.step();
            },
            Opcode::LdA => { // ld a d8
                let val = self.memory.read_value_u8((self.pc + 1) as usize);
                self.a = val;
                self.pc += 2;
            }
            Opcode::LdAE => {
                load_from_reg!(self; a, e);
                self.step();
                self.pc += 1;
            },
            Opcode::LdAH => {
                load_from_reg!(self; a, h);
                self.step();
                self.pc += 1;
            }
            Opcode::XORA => { // XOR A set Z
                println!("XOR A");
                self.a ^= self.a;
                self.f = 0b10000000;
                if self.a == 0 {
                    self.Z = 1;
                }
                self.pc += 1;
                self.step();
            },
            Opcode::CB => {
                self.pc += 1;
                self.redirect();
            },
            Opcode::LdCA => { // LD C, A
                self.c = self.a;
                self.pc += 1;
            }, 
            Opcode::PopBC => { // POP BC

                self.b = self.memory.read_value_u8((self.sp + 1) as usize);
                self.c = self.memory.read_value_u8(self.sp as usize);
                self.sp += 2;
                self.pc += 1;
                // println!("sp 0x{:02X}", self.sp);
                self.step();
                self.step();
                self.step();
            },
            Opcode::PushBC => { // PUSH BC push b then c onto the stack
                self.step();
                self.sp -= 2;
                self.memory.load_value_u8((self.sp + 1) as usize, self.b);
                interconnect.write_word(self.b, (self.sp + 1) as u16);
                self.memory.load_value_u8(self.sp as usize, self.c);
                interconnect.write_word(self.c, (self.sp) as u16);
                self.pc += 1;
                // println!("sp 0x{:02X}", self.sp);
            },
            Opcode::CallNN => { // call nn
                self.sp -= 2; // decrememnt stack pointer
                self.memory.load_value_u8(self.sp as usize, (self.pc + 3) as u8); // put the next location on the stack
                let location: u16 = (self.memory.read_value_u8( (self.pc + 2) as usize ) as u16) << 8 | (self.memory.read_value_u8( (self.pc + 1) as usize ) as u16);
                self.pc = location;
                self.step();
                self.step();
                self.step();
            },
            Opcode::RET => { // RET pop two bytes from the stack
                let first = self.memory.read_value_u8(self.sp as usize);
                let second = self.memory.read_value_u8((self.sp + 1) as usize);
                let location = (second as u16) << 8 | first as u16;
                self.pc = first as u16;
                self.sp = self.sp.wrapping_add(2);
                self.step();
                self.step();
                self.step();
                self.step();
            },
            Opcode::RetNZ => {
                if self.C == 0 {
                    let first = self.memory.read_value_u8(self.sp as usize);
                    let second = self.memory.read_value_u8((self.sp + 1) as usize);
                    let location = (second as u16) << 8 | first as u16;
                    self.pc = first as u16;
                } else {

                }
            },
            Opcode::LdC => { // LD C, d8
                let val: u8 = self.read_byte(1);
                self.c = val;
                self.pc += 2;
            },
            Opcode::LdhA => { // LDH ($FF00 + n), A - load A into 0xFF00 + d8
                let val: u8 = self.read_byte(1);
                self.memory.load_value_u8((0xFF00 | val as u16) as usize, self.a);
                self.pc += 2;
            },
            Opcode::LdHAA8 => {
                let val: u8 = self.read_byte(1);
                self.a = self.memory.read_value_u8((0xFF00 | val as u16) as usize);
                self.pc += 2;
            },
            Opcode::LdCADDA => { // LD (0xFF00+C), A load A into location 0xFF00 + self.c
                let location = 0xFF00 + self.c as u16;
                self.memory.load_value_u8(location as usize, self.c);
                self.pc += 1;
            },
            Opcode::LdHLA => { // LD (HL), A - load A into the address at HL 
                let location = (self.h as u16) << 8 | (self.l as u16);
                self.memory.load_value_u8(location as usize, self.a);
                self.pc += 1;
                self.step();
                self.step();
            },
            Opcode::CpD8 => {
                let imm = self.memory.read_value_u8((self.pc + 1) as usize);
                let value = self.a.wrapping_sub(imm);

                if (value & 0xFF) == 0 {
                    self.Z = 1;
                }
                if self.half_carry_sub(value, imm) {
                    self.C = 1;
                }

                self.N = 1;
                self.step();
                self.step();
                self.pc += 2;
            },
            Opcode::CpHL => {
                let address = get_reg16!(self; h, l);
                let imm = self.memory.read_value_u8(address as usize);
                let value = self.a;

                if value == imm {
                    self.Z = 1;
                }

                if self.half_carry_sub(self.a, imm) {
                    self.H = 1;
                }

                if self.a < imm {
                   self.C = 1; 
                }
                
                self.N =1;
                self.step();
                self.step();
                self.pc += 1;
            },
            Opcode::Lda16A => { // LD (a16), A
                let location = self.read_word();
                self.memory.load_value_u8(location as usize, self.a);
                self.pc += 3;

                self.step();
                self.step();
                self.step();
                self.step();
            },
            Opcode::LdHA => {
                self.h = self.a;
                self.step();
                self.pc += 1;
            },
            Opcode::LdDA => {
                self.d = self.a;
                self.step();
                self.pc += 1;
            },
            Opcode::LdBA => {
                self.b = self.a;
                self.step();
                self.pc += 1;
            }
            Opcode::LdE => {
                let val = self.memory.read_value_u8((self.pc + 1) as usize);
                self.e = val;
                self.step();
                self.step();
                self.pc += 2;
            },
            Opcode::SubB => {
                if self.half_carry_sub(self.a, self.b) {
                    self.H = 1;
                }
                self.a = self.a.wrapping_sub(self.b);
                if self.a == 0 {
                    self.Z = 1;
                }
                if self.a < 0 {
                    self.N = 1;
                }
                // TODO - CHECK FOR CARRY 
                if self.b > self.a {
                    self.C = 1;
                }
                self.step();
                self.pc += 1;
            },
            Opcode::Jp => {
                let location = self.read_word();
                self.step();
                self.step();
                self.step();
                self.pc = location;
            },
            Opcode::LdAHLADD => {
                let mut hl = (self.h as u16) << 8 | (self.l as u16);
                let value = self.memory.read_value_u8(hl as usize);
                
                self.a = value;

                hl = hl.wrapping_add(1);

                self.h = (hl >> 8) as u8;
                self.l = (hl & 0xFF) as u8;
                self.pc += 1;
                self.step();
                self.step();
                
            },
            Opcode::LdDEA => { // LD (DE), A
                let address = get_reg16!(self; d, e);
                self.memory.load_value_u8(address as usize, self.a);
                self.step();
                self.step();
                self.pc += 1;
            },
            Opcode::IncE => {
                inc_reg!(self; e);
            },
            Opcode::LdAB => {
                self.a = self.b;
                self.step();
                self.pc += 1;
            },
            Opcode::LdBCA => {
                let address = get_reg16!(self; b, c);
                self.memory.load_value_u8(address as usize, self.a);
                self.step();
                self.step();
                self.pc += 1;
            },
            Opcode::AdcA => {
                let value = self.memory.read_value_u8((self.pc + 1) as usize);
                let addition = value + self.C;
                self.N = 0;

                if self.half_carry_add(value, addition) {
                    self.H = 1;
                }

                if (value as u16).wrapping_add(addition as u16) > 0xFF {
                    self.C = 1;
                }
                
                self.a = value + addition;
                if self.a == 0 {
                    self.Z = 1;
                }
                self.pc += 2;
            },
            Opcode::LdHHL => {
                let address = get_reg16!(self; h, l) as usize;
                self.h = self.memory.read_value_u8(address);
                self.step();
                self.step();
                self.pc += 1;
            },
            Opcode::CallZ => {
                panic!("Call Z a16")
            },
            _ => {
                println!("Not implemented. Opcode: 0x{:02X} Mem: 0x{:02X}", self.opcode, self.pc);
                panic!();
            }
        }
    }

    fn redirect(&mut self) {
        self.get_opcode();
        match Opcode::redirect(self.opcode){
            Opcode::CBRLC => { // RL C
                let old_value = self.c;
                self.c = self.rotate_left_carry(&old_value);
                // println!("c {}", self.c);
                // self.C = (self.c >> 7) & 0x01;  // check if there was a carry
                // self.c = (self.c << 1) | self.C;
                // self.Z = if self.c == 0 {
                //     1
                // } else {
                //     0
                // };
                // self.step();
                // self.step();

                // self.N = 0;
                // self.H = 0;
                // self.pc += 1;
            },
            Opcode::CB7C => { // BIT 7, H, 2 8, Z 0 1
                let comp = 0b10000000 & self.h;
                if comp == 0 {
                    self.Z = 0;
                } else {
                    self.Z = 1;
                }
                self.pc += 1;
            },
            _ => panic!("Not Implemented Redirect. Opcode: {:X}", self.opcode)
        }
    }

    fn rotate_left_carry(&mut self, reg: &u8) -> u8 {
        let reg_value = *reg;
        self.C = (reg_value >> 7) & 0x01;  // save old bit 7 data
        let r = (reg_value << 1) | self.C;

        self.Z = if r == 0 {
            1
        } else {
            0
        };

        self.step();
        self.step();

        self.N = 0;
        self.H = 0;
        self.pc += 1;
        r
    }
    
}
