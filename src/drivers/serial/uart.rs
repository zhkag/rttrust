use crate::drivers::core::device::{Device, DeviceOps, DeviceClassType};
use crate::drivers::DeviceRegister;
use crate::to::To;
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
    fn getc(&mut self) -> Option<u8>;
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
    fn register(&mut self, name:&str, ops:T)
    {
        let mut hw_uart = Some(DeviceUart::new());
        let hw_uart_mut = hw_uart.as_mut().unwrap();
        hw_uart_mut.ops = Some(Box::new(ops));
        hw_uart_mut.parent.init(name, DeviceClassType::Char);
        system!(device_register(hw_uart.unwrap()));
    }
}
use crate::Any;
impl DeviceOps for DeviceUart {
    fn name(&self) -> &str {
        self.parent.name()
    }
    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
    fn read(&mut self, _pos:isize, _buffer: Option<*mut ()>, size:usize) -> isize{
        // if buffer.is_none(){ return 0; }
        // match size {
        //     1 => {
        //         let mut binding = buffer.unwrap();
        //         let c = binding.to_self_mut::<char>().unwrap();
        //         *c = self.ops().getc() as char;
        //     },
        //     _ => {},
        // }
        size as isize
    }
    fn write(&mut self, _pos:isize, buffer: Option<*const ()>, size:usize) -> isize{
        if buffer.is_none(){ return 0; }
        match size {
            1 => {
                let binding = buffer.unwrap();
                let c = binding.to_self::<char>().unwrap();
                self.ops().putc(*c);
            },
            _ => {
                let s: &str = unsafe {
                    let ptr_u8: *const u8 = core::mem::transmute(buffer.unwrap());
                    core::str::from_utf8(core::slice::from_raw_parts(ptr_u8, size)).unwrap()
                };
                for c in s.chars(){
                    self.ops().putc(c);
                }
            },
        }
        size as isize
    }
    fn control(&mut self, _cmd:usize, _args: Option<*mut ()>) -> isize{
        0
    }
}
