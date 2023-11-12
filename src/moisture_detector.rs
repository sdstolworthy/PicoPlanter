use core::convert::Infallible;

use rp_pico::hal::Adc;

use embedded_hal::adc::{Channel, OneShot};

pub trait Waterer {
    fn water(&mut self) -> Option<()>;
}

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

pub struct MoistureLevelWaterer<'a, W>
where
    W: From<u16>,
{
    watering_threshold: u16,
    moisture_delay: u8,
    moisture_reader: &'a mut dyn MoistureReader<W>,
}

impl<'a, W> Waterer for MoistureLevelWaterer<'a, W>
where
    W: From<u16>,
{
    fn water(&mut self) -> Option<()> {
        loop {
            self.moisture_reader.get_moisture();
        }
    }
}

impl<'a, W> MoistureLevelWaterer<'a, W>
where
    W: From<u16>,
{
    pub fn new(moisture_reader: &'a mut dyn MoistureReader<W>) -> Self {
        MoistureLevelWaterer {
            watering_threshold: 2000,
            moisture_delay: 200,
            moisture_reader,
        }
    }
}
