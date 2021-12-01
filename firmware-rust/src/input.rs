use ufmt::derive::uDebug;
#[derive(uDebug)]
pub struct Input {
    pub left_rotary_direction: KnobDirection,
    pub right_rotary_direction: KnobDirection
}

impl Input {
    pub fn new() -> Input {
        Input { left_rotary_direction: KnobDirection::None, right_rotary_direction: KnobDirection::None}
    }
}

#[derive(uDebug, PartialEq)]
pub enum KnobDirection {
    Clockwise,
    CounterClockwise,
    None
}
