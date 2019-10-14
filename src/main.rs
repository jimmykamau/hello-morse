#![no_main]
#![no_std]

// set the panic handler
extern crate panic_semihosting;

extern crate cortex_m_semihosting;
extern crate heapless;

mod alphabet;

use cortex_m_rt::entry;
use stm32f1xx_hal::prelude::*;
use void::ResultVoidExt;
use embedded_hal::digital::v2::OutputPin;
use heapless::String;
use heapless::consts::*;
use cortex_m_semihosting::dbg;
pub use alphabet::morse_map;

const DIT: u16 = 200;
const DAH: u16 = DIT * 3;
const WORD_SPACE: u16 = DAH * 2;

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

    let my_string: String<U16> = String::from("hello morse");
    let letter_mapping = morse_map::get_morse_map();

    let mut show_word = || {
        led.set_high().void_unwrap();

        for c in my_string.chars() {
            match letter_mapping.get(&c) {
                Some(letter) => {
                    for symbol in letter.chars() {
                        match symbol {
                            '.' => {
                                led.set_low().void_unwrap();
                                delay.delay_ms(DIT);
                                led.set_high().void_unwrap();
                            },
                            '-' => {
                                led.set_low().void_unwrap();
                                delay.delay_ms(DAH);
                                led.set_high().void_unwrap();
                            },
                            ' ' => {
                                led.set_high().void_unwrap();
                                delay.delay_ms(WORD_SPACE);
                            },
                            _ => {
                                dbg!("No matched symbol");
                            }
                        };
                        delay.delay_ms(DIT);
                    };
                    delay.delay_ms(DAH);
                },
                _ => {
                    dbg!("No matched letter");
                }
            };
            delay.delay_ms(WORD_SPACE);
        };
    };

    loop {
        show_word();
    }
}
