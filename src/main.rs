
#![no_main]
#![no_std]
mod global_asm;
mod context;
mod cpuport;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}


use stm32f4xx_hal as hal;

use crate::hal::{pac, prelude::*};

use core::fmt::Write; // for pretty formatting of the serial output

#[no_mangle]
fn entry() {

    let dp = pac::Peripherals::take().unwrap();

    let gpioa = dp.GPIOA.split();

    let rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.use_hse(25.MHz()).freeze();

    let mut delay = dp.TIM1.delay_ms(&clocks);

    // define RX/TX pins
    let tx_pin = gpioa.pa9;

    // configure serial
    // let mut tx = Serial::tx(dp.USART1, tx_pin, 9600.bps(), &clocks).unwrap();
    // or
    let mut tx = dp.USART1.tx(tx_pin, 9600.bps(), &clocks).unwrap();

    let mut value: u8 = 0;

    loop {
        // print some value every 500 ms, value will overflow after 255
        writeln!(tx, "value: {value:02}\r").unwrap();
        value = value.wrapping_add(1);
        delay.delay(2.secs());
    }
    
    let input: u32 = 0;
    unsafe{context::rt_hw_context_switch_to(input);}

    loop {
    }
}


// #[export_name = "entry"]
// unsafe extern "C" fn entry() {
//     let input: u32 = 0;
//     context::rt_hw_context_switch_to(input);


//     loop {
//     }
// }




// #![no_main]
// #![no_std]

// use panic_halt as _;

// use cortex_m_rt::entry;

// #[entry]
// fn main() -> ! {
//     let dp = pac::Peripherals::take().unwrap();

//     let gpioa = dp.GPIOA.split();

//     let rcc = dp.RCC.constrain();

//     let clocks = rcc.cfgr.use_hse(25.MHz()).freeze();

//     let mut delay = dp.TIM1.delay_ms(&clocks);

//     // define RX/TX pins
//     let tx_pin = gpioa.pa9;

//     // configure serial
//     // let mut tx = Serial::tx(dp.USART1, tx_pin, 9600.bps(), &clocks).unwrap();
//     // or
//     let mut tx = dp.USART1.tx(tx_pin, 9600.bps(), &clocks).unwrap();

//     let mut value: u8 = 0;

//     loop {
//         // print some value every 500 ms, value will overflow after 255
//         writeln!(tx, "value: {value:02}\r").unwrap();
//         value = value.wrapping_add(1);
//         delay.delay(2.secs());
//     }
// }

