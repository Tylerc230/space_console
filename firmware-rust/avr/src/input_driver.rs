use firmware_rust as fr;
use fr::input::Input;
use rotary_encoder_hal::{Direction, Rotary};

use arduino_hal::hal::port::PB3;
use arduino_hal::hal::port::PB2;
use arduino_hal::hal::port::PB4;
use arduino_hal::hal::port::PC3;
use arduino_hal::port::{mode, Pin};
type InputPin<P> = Pin<mode::Input<mode::PullUp>, P>;

type LeftRotary = Rotary<InputPin<PB3>, InputPin<PB2>>;
type RightRotary = Rotary<InputPin<PB4>, InputPin<PC3>>;

pub struct InputDriver {
    rotary1: LeftRotary,
    rotary2: RightRotary,
    pub cur_input: Input
    
}

impl InputDriver {
    pub fn new(left_rotary: LeftRotary, right_rotary: RightRotary) -> InputDriver {
        InputDriver { rotary1:  left_rotary, rotary2: right_rotary, cur_input: Input::new() }
    }

    pub fn update(&mut self) {
        let dir1 = self.rotary1.update().unwrap_or(Direction::None);
        let dir2 = self.rotary2.update().unwrap_or(Direction::None);
        //let dir = self.rotary1.update().unwrap();
        self.cur_input = Input{left_rotary_direction: from(dir1), right_rotary_direction: from(dir2) }
    }
}

fn from(dir: Direction) -> fr::input::KnobDirection {
    match dir {
        Direction::Clockwise => fr::input::KnobDirection::Clockwise,
        Direction::CounterClockwise => fr::input::KnobDirection::CounterClockwise,
        Direction::None => fr::input::KnobDirection::None
    }
}
