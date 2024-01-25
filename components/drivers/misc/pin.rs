use crate::drivers::core::device::{Device, DeviceOps};
#[repr(C)]
struct DevicePin
{
    parent:Device,
}

impl DeviceOps for DevicePin {
    // fn rx_indicate(&mut self, size:usize) -> isize{
    // }
    // fn tx_complete(&mut self, buffer: *mut ()) -> isize{
    // }
    // fn init(&mut self) -> isize{
    // }
    // fn open(&mut self, oflag:u16) -> isize{
    // }
    // fn close(&mut self) -> isize{
    // }
    // fn read(&mut self, pos:isize, buffer: *mut (), size:usize) -> isize{
    // }
    // fn write(&mut self, pos:isize, buffer: *const (), size:usize) -> isize{
    // }
    // fn control(&mut self,size:usize) -> isize{
    // }
}
