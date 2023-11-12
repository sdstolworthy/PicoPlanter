use core::convert::Infallible;

use rp_pico::hal::Adc;

use embedded_hal::adc::{Channel, OneShot};

pub struct GPIOMoistureReader<'a, W, S>
where
    S: Channel<Adc, ID = u8>,
    W: From<u16>,
{
    pub pin: &'a mut S,
    adc: &'a mut dyn OneShot<Adc, W, S, Error = Infallible>,
}

impl<'a, W, S> GPIOMoistureReader<'a, W, S>
where
    S: Channel<Adc, ID = u8>,
    W: From<u16>,
{
    pub fn new(adc: &'a mut dyn OneShot<Adc, W, S, Error = Infallible>, pin: &'a mut S) -> Self {
        GPIOMoistureReader { pin, adc }
    }
}

pub trait MoistureReader<W>
where
    W: From<u16>,
{
    fn get_moisture(&mut self) -> W;
}

impl<'a, W, S> MoistureReader<W> for GPIOMoistureReader<'a, W, S>
where
    S: Channel<Adc, ID = u8>,
    W: From<u16>,
{
    fn get_moisture(&mut self) -> W {
        self.adc.read(self.pin).unwrap()
    }
}
