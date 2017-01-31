#[macro_use]
extern crate bitflags;
extern crate byteorder;

use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::str::FromStr;

mod cpu;

use cpu::cpu::CPU;
// use mem::MMU;

fn main() {

    let boot_path: String = match std::env::args().nth(1) {
        Some(p) => p,
        None => String::from_str("./roms/BOOT.bin").unwrap()
    };
    println!("boot parth {}", boot_path);
    let boot = match File::open(boot_path) {
        Ok(f) => f,
        Err(e) => panic!("Could not read boot file, {}", e)
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
    while(true) {
        cpu.cycle();
    }
    
    println!("{:?}", cpu);
}
