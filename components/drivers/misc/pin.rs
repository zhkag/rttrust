use crate::drivers::core::device::{Device, DeviceOps, DeviceClassType};

use kernel::println;

pub trait PinOps
{
    fn pin_mode(&mut self,  _pin: isize, _mode: u8);
    fn pin_write(&mut self,  _pin: isize, _value: u8);
    fn pin_read(&mut self,  _pin: isize);
    fn pin_detach_irq(&mut self,  _pin: isize);
    fn pin_irq_enable(&mut self,  _pin: isize, _enabled: u8);
    // fn pin_get(const char *name);
}


#[repr(C)]
pub struct DevicePin
{
    parent:Device,
    pub ops: Option<*mut dyn PinOps>,
}

static mut _HW_PIN: Option<DevicePin> = None;

impl DevicePin {
    pub fn new() -> Self   {
        DevicePin{
            parent:Device::new(),
            ops: None,
        }
    }
    pub fn ops(&self) -> &mut dyn PinOps{
        unsafe { &mut *(self.ops.unwrap())}
    }
    
    fn device_to_pin(&self, parent: *mut Device) -> &mut DevicePin {
        #[allow(deref_nullptr)]
        unsafe { &mut *((parent as usize - (&(&*(0 as *const DevicePin)).parent) as *const Device as usize) as *mut DevicePin) }
    }

    pub fn register(name:&str, ops:*mut dyn PinOps)
    {
        let _hw_pin = unsafe {&mut _HW_PIN};
        *_hw_pin = Some(DevicePin::new());
        let _hw_pin_mut = _hw_pin.as_mut().unwrap();
        _hw_pin_mut.ops = Some(ops);
        _hw_pin_mut.parent.init(DeviceClassType::Pin);
        _hw_pin_mut.parent.register(name);
    }
}


impl DeviceOps for DevicePin {
    fn read(&mut self, pos:isize, buffer: *mut (), size:usize) -> isize{
        self.ops().pin_read(0);
        0
    }
    fn write(&mut self, pos:isize, buffer: *const (), size:usize) -> isize{0
    }
    fn control(&mut self,size:usize) -> isize{0
    }
}


use kernel::macros::init_export;
#[init_export("6")]
fn device_pin_test6() {
    let pin = DevicePin::new();
    if let Some(device) = pin.parent.find("pin"){
        let pin = pin.device_to_pin(device);
        pin.ops().pin_read(0);
    }
    
}
