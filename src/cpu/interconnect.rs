use cpu::cpu::CPU;
use cpu::mem::MMU;
use cpu::gpu::GPU;

pub struct Interconnect {
    pub mmu: MMU,
    pub gpu: GPU,
    
}

impl Interconnect {
    pub fn new() -> Self {
        Interconnect {
            mmu: MMU::new(),
            gpu: GPU::new(),
        }
    }

    pub fn read_word(&mut self) -> u16 {
        0
    }

    pub fn write_word(&mut self, value: u8, location: u16) {
        match location {

            0x0000 ... 0x00FF => {
                println!("Restart and Interrupt Vectors");
            },
            0x0100 ... 0x014F => {	println!("Cartridge Header Area") },
            0x0150 ... 0x3FFF => {	println!("Cartridge ROM  -  Bank 0 (fixed)") },
            0x4000 ... 0x7FFF => {	println!("Cartridge ROM  -  Switchable Banks 1 - xx") },
            0x8000 ... 0x97FF => {	println!("Character RAM") },
            0x9800 ... 0x9BFF => {	println!("BG Map Data 1") },
            0x9C00 ... 0x9FFF => {	println!("BG Map Data 2") },
            0xA000 ... 0xBFFF => {	println!("Cartridge RAM (If Available)") },
            0xC000 ... 0xCFFF => {	println!("Internal RAM  -  Bank 0 (fixed)") },
            0xD000 ... 0xDFFF => {	println!("Internal RAM  -  Bank 1 - 7 (switchable  -  CGB only)") },
            0xE000 ... 0xFDFF => {	println!("Echo RAM  -  Reserved, Do Not Use") },
            0xFE00 ... 0xFE9F => {	println!("OAM  - Object Attribute Memory") },
            0xFEA0 ... 0xFEFF => {	println!("Unusable Memory") },
            0xFF00 ... 0xFF7F => {
                if location == 0xFF01 {
                    println!("CONSOLE OUTPUT {}", value);
                }
                println!("Hardware I/O Registers")
                    
            },
            0xFF80 ... 0xFFFE => {	println!("Zero Page  127 bytes") },
            _ => { panic!("Trying to write to invalid memory location {}", location)}
        }
        // println!("\n\n\n\n\n\n\n\n\n\n\n\n");
    }
    
    pub fn step(&mut self) {
        println!("I'm a carrot!");
    }

}
