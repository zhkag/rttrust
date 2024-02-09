use crate::println;
use crate::include;

pub struct UsartWriter {
}

impl core::fmt::Write for UsartWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for c in s.chars() {
            crate::hw::putc(c);
        }
        Ok(())
    }
}

pub fn show_version() {
    println!();
    println!(" \\ | /");
    println!("- RT -     Rust Operating System");
    println!(" / | \\     {} build {}", include::VERSION, include::BUILD_DATE);
    println!(" 2023 - {} Copyright by rttrust", &include::BUILD_DATE[0..4]);
}

#[macro_export]
macro_rules! println {
    () => {
        {
            use core::fmt::Write;
            let mut usart_writer = $crate::kservice::UsartWriter {};
            let _ = write!(usart_writer, "\r\n");
        }
    };
    ($($arg:tt)*) => {
        {
            use core::fmt::Write;
            let mut usart_writer = $crate::kservice::UsartWriter {};
            let _ = write!(usart_writer, $($arg)*);
            let _ = write!(usart_writer, "\r\n");
        }
    };
}

#[macro_export]
macro_rules! align {
    ($size:expr, $align:expr) => {
        (($size) + ($align) - 1) & !($align - 1)
    };
}

#[macro_export]
macro_rules! align_down {
    ($size:expr, $align:expr) => {
        ($size) & !($align - 1)
    };
}
