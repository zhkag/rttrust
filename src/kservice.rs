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

#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => {
        {
            use core::fmt::Write;
            let mut usart_writer = crate::kservice::UsartWriter {};
            let _ = write!(usart_writer, $($arg)*);
            let _ = write!(usart_writer, "\r\n");
        }
    };
}
