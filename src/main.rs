#[macro_use]
extern crate bitflags;
use std::io::prelude::*;
use std::fs::File;

mod cpu;

use cpu::cpu::CPU;
// use mem::MMU;

fn main() {
    
    
    let boot = match File::open("../roms/BOOT.bin") {
        Ok(f) => f,
        Err(e) => panic!("Could not read boot file")
    };
    println!("Hello, world!");

    //load boot rom
    
    let mut cpu = CPU::new();
    let mut count = 0;
    for byte in boot.bytes() {
        cpu.boot[count] = byte.unwrap();
        count += 1;
    }
    // cpu.print_boot();
    cpu.cycle();
    cpu.cycle();
    cpu.cycle();
    cpu.cycle();
    
    println!("{:?}", cpu);
}
