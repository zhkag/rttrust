use crate::drivers::core::device::{Device, DeviceRegister, DeviceOps, DeviceClassType, DeviceSelf};
use crate::Box;
use crate::system;
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
    pub ops: Option<Box<dyn PinOps>>,
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

impl DevicePin {
    pub fn new() -> Self   {
        DevicePin{
            parent:Device::new(),
            ops: None,
        }
    }
    pub fn ops(&mut self) -> &mut Box<dyn PinOps>{
        self.ops.as_mut().unwrap()
    }
}

impl<T: PinOps + 'static> DeviceRegister<T> for DevicePin {
    fn register(&mut self, _name:&str, ops:T)
    {
        let mut hw_pin = Some(DevicePin::new());
        let _hw_pin_mut = hw_pin.as_mut().unwrap();
        _hw_pin_mut.ops = Some(Box::new(ops));
        _hw_pin_mut.parent.init(DeviceClassType::Pin);
        system!(device_register(hw_pin.unwrap()));
    }
}

impl DeviceOps for DevicePin {
    fn name(&self) -> &str {
        "pin"
    }
    fn device_self(&mut self) -> Option<DeviceSelf> {
        Some(DeviceSelf::Pin(self))
    }
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

pub fn pin_get(name:&str) -> usize{
    let pin_self = system!(device_list_mut()).get_mut("pin").unwrap().device_self().unwrap();
    if let DeviceSelf::Pin(device_pin) = pin_self {
        return device_pin.ops().pin_get(name)
    }
    0
}

pub fn pin_mode(pin: usize, mode: u8){
    let pin_self = system!(device_list_mut()).get_mut("pin").unwrap().device_self().unwrap();
    if let DeviceSelf::Pin(device_pin) = pin_self {
        device_pin.ops().pin_mode(pin, mode);
    }
}

pub fn pin_write(pin: usize, value: bool){
    let pin_self = system!(device_list_mut()).get_mut("pin").unwrap().device_self().unwrap();
    if let DeviceSelf::Pin(device_pin) = pin_self {
        device_pin.ops().pin_write(pin, value);
    }
}

pub fn pin_read(pin: usize) -> bool{
    let pin_self = system!(device_list_mut()).get_mut("pin").unwrap().device_self().unwrap();
    if let DeviceSelf::Pin(device_pin) = pin_self {
        return device_pin.ops().pin_read(pin)
    }
    false
}
