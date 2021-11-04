use smart_leds::RGB8;
pub trait Program {
    fn init(&self);
    fn update(&self, buffer: &mut [RGB8]);
}

