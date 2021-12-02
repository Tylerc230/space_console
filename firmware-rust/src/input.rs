use ufmt::derive::uDebug;
#[derive(uDebug, Copy, Clone)]
pub struct Input {
    pub left_rot_dir: KnobDirection,
    pub right_rot_dir: KnobDirection
}

impl Input {
    pub fn new() -> Input {
        Input { left_rot_dir: KnobDirection::None, right_rot_dir: KnobDirection::None}
    }
}

#[derive(uDebug, PartialEq, Copy, Clone)]
pub enum KnobDirection {
    Clockwise,
    CounterClockwise,
    None
}
