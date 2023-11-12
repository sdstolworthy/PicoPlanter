#![no_std]
#![no_main]

mod moisture_detector;
use moisture_detector::{GPIOMoistureReader, MoistureReader};

use defmt::*;
use defmt_rtt as _;
use embedded_hal::digital::v2::{InputPin, OutputPin};
use panic_probe as _;

use rp_pico as bsp;

use bsp::{
    entry,
    hal::{
        adc::AdcPin,
        //    clocks::{init_clocks_and_plls, Clock},
        //    gpio::{bank0::Gpio28, AnyPin, SpecificPin},
        pac,
        sio::Sio,
        Adc,
    },
};

#[entry]
fn main() -> ! {
    info!("Program start");
    let mut pac = pac::Peripherals::take().unwrap();
    let sio = Sio::new(pac.SIO);

    // External high-speed crystal on the pico board is 12Mhz
    //    let external_xtal_freq_hz = 12_000_000u32;
    //    let clocks = init_clocks_and_plls(
    //        external_xtal_freq_hz,
    //        pac.XOSC,
    //        pac.CLOCKS,
    //        pac.PLL_SYS,
    //        pac.PLL_USB,
    //        &mut pac.RESETS,
    //        &mut watchdog,
    //    )
    //    .ok()
    //    .unwrap();
    //
    //    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    let pins = bsp::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // This is the correct pin on the Raspberry Pico board. On other boards, even if they have an
    // on-board LED, it might need to be changed.
    // Notably, on the Pico W, the LED is not connected to any of the RP2040 GPIOs but to the cyw43 module instead. If you have
    // a Pico W and want to toggle a LED with a simple GPIO output pin, you can connect an external
    // LED to one of the GPIO pins, and reference that pin here.
    //    let led_pin = pins.led.into_push_pull_output();
    // Enable adc
    let mut adc = Adc::new(pac.ADC, &mut pac.RESETS);
    // Configure one of the pins as an ADC input
    let mut adc_pin_0 = AdcPin::new(pins.gpio28.into_floating_input());
    info!("pins");

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

    info!("{}", reader.get_moisture());

    //    info!("setting relay pin");
    //    let mut relay_pin = pins.gpio15.into_push_pull_output();
    //
    //    info!("setting pin");
    //    if relay_pin.is_low().unwrap() {
    //        relay_pin.set_high().unwrap();
    //        info!("set high");
    //    } else {
    //        relay_pin.set_low().unwrap();
    //        info!("set set low");
    //    }

    // Read the ADC counts from the ADC channel
    loop {
        info!("get moisture");
        //let pin_adc_counts: u16 = adc.read(&mut adc_pin_0).unwrap();
        let moisture = reader.get_moisture();
        //let pin_adc_counts: u16 = adc.read(&mut adc_pin_0).unwrap();
        info!("{}", moisture);
    }
}

//        info!("on!");
//        led_pin.set_high().unwrap();
// delay.delay_ms(500);
//        info!("off!");
//        led_pin.set_low().unwrap();
//        delay.delay_ms(500);
