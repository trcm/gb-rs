extern crate bitflags;
extern crate byteorder;
extern crate sdl2;
extern crate clap;

use std::io::prelude::*;
use std::io::{stdin, stdout};
use std::process::exit;
use std::fs::File;
use std::str::FromStr;

mod cpu;
mod debug;

use clap::{Arg, App, SubCommand};

use cpu::cpu::CPU;
use cpu::interconnect::Interconnect;
use cpu::gb::Gameboy;
use debug::debug::{Debug, Actions};
// use mem::MMU;


fn main() {

    let matches = App::new("Gb-rs")
        .version("0.1")
        .about("Gameboy emu in Rust")
        .arg(Arg::with_name("rom")
             .short("r")
             .long("rom")
             .value_name("ROM")
             .help("Sets the location of the current rom")
             .takes_value(true))
        .arg(Arg::with_name("debug")
             .short("d")
             .long("debug")
             .help("Sets the debug value, if set it will start the debugger"))
        .get_matches();

    let boot_path = matches.value_of("rom").unwrap_or("./roms/BOOT.bin");

    let boot = match File::open(boot_path) {
        Ok(f) => f,
        Err(e) => panic!("Could not read boot file, {}", e)
    };

    //load boot rom
    let mut debug: bool = match matches.occurrences_of("debug") {
        0 => false,
        _ => true,
    };

    let mut machine = Gameboy::new(boot);
    // let mut cpu = CPU::new(boot);

    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();

    let _window = video.window("Gb-rs", 160, 144)
        .position_centered()
        .build()
        .unwrap();

    let mut debugger = Debug::new();
    if debug {
        debugger.print_status(&machine.cpu);
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
                            Keycode::S => debugger.step(&mut machine),
                            Keycode::P => debugger.print_status(&mut machine.cpu),
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
            if debugger.check_breakpoints(machine.cpu.pc) {
                debug = !debug;
            } else {
                machine.step();
                machine.cpu.update_timers();
                machine.cpu.interrupts();
            }
        } else {
            'debug: loop {
                print!("> ");
                let _ = stdout().flush();
                let mut input = String::new();
                stdin().read_line(&mut input).expect("Input invalid");
                match debugger.parse_input(input.trim(), &machine) {
                    Actions::BREAK => {
                        debug = !debug;
                        break;
                    },
                    Actions::EXIT => {
                        exit(0);
                    },
                    Actions::STEP => {
                        machine.step();
                    },
                    Actions::NOOP => (),
                };
            }
        }
    }
    
}

