use crate::drivers::core::device::{Device, DeviceOps, DeviceClassType};
use crate::drivers::DeviceRegister;

use crate::Box;
use crate::system;
#[repr(C)]
pub struct SerialConfigure {
    pub baud_rate: u32,

    pub data_bits: u32,
    pub stop_bits: u32,
    pub parity: u32,
    pub bit_order: u32,
    pub invert: u32,
    pub bufsz: u32,
    pub flowcontrol: u32,
    pub reserved: u32,
}

pub trait UartOps
{
    fn configure(&mut self,  _cfg: &mut SerialConfigure);
    fn control(&mut self,  _cmd: usize, args: Option<*mut ()>);
    fn putc(&mut self,  _c: char);
    fn getc(&mut self) -> u8;
    // fn dma_transmit(&mut self);
}


#[repr(C)]
pub struct DeviceUart
{
    parent:Device,
    // config:SerialConfigure,
    pub ops: Option<Box<dyn UartOps>>,
}

impl DeviceUart {
    pub fn new() -> Self   {
        DeviceUart{
            parent:Device::new(),
            ops: None,
        }
    }
    pub fn ops(&mut self) -> &mut Box<dyn UartOps>{
        self.ops.as_mut().unwrap()
    }
}

impl<T: UartOps + 'static> DeviceRegister<T> for DeviceUart {
    fn register(&mut self, _name:&str, ops:T)
    {
        let mut hw_uart = Some(DeviceUart::new());
        let hw_uart_mut = hw_uart.as_mut().unwrap();
        hw_uart_mut.ops = Some(Box::new(ops));
        hw_uart_mut.parent.init(DeviceClassType::Char);
        system!(device_register(hw_uart.unwrap()));
    }
}

impl DeviceOps for DeviceUart {
    fn name(&self) -> &str {
        "uart1"
    }
    fn read(&mut self, _pos:isize, _buffer: Option<*mut ()>, size:usize) -> isize{
        size as isize
    }
    fn write(&mut self, _pos:isize, buffer: Option<*const ()>, size:usize) -> isize{
        if buffer.is_none() || size != core::mem::size_of::<char>() { return 0; }
        let pin_value = unsafe { &mut *(buffer.unwrap() as *mut char)};
        self.ops().putc(*pin_value);
        size as isize
    }
    fn control(&mut self, _cmd:usize, _args: Option<*mut ()>) -> isize{
        0
    }
}
