#![no_main]
#![no_std]
pub mod object;
pub mod system;
pub mod scheduler;
pub mod idle;
pub mod bsp;
pub mod thread;
pub mod list;
pub mod tick;
pub mod timer;
pub mod kservice;
mod libcpu;
mod irq;
mod include;
mod components;
mod mem;
mod heaplist;
pub extern crate alloc;
pub use alloc::boxed::Box;

pub use core::result::Result as Result;

pub use libcpu::{LibcpuTrait, sys_tick};
pub use bsp::BspTrait;
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

#[derive(PartialEq)]
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

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Error type : ")?;
        let str = match self {
            Self::Ok => "There is no error",
            Self::Error => "A generic/unknown error happens",
            Self::TimeOut => "Timed out",
            Self::Full => "The resource is full",
            Self::Empty => "The resource is empty",
            Self::NoMem => "No memory",
            Self::NoSys => "Function not implemented",
            Self::Busy => "Busy",
            Self::IO => "IO error",
            Self::Intr => "Interrupted system call",
            Self::Inval => "Invalid argument",
            Self::NoEnt => "No entry",
            Self::NoSpc => "No space left",
            Self::Perm => "Operation not permitted",
            Self::Trap => "Trap event",
            Self::Fault => "Bad address ",
            Self::NoBufs => "No buffer space is available",
        };
        write!(f, "{}!",str)
    }
}

pub type ResultE<R> = Result<R, Error>;
