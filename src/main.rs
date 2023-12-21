#![no_main]
#![no_std]
mod system;
mod scheduler;
mod idle;
mod global_asm;
mod context;
mod cpuport;
mod hw;
mod thread;
mod list;
mod tick;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}


#[no_mangle]
fn main() {
    let mut tick:usize = 0;
    loop {
        tick = tick!(get());
    }
}

#[no_mangle]
fn entry() {
    system!(startup());
    unreachable!();
}

