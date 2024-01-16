#![no_main]
#![no_std]
mod object;
pub mod system;
pub mod scheduler;
pub mod idle;
pub mod hw;
pub mod thread;
pub mod list;
pub mod tick;
pub mod timer;
pub mod kservice;
mod libcpu;
mod irq;
mod include;
mod components;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        println!("Panic in {}:{}:{}", location.file(), location.line(),location.column());
    } else {
        println!("Panic in unknown location");
    }
    loop {}
}
