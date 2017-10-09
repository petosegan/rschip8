extern crate rand;
extern crate termion;

mod cpu;
mod termion_frontend;

pub use cpu::Chip8;
pub use termion_frontend::TermionFrontend;