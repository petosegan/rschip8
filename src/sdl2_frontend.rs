extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use frontend::Frontend;

use {DISPWIDTH, DISPHEIGHT, DISPSIZE, NUM_KEYS};

const SCALE: u32 = 10;

pub struct SDL2Frontend {
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
    event_pump: sdl2::EventPump,
}

impl SDL2Frontend {
    pub fn new() -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem.window("rschip8", DISPWIDTH as u32 * SCALE, DISPHEIGHT as u32 * SCALE)
            .position_centered()
            .opengl()
            .build()
            .unwrap();
        let mut canvas = window.into_canvas().build().unwrap();
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();
        let event_pump = sdl_context.event_pump().unwrap();

        SDL2Frontend {  canvas: canvas,
                        event_pump: event_pump }
    }
}

impl Frontend for SDL2Frontend {
    fn draw_graphics(&mut self, display: [bool; DISPSIZE]) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();
        self.canvas.set_draw_color(Color::RGB(0, 230, 20));
        for y in 0..DISPHEIGHT {
            for x in 0..DISPWIDTH {
                let this_index = y * DISPWIDTH + x;
                if display[this_index] {
                    self.canvas.fill_rect(Rect::new((x * SCALE as usize) as i32, (y * SCALE as usize) as i32, SCALE, SCALE)).unwrap();
                }
            }
        }
        self.canvas.present();
    }
    fn beep(&self) {
        unimplemented!();
    }
    fn check_keys(&mut self) -> Option<[bool; NUM_KEYS]> {
        let mut result = [false; NUM_KEYS];
        for event in self.event_pump.poll_iter() {
            match event {
                Event::KeyDown { keycode: Some(Keycode::Up), .. }    => { result[0x2] = true; },
                Event::KeyDown { keycode: Some(Keycode::Down), .. }  => { result[0x8] = true; },
                Event::KeyDown { keycode: Some(Keycode::Left), .. }  => { result[0x4] = true; },
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => { result[0x6] = true; },
                Event::KeyDown { keycode: Some(Keycode::Q), .. }     => { result[0x0] = true; },
                Event::KeyDown { keycode: Some(Keycode::W), .. }     => { result[0x1] = true; },
                Event::KeyDown { keycode: Some(Keycode::E), .. }     => { result[0x3] = true; },
                Event::KeyDown { keycode: Some(Keycode::R), .. }     => { result[0x5] = true; },
                Event::KeyDown { keycode: Some(Keycode::T), .. }     => { result[0x7] = true; },
                Event::KeyDown { keycode: Some(Keycode::Y), .. }     => { result[0x9] = true; },
                Event::KeyDown { keycode: Some(Keycode::A), .. }     => { result[0xA] = true; },
                Event::KeyDown { keycode: Some(Keycode::S), .. }     => { result[0xB] = true; },
                Event::KeyDown { keycode: Some(Keycode::D), .. }     => { result[0xC] = true; },
                Event::KeyDown { keycode: Some(Keycode::F), .. }     => { result[0xD] = true; },
                Event::KeyDown { keycode: Some(Keycode::G), .. }     => { result[0xE] = true; },
                Event::KeyDown { keycode: Some(Keycode::H), .. }     => { result[0xF] = true; },
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::X), .. } => {
                    return None;
                },
                _ => {}
            }
        }
        Some(result)
    }
    fn get_key(&mut self) -> Option<u8> {
        loop {
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::KeyDown { keycode: Some(Keycode::Up), .. }    => { return Some(0x2); },
                    Event::KeyDown { keycode: Some(Keycode::Down), .. }  => { return Some(0x8); },
                    Event::KeyDown { keycode: Some(Keycode::Left), .. }  => { return Some(0x4); },
                    Event::KeyDown { keycode: Some(Keycode::Right), .. } => { return Some(0x6); },
                    Event::KeyDown { keycode: Some(Keycode::Q), .. }     => { return Some(0x0); },
                    Event::KeyDown { keycode: Some(Keycode::W), .. }     => { return Some(0x1); },
                    Event::KeyDown { keycode: Some(Keycode::E), .. }     => { return Some(0x3); },
                    Event::KeyDown { keycode: Some(Keycode::R), .. }     => { return Some(0x5); },
                    Event::KeyDown { keycode: Some(Keycode::T), .. }     => { return Some(0x7); },
                    Event::KeyDown { keycode: Some(Keycode::Y), .. }     => { return Some(0x9); },
                    Event::KeyDown { keycode: Some(Keycode::A), .. }     => { return Some(0xA); },
                    Event::KeyDown { keycode: Some(Keycode::S), .. }     => { return Some(0xB); },
                    Event::KeyDown { keycode: Some(Keycode::D), .. }     => { return Some(0xC); },
                    Event::KeyDown { keycode: Some(Keycode::F), .. }     => { return Some(0xD); },
                    Event::KeyDown { keycode: Some(Keycode::G), .. }     => { return Some(0xE); },
                    Event::KeyDown { keycode: Some(Keycode::H), .. }     => { return Some(0xF); },
                    Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::X), .. } => {
                        return None;
                    },
                    _ => {}
                }
            }
        }
    }
}