const DISPWIDTH: usize = 64;
const DISPHEIGHT: usize = 32;
const DISPSIZE: usize = DISPWIDTH * DISPHEIGHT;
const NUM_KEYS: usize = 16;

pub trait Frontend {
    fn draw_graphics(&mut self, display: [bool; DISPSIZE]);
    fn beep(&self);
    fn check_keys(&mut self) -> Option<[bool; NUM_KEYS]>;
    fn get_key(&mut self) -> Option<u8>;
}