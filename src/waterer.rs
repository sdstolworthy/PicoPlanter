use embassy_rp::gpio::{Output, Pin};
pub trait Waterer {
    fn water(&mut self) -> ();
    fn stop_water(&mut self) -> ();
}

pub struct Pump<'a, T>
where
    T: Pin,
{
    pin: Output<'a, T>,
}

impl<'a, T> Pump<'a, T>
where
    T: Pin,
{
    pub fn new(pin: Output<'a, T>) -> Self {
        Pump { pin }
    }
}

impl<'a, T> Waterer for Pump<'a, T>
where
    T: Pin,
{
    fn water(&mut self) -> () {
        self.pin.set_low();
    }

    fn stop_water(&mut self) -> () {
        self.pin.set_high();
    }
}
