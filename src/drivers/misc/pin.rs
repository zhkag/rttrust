use crate::drivers::core::device::{Device, DeviceRegister, DeviceOps, DeviceClassType};
use crate::Box;
use crate::system;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum PinState {
    LOW,
    HIGH
}

impl From<PinState> for bool {
    fn from(value: PinState) -> bool {
        match value {
            PinState::LOW => false,
            PinState::HIGH => true,
        }
    }
}

impl From<bool> for PinState {
    fn from(value: bool) -> PinState {
        match value {
            false => PinState::LOW,
            true => PinState::HIGH,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum PinMode {
    OUTPUT = 0,
    INPUT,
    InputPullup,
    InputPulldown,
    OutputOd,
}

impl From<PinMode> for usize {
    fn from(value: PinMode) -> usize {
        match value {
            PinMode::OUTPUT => 0,
            PinMode::INPUT => 1,
            PinMode::InputPullup => 2,
            PinMode::InputPulldown => 3,
            PinMode::OutputOd => 4,
        }
    }
}

impl From<usize> for PinMode {
    fn from(value: usize) -> PinMode {
        match value {
            0 => PinMode::OUTPUT,
            1 => PinMode::INPUT,
            2 => PinMode::InputPullup,
            3 => PinMode::InputPulldown,
            4 => PinMode::OutputOd,
            _ => unreachable!(),
        }
    }
}

pub trait PinOps
{
    fn pin_mode(&mut self,  _pin: usize, _mode: PinMode);
    fn pin_write(&mut self,  _pin: usize, _value: PinState);
    fn pin_read(&mut self,  _pin: usize) -> PinState;
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
use crate::To;
#[derive(To)]
pub struct DevicePinValue
{
    pin:usize,
    value:PinState,
}

#[derive(To)]
pub struct DevicePinMode
{
    pin:usize,
    mode:PinMode,
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
    fn register(&mut self, name:&str, ops:T)
    {
        let mut hw_pin = Some(DevicePin::new());
        let _hw_pin_mut = hw_pin.as_mut().unwrap();
        _hw_pin_mut.ops = Some(Box::new(ops));
        _hw_pin_mut.parent.init(name, DeviceClassType::Pin);
        system!(device_register(hw_pin.unwrap()));
    }
}
use crate::Any;
impl DeviceOps for DevicePin {
    fn name(&self) -> &str {
        self.parent.name()
    }
    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
    fn read(&mut self, _pos:isize, buffer: Option<*mut ()>, size:usize) -> isize{
        if buffer.is_none() || size != core::mem::size_of::<DevicePinValue>() { return 0; }
        let mut binding = buffer.unwrap();
        let pin_value = binding.to_self_mut::<DevicePinValue>().unwrap();
        pin_value.value = self.ops().pin_read(pin_value.pin);
        size as isize
    }
    fn write(&mut self, _pos:isize, buffer: Option<*const ()>, size:usize) -> isize{
        if buffer.is_none() || size != core::mem::size_of::<DevicePinValue>() { return 0; }
        let binding = buffer.unwrap();
        let pin_value = binding.to_self::<DevicePinValue>().unwrap();
        self.ops().pin_write(pin_value.pin, pin_value.value);
        size as isize
    }
    fn control(&mut self, _cmd:usize, args: Option<*mut ()>) -> isize{
        if args.is_none(){ return -1;}
        let mut binding = args.unwrap();
        let pin_mode = binding.to_self_mut::<DevicePinMode>().unwrap();
        self.ops().pin_mode(pin_mode.pin, pin_mode.mode);
        0
    }
}

impl DevicePinValue {
    pub fn init(pin:usize, value:PinState) -> Self{
        DevicePinValue{pin,value}
    }
    pub fn set_value(&mut self, value:PinState){
        self.value = value;
    }
}

impl DevicePinMode {
    pub fn init(pin:usize, mode:PinMode) -> Self{
        DevicePinMode{pin,mode}
    }
}

pub fn pin_get(name:&str) -> usize{
    if let Some(device_pin) = crate::derive_find!("pin").unwrap().as_any().downcast_mut::<DevicePin>() {
        return device_pin.ops().pin_get(name)
    }
    0
}

pub fn pin_mode(pin: usize, mode: PinMode){
    if let Some(device_pin) = crate::derive_find!("pin").unwrap().as_any().downcast_mut::<DevicePin>() {
        device_pin.ops().pin_mode(pin, mode);
    }
}

pub fn pin_write(pin: usize, value: PinState){
    if let Some(device_pin) = crate::derive_find!("pin").unwrap().as_any().downcast_mut::<DevicePin>() {
        device_pin.ops().pin_write(pin, value);
    }
}

pub fn pin_read(pin: usize) -> PinState{
    if let Some(device_pin) = crate::derive_find!("pin").unwrap().as_any().downcast_mut::<DevicePin>() {
        return device_pin.ops().pin_read(pin)
    }
    PinState::LOW
}
