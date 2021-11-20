use ufmt::derive::uDebug;
use ufmt::uDebug;
#[derive(uDebug)]
pub struct Input {
    pub left_rotary_direction: KnobDirection,
    pub right_rotary_direction: KnobDirection
}

#[derive(uDebug, PartialEq)]
pub enum KnobDirection {
    Clockwise,
    CounterClockwise,
    None
}
