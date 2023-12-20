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

use list::{*};
use thread::Thread;


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

