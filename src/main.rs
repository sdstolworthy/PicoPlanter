#![no_std]
#![no_main]

mod moisture_detector;

use moisture_detector::{GPIOMoistureReader, MoistureReader};

use defmt::*;
use defmt_rtt as _;
use panic_probe as _;

use rp_pico as bsp;

use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text},
};
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};

use bsp::{
    entry,
    hal::{
        adc::AdcPin,
        clocks::{init_clocks_and_plls, Clock},
        gpio::{FunctionI2C, Pin, PullUp},
        pac,
        sio::Sio,
        Adc, Watchdog,
    },
};

use fugit::RateExtU32;

#[entry]
fn main() -> ! {
    info!("Program start");
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let sio = Sio::new(pac.SIO);

    // External high-speed crystal on the pico board is 12Mhz
    let external_xtal_freq_hz = 12_000_000u32;
    let clocks = init_clocks_and_plls(
        external_xtal_freq_hz,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());
    delay.delay_ms(1000);
    let pins = bsp::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    info!("pins");
    let mut adc = Adc::new(pac.ADC, &mut pac.RESETS);
    // Configure one of the pins as an ADC input
    let mut adc_pin_0 = AdcPin::new(pins.gpio28.into_floating_input());

    let mut reader: moisture_detector::GPIOMoistureReader<
        '_,
        u16,
        AdcPin<
            bsp::hal::gpio::Pin<
                bsp::hal::gpio::bank0::Gpio28,
                bsp::hal::gpio::FunctionSio<bsp::hal::gpio::SioInput>,
                bsp::hal::gpio::PullNone,
            >,
        >,
    > = GPIOMoistureReader::new(&mut adc, &mut adc_pin_0);

    // let mut relay_pin = pins.gpio16.into_push_pull_output();

    // Configure two pins as being I²C, not GPIO
    let sda_pin: Pin<_, FunctionI2C, PullUp> = pins.gpio14.reconfigure();
    let scl_pin: Pin<_, FunctionI2C, PullUp> = pins.gpio15.reconfigure();
    // let not_an_scl_pin: Pin<_, FunctionI2C, PullUp> = pins.gpio20.reconfigure();

    // Create the I²C drive, using the two pre-configured pins. This will fail
    // at compile time if the pins are in the wrong mode, or if this I²C
    // peripheral isn't available on these pins!
    let i2c = rp_pico::hal::I2C::i2c1(
        pac.I2C1,
        sda_pin,
        scl_pin, // Try `not_an_scl_pin` here
        400.kHz(),
        &mut pac.RESETS,
        &clocks.system_clock,
    );

    let interface = I2CDisplayInterface::new(i2c);
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();
    display.init().unwrap();

    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_6X10)
        .text_color(BinaryColor::On)
        .build();

    let initial_text = "Hello, world! ";

    Text::with_baseline(initial_text, Point::zero(), text_style, Baseline::Top)
        .draw(&mut display)
        .unwrap();

    display.flush().unwrap();
    info!("setting relay pin");

    loop {
        delay.delay_ms(500);
        info!("setting pin");
        // if relay_pin.is_low().unwrap() {
        //     relay_pin.set_high().unwrap();
        //     info!("set high");
        //     Text::with_baseline("Hello world! Hi", Point::zero(), text_style, Baseline::Top)
        //         .draw(&mut display)
        //         .unwrap();
        // } else {
        //     relay_pin.set_low().unwrap();
        //     info!("set set low");
        //     Text::with_baseline("Hello world! Low", Point::zero(), text_style, Baseline::Top)
        //         .draw(&mut display)
        //         .unwrap();
        // }

        //let pin_adc_counts: u16 = adc.read(&mut adc_pin_0).unwrap();
        let moisture = reader.get_moisture();
        info!("{}", moisture);
        display.clear_buffer();
        display.flush().unwrap();
        Text::with_baseline("Hello, world", Point::zero(), text_style, Baseline::Top)
            .draw(&mut display)
            .unwrap();
        display.flush().unwrap();
        delay.delay_ms(500);
        display.clear_buffer();
        display.flush().unwrap();

        Text::with_baseline("Hola", Point::zero(), text_style, Baseline::Top)
            .draw(&mut display)
            .unwrap();
        display.flush().unwrap();
    }
}
