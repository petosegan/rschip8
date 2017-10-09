extern crate termion;

use std::io::{Read, Write, stdout, Stdout, Bytes};
use termion::async_stdin;
use termion::raw::IntoRawMode;

const DISPWIDTH: usize = 64;
const DISPHEIGHT: usize = 32;
const DISPSIZE: usize = DISPWIDTH * DISPHEIGHT;
const NUM_KEYS: usize = 16;

pub fn get_output_stream() -> termion::raw::RawTerminal<Stdout>{
    stdout().into_raw_mode().unwrap()
}

pub fn get_input_stream() -> Bytes<termion::AsyncReader>{
    async_stdin().bytes()
}

pub fn draw_graphics(stream: &mut Stdout,
                 display: [bool; DISPSIZE]) {
    write!(stream, "{}{}", termion::cursor::Goto(1, 1),
                           termion::cursor::Hide).unwrap();

    let mut header = String::new();
    for _ in 0..DISPWIDTH+2 { header.push_str("##"); }
    write!(stream, "{}\n\r", header).unwrap();

    for row in 0..DISPHEIGHT {
        let mut this_row = String::new();
        this_row.push_str("##");
        for col in 0..DISPWIDTH {
            if display[row * DISPWIDTH + col] {
                this_row.push_str("\u{2588}\u{2588}");
            } else {
                this_row.push_str("  ");
            }
        }
        this_row.push_str("##");
        write!(stream, "{}\n\r", this_row).unwrap();
    }

    let mut footer = String::new();
    for _ in 0..DISPWIDTH+2 { footer.push_str("##"); }
    write!(stream, "{}\n\r", footer).unwrap();
}

pub fn beep(_: &mut Stdout) {
    //panic!("bell not implemented in termion");
}

pub fn check_keys(stream: &mut Bytes<termion::AsyncReader>) -> Option<[bool; NUM_KEYS]> {
    let mut result = [false; 16];

    let next_ch = stream.next();
    if let Some(Ok(ch)) = next_ch {
        match ch {
            b'\x1B' => {
                if let Some(Ok(b'[')) = stream.next() {
                    if let Some(Ok(ch2)) = stream.next() {
                        match ch2 {
                            65 => { result[0x2] = true; },
                            68 => { result[0x4] = true; },
                            67 => { result[0x6] = true; },
                            66 => { result[0x8] = true; },
                            _ => {},
                        }
                    }
                }
            }
            b'q' => { result[0x0] = true; },
            b'w' => { result[0x1] = true; },
            b'e' => { result[0x3] = true; },
            b'r' => { result[0x5] = true; },
            b't' => { result[0x7] = true; },
            b'y' => { result[0x9] = true; },
            b'a' => { result[0xA] = true; },
            b's' => { result[0xB] = true; },
            b'd' => { result[0xC] = true; },
            b'f' => { result[0xD] = true; },
            b'g' => { result[0xE] = true; },
            b'h' => { result[0xF] = true; },
            b'x' => { return None; }
            _ => {},
        }
    }

   Some(result)
}

pub fn get_key(stream: &mut Bytes<termion::AsyncReader>) -> Option<u8> {
    // The nested match statements make the escape sequence
    // prefix for arrow keys optional. This is a kludge to
    // deal with loss of characters from asynchronous stdin.
    loop {
        let next_ch = stream.next();
        if let Some(Ok(ch)) = next_ch {
            match ch {
                b'\x1B' => {
                    if let Some(Ok(b'[')) = stream.next() {
                        if let Some(Ok(ch2)) = stream.next() {
                            match ch2 {
                                65 => { return Some(0x2);},
                                68 => { return Some(0x4);},
                                67 => { return Some(0x6);},
                                66 => { return Some(0x8);},
                                _ => {},
                            }
                        }
                    }
                }
                b'[' => {
                    if let Some(Ok(ch2)) = stream.next() {
                        match ch2 {
                            65 => { return Some(0x2);},
                            68 => { return Some(0x4);},
                            67 => { return Some(0x6);},
                            66 => { return Some(0x8);},
                            _ => {},
                        }
                    }
                }
                65 => { return Some(0x2);},
                68 => { return Some(0x4);},
                67 => { return Some(0x6);},
                66 => { return Some(0x8);},
                b'q' => { return Some(0x0); },
                b'w' => { return Some(0x1); },
                b'e' => { return Some(0x3); },
                b'r' => { return Some(0x5); },
                b't' => { return Some(0x7); },
                b'y' => { return Some(0x9); },
                b'a' => { return Some(0xA); },
                b's' => { return Some(0xB); },
                b'd' => { return Some(0xC); },
                b'f' => { return Some(0xD); },
                b'g' => { return Some(0xE); },
                b'h' => { return Some(0xF); },
                b'x' => { return None; }
                _ => {},
            }
        }
    }
}