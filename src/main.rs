#![no_main]
#![no_std]

// set the panic handler
extern crate panic_semihosting;

use cortex_m_rt::entry;
use stm32f1xx_hal::prelude::*;
use void::ResultVoidExt;
use embedded_hal::digital::v2::OutputPin;

#[entry]
fn main() -> ! {
    let core = cortex_m::Peripherals::take().unwrap();
    let device = stm32f1xx_hal::stm32::Peripherals::take().unwrap();
    let mut rcc = device.RCC.constrain();
    let mut flash = device.FLASH.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let mut delay = stm32f1xx_hal::delay::Delay::new(core.SYST, clocks);

    // configure the user led
    let mut gpioc = device.GPIOC.split(&mut rcc.apb2);
    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

    loop {
        delay.delay_ms(1_000_u16);
        led.set_high().void_unwrap();
        delay.delay_ms(100_u16);
        led.set_low().void_unwrap();
    }
}
