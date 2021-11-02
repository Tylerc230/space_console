use embedded_hal::timer::{CountDown, Periodic};
use arduino_hal::delay_us;
use void::Void;
struct Timer {
    micro_seconds: MicroSeconds
}
impl Periodic for Timer  {
}

struct MicroSeconds(u32);
trait U32Ext {
fn micro_seconds(self) -> MicroSeconds; 
}
impl U32Ext for u32 {
    fn micro_seconds(self) -> MicroSeconds {
        MicroSeconds(self) 
    } 
}
impl CountDown for Timer {
    type Time = MicroSeconds;

    fn start<T>(&mut self, count: T) where T: Into<Self::Time> {
        self.micro_seconds = count.into();
    }


    fn wait(&mut self) -> nb::Result<(), Void> {
        delay_us(self.micro_seconds.0);
        Ok(())
    }
}
