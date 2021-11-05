use smart_leds::RGB8;
const ROWS: usize = 2;
const COLUMNS: usize = 2;
#[derive(PartialEq, Eq, Debug)]
pub struct PixelBuffer {
    pixels: [[RGB8; COLUMNS]; ROWS]
}

impl PixelBuffer {
    pub fn new(fill_color: RGB8) -> PixelBuffer {
        PixelBuffer { pixels: [[fill_color; COLUMNS]; ROWS]}
    }
     
    pub fn fill(&mut self, color: RGB8) {
        for row in &mut self.pixels {
            for led in row {
                *led = color;
            }
        }
    }
}
