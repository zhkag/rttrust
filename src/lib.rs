#![no_main]
#![no_std]
pub mod object;
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

pub use core::result::Result as Result;

pub use libcpu::{LibcpuTrait, sys_tick};
use core::panic::PanicInfo;

pub extern crate macros;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        println!("Panic in {}:{}:{}", location.file(), location.line(),location.column());
    } else {
        println!("Panic in unknown location");
    }
    loop {}
}

#[derive(Copy, Clone)]
pub enum Error {
    Ok,
    Error,
    TimeOut,
    Full,
    Empty,
    NoMem,
    NoSys,
    Busy,
    IO,
    Intr,
    Inval,
    NoEnt,
    NoSpc,
    Perm,
    Trap,
    Fault,
    NoBufs,
}

pub type ResultE<R> = Result<R, Error>;
