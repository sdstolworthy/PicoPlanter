#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::info;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_rp::{
    adc::{self, Adc, Channel, InterruptHandler},
    bind_interrupts,
    gpio::{Level, Output},
    i2c::{self, Blocking, Config, I2c},
    peripherals::I2C0,
};
use embassy_time::{Duration, Timer};
use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{PrimitiveStyle, Rectangle},
    text::{Baseline, Text},
};
use itoa;
use panic_probe as _;
use ssd1306::{mode::BufferedGraphicsMode, prelude::*, I2CDisplayInterface, Ssd1306};

bind_interrupts!(struct Irqs {
    ADC_IRQ_FIFO => InterruptHandler;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    defmt::info!("Initializing...");

    let p = embassy_rp::init(Default::default());
    let mut led = Output::new(p.PIN_25, Level::Low);

    defmt::info!("Initialized.");
    let sda = p.PIN_16;
    let scl = p.PIN_17;
    let i2c = i2c::I2c::new_blocking(p.I2C0, scl, sda, Config::default());
    let interface = I2CDisplayInterface::new(i2c);

    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();
    display.init().unwrap();

    let mut oled = OledScreen::new(&mut display);

    let mut adc = Adc::new(p.ADC, Irqs, adc::Config::default());

    let mut p26 = Channel::new_pin(p.PIN_26, embassy_rp::gpio::Pull::None);

    let mut buffer = itoa::Buffer::new();

    let mut relay = Output::new(p.PIN_21, Level::High);

    oled.write("Moisture Meter", 0);

    oled.write("Current moisture:", 1);

    loop {
        led.set_high();
        Timer::after(Duration::from_millis(500)).await;

        let moisture_level = adc.read(&mut p26).await.unwrap();

        let moisture_desc = buffer.format(moisture_level);

        info!("{}", moisture_level);

        oled.write(moisture_desc, 2);

        led.set_low();
        Timer::after(Duration::from_millis(500)).await;

        if moisture_level > 2400 {
            oled.write("Pump on", 3);
            info!("turning on relay");
            relay.set_low();
        } else if moisture_level < 2000 {
            oled.write("Pump off", 3);
            info!("turning off relay");
            relay.set_high();
        }
    }
}

struct OledScreen<'a> {
    line_height: i32,
    display: &'a mut Ssd1306<
        I2CInterface<I2c<'a, I2C0, Blocking>>,
        ssd1306::prelude::DisplaySize128x64,
        BufferedGraphicsMode<ssd1306::prelude::DisplaySize128x64>,
    >,
    text_style: MonoTextStyle<'a, BinaryColor>,
    clear_style: PrimitiveStyle<BinaryColor>,
}

impl<'a> OledScreen<'a> {
    fn new(
        display: &'a mut Ssd1306<
            I2CInterface<I2c<'a, I2C0, Blocking>>,
            ssd1306::prelude::DisplaySize128x64,
            BufferedGraphicsMode<ssd1306::prelude::DisplaySize128x64>,
        >,
    ) -> Self {
        let clear_style = PrimitiveStyle::with_fill(BinaryColor::Off);
        let text_style = MonoTextStyleBuilder::new()
            .font(&FONT_6X10)
            .text_color(BinaryColor::On)
            .build();
        OledScreen {
            line_height: 16,
            display,
            text_style,
            clear_style,
        }
    }

    fn get_line_bound(&mut self, line: i32) -> Point {
        Point::new(0, line * self.line_height)
    }
    pub fn write(&mut self, s: &str, line: i32) -> () {
        let line_start = self.get_line_bound(line);
        self.clear_line(line);
        self.display.flush().unwrap();
        Text::with_baseline(s, line_start, self.text_style, Baseline::Top)
            .draw(self.display)
            .unwrap();
        self.update();
    }
    fn clear_line(&mut self, line: i32) -> () {
        let width = self.display.size().width;
        Rectangle::new(
            self.get_line_bound(line),
            Size::new(width, self.line_height as u32),
        )
        .into_styled(self.clear_style)
        .draw(self.display)
        .unwrap();
    }
    fn update(&mut self) -> () {
        self.display.flush().unwrap();
    }
}
