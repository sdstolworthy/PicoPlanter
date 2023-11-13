#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

mod moisture_reader;
mod oled;
mod plant;
mod waterer;

use crate::{
    moisture_reader::MoistureSensor,
    oled::OledScreen,
    plant::Plant,
    waterer::{Pump, Waterer},
};
use defmt::info;
#[allow(unused_imports)]
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_rp::{
    adc::{self, Adc, Channel, InterruptHandler},
    bind_interrupts,
    gpio::{Level, Output},
    i2c::{self, Config, I2c},
};
use embassy_time::{Duration, Timer};
use itoa;
use moisture_reader::MoistureReader;
#[allow(unused_imports)]
use panic_probe as _;
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};

bind_interrupts!(struct Irqs {
    ADC_IRQ_FIFO => InterruptHandler;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    defmt::info!("Initializing...");

    let p = embassy_rp::init(Default::default());
    let adc = Adc::new_blocking(p.ADC, adc::Config::default());

    let mut led = Output::new(p.PIN_25, Level::Low);
    let sda_pin = p.PIN_16;
    let scl_pin = p.PIN_17;
    let moisture_pin = Channel::new_pin(p.PIN_26, embassy_rp::gpio::Pull::None);
    let relay_pin = Output::new(p.PIN_21, Level::High);

    let i2c = i2c::I2c::new_blocking(p.I2C0, scl_pin, sda_pin, Config::default());
    let interface = I2CDisplayInterface::new(i2c);
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();
    display.init().unwrap();
    let mut oled = OledScreen::new(&mut display);

    let mut moisture_sensor = MoistureSensor::new(adc, moisture_pin);
    let mut pump = Pump::new(relay_pin);
    let mut plant = Plant::new(&mut moisture_sensor, &mut pump);

    oled.write("Moisture Meter", 0);

    oled.write("Current moisture:", 1);

    let mut buffer = itoa::Buffer::new();
    loop {
        led.set_high();
        Timer::after(Duration::from_millis(500)).await;

        let moisture_level = plant.read_moisture();

        let moisture_desc = buffer.format(moisture_level);

        info!("{}", moisture_level);

        oled.write(moisture_desc, 2);

        led.set_low();
        Timer::after(Duration::from_millis(500)).await;

        if plant.needs_water() {
            oled.write("Pump on", 3);
            info!("turning on relay");
            plant.water();
        } else {
            oled.write("Pump off", 3);
            info!("turning off relay");
            plant.stop_water();
        }
    }
}

trait PlantController: Waterer + MoistureReader {}

struct WateringManager<'a> {
    controllers: [&'a dyn PlantController; 12],
}
