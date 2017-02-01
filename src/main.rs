#[macro_use]
extern crate bitflags;
extern crate byteorder;
extern crate sdl2;

use std::io::prelude::*;
use std::process::exit;
use std::fs::File;
use std::path::Path;
use std::str::FromStr;

mod cpu;
mod debug;

use cpu::cpu::CPU;
use debug::debug::Debug;
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
    let debug: bool = true;
    let run: bool = true;
    let mut cpu = CPU::new();
    let mut count = 0;
    for byte in boot.bytes() {
        cpu.boot[count] = byte.unwrap();
        count += 1;
    }

    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();

    let _window = video.window("Gb-rs", 800, 600)
        .position_centered()
        .build()
        .unwrap();
    let mut renderer = _window.renderer().build().unwrap();
    let mut debugger = Debug::new(&cpu);
    if debug {
        debugger.print_status();
    }
    // cpu.print_boot();
    
    let mut event_pump = sdl_context.event_pump().unwrap();
    loop {
        for event in event_pump.poll_event() {
            use sdl2::event::Event;
            use sdl2::keyboard::Keycode;

            match event {
                Event::KeyDown { keycode, .. } => match keycode {
                    Some(key) => {
                        match key {
                            Keycode::Q => exit(0),
                            Keycode::S => debugger.step(),
                            _ => ()
                        }
                    },
                    None => (),
                },
                _ => (),
            }
        }

        // if !debug {
        //     let cycles = cpu.cycle();
        //     cpu.updateTimers(cycles);
        //     cpu.renderScreen(cycles);
        //     cpu.interrupts(cycles);
        // }
    }

    
    println!("{:?}", cpu);
}
