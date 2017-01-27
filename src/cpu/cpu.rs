use std::fmt;
use cpu::mem::MMU;
const BOOT_SIZE: u16 = 256;

pub struct CPU {
    f: u8, // flags
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    Z: u8, // set if math operation is zzero or two values match during CP
    N: u8, // set if subtraction was performed in the last math instruction
    H: u8, // set if a carry occurs from the lower nibble
    C: u8, // set if if carry occued in the last math op or if registare A is smaller in CP instruction
    sp: u16,
    pc: u16,
    opcode: u8,
    word: u16,
    byte: u8,
    memory: MMU,
    pub boot: [u8; 256], // boot rom
}

impl fmt::Debug for CPU {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "A: {:X}\nB: {:X}\nC: {:X}\nH: {:X}\nL: {:X}\nSP: {:X}\nPC: {:X}", self.a, self.b, self.c, self.h, self.l, self.sp, self.pc)
    }
}

impl CPU {
    pub fn new() -> CPU {
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
            opcode: 0x0,
            word: 0x0,
            byte: 0x0,
            memory: MMU::new(),
            boot: [0; 256],
        };
        cpu
    }

    pub fn cycle(&mut self) {
        println!("Cycle");
        println!("{:X}", self.opcode);
        self.get_opcode();
        self.parse_opcode();
    }
    
    pub fn print_boot(&self) {
        println!("Boot Rom");
        for i in 0..BOOT_SIZE {
            println!("{:X}", self.boot[i as usize]);
        }
    }

    pub fn get_opcode(&mut self) {
        self.opcode = self.boot[self.pc as usize];
    }
    
    pub fn read_word(&self) -> u16 {
        let aa: u8 = self.boot[(self.pc + 2) as usize];
        let bb: u8 = self.boot[(self.pc + 1) as usize];
        let word: u16 = (aa as u16) << 8 | bb as u16;
        word
    }

    pub fn read_byte(&self, count: u16) -> u8 {
        self.boot[(self.pc + count) as usize]
    }

    fn parse_opcode(&mut self) {
        match self.opcode {
            0xaf => { // XOR A set Z
                self.a ^= self.a;
                self.f = 0b10000000;
                self.pc += 1;

            }
            0x21 => { // LD HL, $aabb
                self.h = self.read_byte(2);
                self.l = self.read_byte(1);
                self.pc += 3;
            },
            0x31 => { // LD SP, $aabb
                let val: u16 = self.read_word();
                self.sp = val;
                self.pc += 3;
            },
            0x32 => { // LD (HL-), A
                // load the value in A into memory location HL, then decremement HL

                // construct memory loation from HL
                println!("H {:X}, L {:X}", self.h, self.l);

                let mut location = (self.h as u16) << 8 | (self.l as u16);
                println!("loocation {:X}", location);
                self.memory.load_value_u8(location as usize, self.a);
                
                // location = location - 1;
                // let newH = location >> 4;
                
                self.pc += 1;

            }
            _ => panic!("Not implemented. Opcode: {:X}", self.opcode)
        }
        
    }

    
}
