use std::io::prelude::*;
use std::fs::File;

use cpu::interconnect::Interconnect;
use cpu::cpu::CPU;

pub struct Gameboy {
    pub cpu: CPU,
    pub interconnect: Interconnect,
}

impl Gameboy {
    pub fn new(boot_rom: File) -> Self {
        Gameboy {
            cpu: CPU::new(boot_rom),
            interconnect: Interconnect::new(),
        }
    }

    pub fn step(&mut self) -> u8 {
        self.cpu.cycle(&mut self.interconnect)
    }

}
