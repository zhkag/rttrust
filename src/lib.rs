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

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
fn entry() {
    system!(startup());
    unreachable!();
}
