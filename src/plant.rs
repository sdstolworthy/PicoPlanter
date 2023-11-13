use crate::moisture_reader::MoistureReader;
use crate::waterer::Waterer;

pub struct Plant<'a> {
    moisture_sensor: &'a mut dyn MoistureReader,
    pump: &'a mut dyn Waterer,
    moisture_threshold: u16,
}

impl<'a> Plant<'a> {
    pub fn new(
        moisture_sensor: &'a mut dyn MoistureReader,
        pump: &'a mut dyn Waterer,
    ) -> Plant<'a> {
        Plant {
            moisture_sensor,
            pump,
            moisture_threshold: 2000,
        }
    }

    pub fn needs_water(&mut self) -> bool {
        self.moisture_sensor.read_moisture() > self.moisture_threshold
    }
}

impl MoistureReader for Plant<'_> {
    fn read_moisture(&mut self) -> u16 {
        self.moisture_sensor.read_moisture()
    }
}

impl Waterer for Plant<'_> {
    fn water(&mut self) -> () {
        self.pump.water()
    }
    fn stop_water(&mut self) -> () {
        self.pump.stop_water()
    }
}
