use crate::drivers::core::device::{Device, DeviceOps, DeviceClassType};
use crate::drivers::DeviceRegister;

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
    pub ops: Option<*mut dyn UartOps>,
}

impl DeviceUart {
    pub fn new() -> Self   {
        DeviceUart{
            parent:Device::new(),
            ops: None,
        }
    }
    pub fn ops(&self) -> &mut dyn UartOps{
        unsafe { &mut *(self.ops.unwrap())}
    }
    pub fn find(name:&str)->Option<&mut DeviceUart>{
        if let Some(device) = Device::find(name){
            return Some(DeviceUart::device_to_uart(device));
        }
        None
    }
    
    fn device_to_uart(parent: *mut Device) -> &'static mut DeviceUart {
        #[allow(deref_nullptr)]
        unsafe { &mut *((parent as usize - (&(&*(0 as *const DeviceUart)).parent) as *const Device as usize) as *mut DeviceUart) }
    }
}

impl<T: UartOps + 'static> DeviceRegister<T> for DeviceUart {
    fn register(&mut self, name:&str, ops:*mut T)
    {
        self.ops = Some(ops);
        self.parent.init(DeviceClassType::Char);
        self.parent.register(name);
    }
}

impl DeviceOps for DeviceUart {
    fn read(&mut self, _pos:isize, _buffer: Option<*mut ()>, size:usize) -> isize{
        size as isize
    }
    fn write(&mut self, _pos:isize, _buffer: Option<*const ()>, size:usize) -> isize{
        size as isize
    }
    fn control(&mut self, _cmd:usize, _args: Option<*mut ()>) -> isize{
        0
    }
}
