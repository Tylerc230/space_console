use firmware_rust as fr;
use fr::input::Input;
use rotary_encoder_hal::{Direction, Rotary};
use embedded_hal::digital::v2::InputPin;
pub struct InputDriver<PinR1A: InputPin, PinR1B: InputPin, PinR2A: InputPin, PinR2B: InputPin> {
    rotary1: Rotary<PinR1A, PinR1B>,
    rotary2: Rotary<PinR2A, PinR2B>
    
}

impl<PinR1A: InputPin, PinR1B: InputPin, PinR2A: InputPin, PinR2B: InputPin> InputDriver<PinR1A, PinR1B, PinR2A, PinR2B> {
    pub fn new(left_rotary: Rotary<PinR1A, PinR1B>, right_rotary: Rotary<PinR2A, PinR2B>) -> InputDriver<PinR1A, PinR1B, PinR2A, PinR2B> {
        InputDriver { rotary1:  left_rotary, rotary2: right_rotary }
    }

    pub fn update(&mut self) -> Input {
        let dir1 = self.rotary1.update().unwrap_or(Direction::None);
        let dir2 = self.rotary2.update().unwrap_or(Direction::None);
        //let dir = self.rotary1.update().unwrap();
        Input{left_rotary_direction: from(dir1), right_rotary_direction: from(dir2) }
    }
}

fn from(dir: Direction) -> fr::input::KnobDirection {
    match dir {
        Direction::Clockwise => fr::input::KnobDirection::Clockwise,
        Direction::CounterClockwise => fr::input::KnobDirection::CounterClockwise,
        Direction::None => fr::input::KnobDirection::None
    }
}
