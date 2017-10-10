use {DISPSIZE, NUM_KEYS};

pub trait Frontend {
    fn draw_graphics(&mut self, display: [bool; DISPSIZE]);
    fn beep(&self);
    fn check_keys(&mut self) -> Option<[bool; NUM_KEYS]>;
    fn get_key(&mut self) -> Option<u8>;
}