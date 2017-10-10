extern crate rand;
extern crate termion;
extern crate sdl2;

const DISPWIDTH: usize = 64;
const DISPHEIGHT: usize = 32;
const DISPSIZE: usize = DISPWIDTH * DISPHEIGHT;
const MEMSIZE: usize = 4096;
const STACKSIZE: usize = 16;
const NUM_KEYS: usize = 16;
const NUM_REGS: usize = 16;

mod cpu;
mod termion_frontend;
mod sdl2_frontend;
mod frontend;

pub use cpu::Chip8;
pub use termion_frontend::TermionFrontend;
pub use sdl2_frontend::SDL2Frontend;
pub use frontend::Frontend;