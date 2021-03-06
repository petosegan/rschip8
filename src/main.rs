extern crate getopts;
extern crate rschip8;

use std::io::Write;
use std::env;
use std::{thread, time};
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use getopts::Options;
use rschip8::*;

fn print_usage(program: &str, opts: Options) {
    print!("{}", opts.usage(&brief(&program)));
}

fn brief<ProgramName>(program: ProgramName) -> String
        where ProgramName: std::fmt::Display {
    return format!("Usage: {} -g GAME [-c CLOCK_SPEED] [(-q|-v|--vv)]", program);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = getopts::Options::new();
    opts.reqopt("g", "", "path to game rom", "GAME");
    opts.optopt("c", "", "clock speed (Hz)", "CLOCK_SPEED");
    opts.optflag("t", "", "terminal display");
    opts.optflag("", "vv", "print opcodes and disable display");
    opts.optflag("h", "help", "print this help message");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => {let message = format!("{}\n{}\n",
                                  f.to_string(),
                                  opts.usage(&brief(&args[0])));
            if let Err(err) = write!(std::io::stderr(), "{}", message) {
                panic!("Failed to write to standard error: {}\n\
                       Error encountered while trying to log the \
                       following message: \"{}\"",
                       err,
                       message);
            }
            std::process::exit(1);
        }
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }
    
    let game_path = match matches.opt_str("g") {
        Some(s) => s,
        None => "./games/BRIX".to_string(),
    };

    let path = Path::new(&game_path);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", display,
                                                   why.description()),
        Ok(file) => file,
    };

    // Read the file contents into a Vec<u8>, returns `io::Result<usize>`
    let mut buffer = Vec::new();
    match file.read_to_end(&mut buffer) {
        Err(why) => panic!("couldn't read {}: {}", display,
                                                   why.description()),
        Ok(_) => {println!("loaded {}", display)},
    }

    let clock_speed = match matches.opt_str("c") {
        Some(s) => s.parse::<f64>().unwrap(),
        None => 500.0,
    };

    let mut chip8 = Chip8::new();
    chip8.trace_flag = matches.opt_present("vv");
    chip8.load(buffer);

    let clock_period_ns = (1.0 / clock_speed * 1_000_000_000.0).floor() as u32;
    let sleep_duration = time::Duration::new(0, clock_period_ns);

    if matches.opt_present("t") {
        run_game_loop(chip8, TermionFrontend::new(), sleep_duration);
    } else {
        run_game_loop(chip8, SDL2Frontend::new(), sleep_duration);
    }
}

fn run_game_loop<T>(mut chip8: Chip8, mut frontend: T, sleep_duration: std::time::Duration)
    where T: Frontend {
    loop {
        chip8.emulate_cycle();

        if chip8.wait_for_key_flag {
            if let Some(key) = frontend.get_key() {
                chip8.give_key(key);
                chip8.wait_for_key_flag = false;
            } else { break; }
        }

        if chip8.beep_flag {
            frontend.beep();
        }

        if chip8.draw_flag && !chip8.trace_flag {
            frontend.draw_graphics(chip8.display);
        }

        thread::sleep(sleep_duration);

        if let Some(keys_pressed) = frontend.check_keys() {
            chip8.set_keys(keys_pressed);
        } else { break; }
    }
}