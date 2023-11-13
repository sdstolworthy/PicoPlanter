use embassy_rp::adc::{Adc, Blocking, Channel};
pub trait MoistureReader {
    fn read_moisture(&mut self) -> u16;
}

pub struct MoistureSensor<'a> {
    adc: Adc<'a, Blocking>,
    channel: Channel<'a>,
}

impl<'a> MoistureSensor<'a> {
    pub fn new(adc: Adc<'a, Blocking>, channel: Channel<'a>) -> Self {
        MoistureSensor { adc, channel }
    }
}

impl<'a> MoistureReader for MoistureSensor<'a> {
    fn read_moisture(&mut self) -> u16 {
        self.adc.blocking_read(&mut self.channel).unwrap()
    }
}
