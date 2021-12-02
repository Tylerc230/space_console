use firmware_rust as fr;
use fr::input::Input;
use rotary_encoder_hal::{Direction, Rotary};

use arduino_hal::hal::port::PB3;
use arduino_hal::hal::port::PB2;
use arduino_hal::hal::port::PB4;
use arduino_hal::hal::port::PC3;
use arduino_hal::port::{mode, Pin};
type InputPin<P> = Pin<mode::Input<mode::PullUp>, P>;

type LeftRotary = Rotary<InputPin<PB4>, InputPin<PC3>>;
type RightRotary = Rotary<InputPin<PB3>, InputPin<PB2>>;

pub struct InputDriver {
    left_rot: LeftRotary,
    right_rot: RightRotary,
    pub cur_input: Input
    
}

impl InputDriver {
    pub fn new(left_rotary: LeftRotary, right_rotary: RightRotary) -> InputDriver {
        InputDriver { left_rot:  left_rotary, right_rot: right_rotary, cur_input: Input::new() }
    }

    pub fn update(&mut self) {
        let dir1 = self.left_rot.update().unwrap_or(Direction::None);
        let dir2 = self.right_rot.update().unwrap_or(Direction::None);
        //let dir = self.rotary1.update().unwrap();
        self.cur_input = Input{left_rot_dir: from(dir1), right_rot_dir: from(dir2) }
    }

    pub fn pin_values(&mut self) -> (bool, bool) {
        (self.right_rot.pin_a().is_high(), self.right_rot.pin_b().is_high())
    }
}

fn from(dir: Direction) -> fr::input::KnobDirection {
    match dir {
        Direction::Clockwise => fr::input::KnobDirection::Clockwise,
        Direction::CounterClockwise => fr::input::KnobDirection::CounterClockwise,
        Direction::None => fr::input::KnobDirection::None
    }
}
