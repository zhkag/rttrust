use crate::drivers::core::device::{Device, DeviceOps, DeviceClassType};

pub trait PinOps
{
    fn pin_mode(&mut self,  _pin: usize, _mode: u8);
    fn pin_write(&mut self,  _pin: usize, _value: bool);
    fn pin_read(&mut self,  _pin: usize) -> bool;
    fn pin_detach_irq(&mut self,  _pin: usize);
    fn pin_irq_enable(&mut self,  _pin: usize, _enabled: u8);
    fn pin_get(&mut self, _name:&str) -> usize;
}


#[repr(C)]
pub struct DevicePin
{
    parent:Device,
    pub ops: Option<*mut dyn PinOps>,
}

pub struct DevicePinValue
{
    pin:usize,
    value:bool,
}

pub struct DevicePinMode
{
    pin:usize,
    mode:u8,
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
    pub fn find(name:&str)->Option<&mut DevicePin>{
        if let Some(device) = Device::find(name){
            return Some(DevicePin::device_to_pin(device));
        }
        None
    }
    
    fn device_to_pin(parent: *mut Device) -> &'static mut DevicePin {
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
    fn read(&mut self, _pos:isize, buffer: Option<*mut ()>, size:usize) -> isize{
        if buffer.is_none() || size != core::mem::size_of::<DevicePinValue>() { return 0; }
        let pin_value = unsafe { &mut *(buffer.unwrap() as *mut DevicePinValue)};
        pin_value.value = self.ops().pin_read(pin_value.pin);
        size as isize
    }
    fn write(&mut self, _pos:isize, buffer: Option<*const ()>, size:usize) -> isize{
        if buffer.is_none() || size != core::mem::size_of::<DevicePinValue>() { return 0; }
        let pin_value = unsafe { &*(buffer.unwrap() as *const DevicePinValue)};
        self.ops().pin_write(pin_value.pin, pin_value.value);
        size as isize
    }
    fn control(&mut self, _cmd:usize, args: Option<*mut ()>) -> isize{
        if args.is_none(){ return -1;}
        let pin_mode = unsafe { &mut *(args.unwrap() as *mut DevicePinMode)};
        self.ops().pin_mode(pin_mode.pin, pin_mode.mode);
        0
    }
}

impl DevicePinValue {
    pub fn init(pin:usize, value:bool) -> Self{
        DevicePinValue{pin,value}
    }
    pub fn r#const(&self) -> Option<*const()> {
        Some(self as *const DevicePinValue as *const())
    }
    pub fn r#mut(&mut self) -> Option<*mut()> {
        Some(self as *mut DevicePinValue as *mut())
    }
    pub fn set_value(&mut self, value:bool){
        self.value = value;
    }
}

impl DevicePinMode {
    pub fn init(pin:usize, mode:u8) -> Self{
        DevicePinMode{pin,mode}
    }
    pub fn r#mut(&mut self) -> Option<*mut()> {
        Some(self as *mut DevicePinMode as *mut())
    }
}
