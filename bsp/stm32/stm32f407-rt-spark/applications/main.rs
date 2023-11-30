#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use panic_halt as _;

use stm32f4xx_hal as hal;

use crate::hal::{pac, prelude::*};
use cortex_m_rt::entry;

// #[entry]
// fn main() -> ! {
//     let p = pac::Peripherals::take().unwrap();

//     let gpiof = p.GPIOF.split();
//     let mut led = gpiof.pf12.into_push_pull_output();

//     loop {
//         for _ in 0..10_000 {
//             led.set_high();
//         }
//         for _ in 0..10_000 {
//             led.set_low();
//         }
//     }
// }



// use cortex_m_rt::entry;

use kernel;

#[entry]
fn entry() -> !{
    kernel::entry();
    rtthread_startup();
    loop{}
}

fn rtthread_startup(){
    let p = pac::Peripherals::take().unwrap();
    let gpiof = p.GPIOF.split();
    let mut led = gpiof.pf12.into_push_pull_output();

    loop {
        for _ in 0..10_000 {
            led.set_high();
        }
        for _ in 0..10_000 {
            led.set_low();
        }
    }
}
