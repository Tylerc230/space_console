#![no_std]
#![no_main]

use panic_halt as _;
use firmware_rust as fr;
mod serial;
mod led_driver;
mod hal_timer;

use void::ResultVoidExt;
use rotary_encoder_hal::{Direction, Rotary};
use led_driver::{
    LEDStrip,
    Screen
};
use embedded_hal::digital::v2::InputPin;
use embedded_hal::digital::v2::OutputPin;
use fr::input::Input;
use firmware_rust::ufmt;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins  = arduino_hal::pins!(dp);

    let serial = arduino_hal::default_serial!(dp, pins, 57600);
    serial::init(serial);
    serial_println!("Hello from Arduino!\r").void_unwrap();

    let mut simple = fr::programs::simple_program::SimpleProgram::new();
    let mut runner = fr::ProgramRunner::new(&mut simple);

    let mut led_strip0 = LEDStrip::new(pins.d4.into_output());
    let mut led_strip1 = LEDStrip::new(pins.d2.into_output());//not working
    let mut led_strip2 = LEDStrip::new(pins.d3.into_output());
    let mut led_strip3 = LEDStrip::new(pins.d5.into_output());
    let mut led_strip4 = LEDStrip::new(pins.d6.into_output());
    let mut screen = Screen {led_strips: [
        &mut led_strip0,
        &mut led_strip1,
        &mut led_strip2,
        &mut led_strip3,
        &mut led_strip4,
    ] };
    //prog select (outer) switch (a1)
    let mut left_rotary = Rotary::new(pins.d11.into_pull_up_input(), pins.d10.into_pull_up_input());
    //let switch = pins.a1.into_pull_up_input();

    //mod select (inner) switch (a2)
    let right_rotary = Rotary::new(pins.d12.into_pull_up_input(), pins.a3.into_pull_up_input());
    //let switch = pins.a2.into_pull_up_input();
    let mut input = InputDriver::new(left_rotary, right_rotary);
    loop {
        let input_values = input.update();
        if input_values.left_rotary_direction != fr::input::KnobDirection::None || input_values.right_rotary_direction != fr::input::KnobDirection::None {
            serial_println!("Input {:#?}\r", input_values).void_unwrap();
        }
        runner.update();
        screen.write_buffer(&runner.pixel_buffer);
        //arduino_hal::delay_ms(1000 as u16);
    }
}

struct InputDriver<PinR1A: InputPin, PinR1B: InputPin, PinR2A: InputPin, PinR2B: InputPin> {
    rotary1: Rotary<PinR1A, PinR1B>,
    rotary2: Rotary<PinR2A, PinR2B>
    
}

impl<PinR1A: InputPin, PinR1B: InputPin, PinR2A: InputPin, PinR2B: InputPin> InputDriver<PinR1A, PinR1B, PinR2A, PinR2B> {
    fn new(left_rotary: Rotary<PinR1A, PinR1B>, right_rotary: Rotary<PinR2A, PinR2B>) -> InputDriver<PinR1A, PinR1B, PinR2A, PinR2B> {
        InputDriver { rotary1:  left_rotary, rotary2: right_rotary }
    }

    fn update(&mut self) -> Input {
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
