extern crate bitflags;
extern crate byteorder;
extern crate sdl2;

use std::io::prelude::*;
use std::io::{stdin, stdout};
use std::process::exit;
use std::fs::File;
use std::str::FromStr;

mod cpu;
mod debug;

use cpu::cpu::CPU;
use debug::debug::{Debug, Actions};
// use mem::MMU;


fn main() {

    let boot_path: String = match std::env::args().nth(1) {
        Some(p) => p,
        None => String::from_str("./roms/BOOT.bin").unwrap()
    };

    let boot = match File::open(boot_path) {
        Ok(f) => f,
        Err(e) => panic!("Could not read boot file, {}", e)
    };

    //load boot rom
    let mut debug: bool = true;
    let mut cpu = CPU::new(boot);

    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();

    let _window = video.window("Gb-rs", 160, 144)
        .position_centered()
        .build()
        .unwrap();

    let mut debugger = Debug::new();
    if debug {
        debugger.print_status(&cpu);
    }
    // cpu.print_boot();
    
    let mut event_pump = sdl_context.event_pump().unwrap();
    loop {

        // check breakpoints

        for event in event_pump.poll_event() {
            use sdl2::event::Event;
            use sdl2::keyboard::Keycode;

            match event {
                Event::KeyDown { keycode, .. } => match keycode {
                    Some(key) => {
                        match key {
                            Keycode::Q => exit(0),
                            Keycode::S => debugger.step(&mut cpu),
                            Keycode::P => debugger.print_status(&mut cpu),
                            Keycode::C => debug = !debug,
                            _ => ()
                        }
                    },
                    None => (),
                },
                _ => (),
            }
        }
        
        
        if !debug {
            // check breakpoints
            if debugger.check_breakpoints(cpu.pc) {
                debug = !debug;
            } else {
                cpu.cycle();
                cpu.update_timers();
                cpu.interrupts();
            }
        } else {
            'debug: loop {
                print!("> ");
                let _ = stdout().flush();
                let mut input = String::new();
                stdin().read_line(&mut input).expect("Input invalid");
                match debugger.parse_input(input.trim(), &cpu) {
                    Actions::BREAK => {
                        debug = !debug;
                        break;
                    },
                    Actions::EXIT => {
                        exit(0);
                    },
                    Actions::STEP => {
                        cpu.cycle ();
                    },
                    Actions::NOOP => (),
                };
            }
        }
    }
    
}

