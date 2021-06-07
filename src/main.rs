#![no_std]
#![no_main]

// pick a panicking behavior
// use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m_rt::entry;
use defmt_rtt as _;
use hal::{pac, prelude::*};
use panic_probe as _;
use stm32f7xx_hal as hal;

#[entry]
fn main() -> ! {
    defmt::info!("Hello world!");
    let peripherals = pac::Peripherals::take().unwrap();
    let gpiob = peripherals.GPIOB.split();
    let mut led = gpiob.pb0.into_push_pull_output();

    loop {
        // your code goes here
        for _ in 0..10_000 {
            led.set_high().expect("GPIO can never fail");
        }
        for _ in 0..10_000 {
            led.set_low().expect("GPIO can never fail");
        }
    }
}
