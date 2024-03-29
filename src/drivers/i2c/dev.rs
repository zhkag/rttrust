use crate::Box;
use crate::system;
use crate::DeviceOps;
use crate::drivers::core::device::Device;
use crate::drivers::DeviceRegister;
use crate::drivers::core::device::DeviceClassType;

use super::core::I2cMsg;
use super::core::I2cState;
use super::core::I2cPrivData;


#[repr(C)]
pub enum I2cDevCtrl {
    BIT10 = 1,
    ADDR,
    TIMEOUT,
    RW,
    CLK,
    UNLOCK,
    GetState,
    GetMode,
    GetError,
}

impl From<usize> for I2cDevCtrl {
    fn from(value:usize ) -> I2cDevCtrl {
        match value {
            1 => I2cDevCtrl::BIT10,
            2 => I2cDevCtrl::ADDR,
            3 => I2cDevCtrl::TIMEOUT,
            4 => I2cDevCtrl::RW,
            5 => I2cDevCtrl::CLK,
            6 => I2cDevCtrl::UNLOCK,
            7 => I2cDevCtrl::GetState,
            8 => I2cDevCtrl::GetMode,
            9 => I2cDevCtrl::GetError,
            _ => unreachable!(),
        }
    }
}

use crate::Vec;

pub trait I2cBusOps
{
    fn master_xfer(&mut self, _msgs:Vec<&I2cMsg>) -> usize{0}
    fn slave_xfer(&mut self, _msgs:Vec<&I2cMsg>) -> usize{0}
    fn i2c_bus_control(&mut self, _cmd:usize, _args:Option<*mut ()>){}
}

#[repr(C)]
pub struct DeviceI2cBus
{
    parent:Device,
    flags:u16,
    timeout:usize,
    r#priv:Option<*mut ()>,
    pub ops: Option<Box<dyn I2cBusOps>>,
}
impl DeviceI2cBus {
    pub fn new() -> Self   {
        DeviceI2cBus{
            parent:Device::new(),
            flags: 0,
            timeout: 0,
            r#priv: None,
            ops: None,
        }
    }
    pub fn ops(&mut self) -> &mut Box<dyn I2cBusOps>{
        self.ops.as_mut().unwrap()
    }
    fn master_recv(&mut self, addr:u16, flags:u16,buffer:Option<*mut u8>, count:usize) -> isize{
        let msg: I2cMsg = I2cMsg::init(addr, flags, count, buffer);
        let mut msgs:Vec<&I2cMsg> = Vec::new();
        msgs.push(&msg);
        self.transfer(msgs);
        0
    }
    fn master_send(&mut self, addr:u16, flags:u16,buffer:Option<*mut u8>, count:usize) -> isize{
        let msg = I2cMsg::init(addr, flags | (usize::from(I2cState::RD) as u16), count, buffer);
        let mut msgs:Vec<&I2cMsg> = Vec::new();
        msgs.push(&msg);
        self.transfer(msgs);
        0
    }

    pub fn transfer(&mut self, msgs:Vec<&I2cMsg>) -> usize{
        self.ops().master_xfer(msgs);
        0
    }
}

impl<T: I2cBusOps + 'static> DeviceRegister<T> for DeviceI2cBus {
    fn register(&mut self, name:&str, ops:T)
    {
        let mut hw_i2c = Some(DeviceI2cBus::new());
        let hw_i2c_mut = hw_i2c.as_mut().unwrap();
        hw_i2c_mut.ops = Some(Box::new(ops));
        hw_i2c_mut.parent.init(name , DeviceClassType::I2CBUS);
        system!(device_register(hw_i2c.unwrap()));
    }
}

use crate::Any;


impl DeviceOps for DeviceI2cBus {
    fn name(&self) -> &str {
        self.parent.name()
    }
    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
    fn read(&mut self, pos:isize, buffer: Option<*mut ()>, size:usize) -> isize {
        let addr:u16 = (pos & 0xffff) as u16;
        let flags: u16 = ((pos >> 16) & 0xffff) as u16;
        return self.master_recv(addr, flags, buffer.map(|ptr| ptr as *mut u8), size);
    }
    fn write(&mut self, pos:isize, buffer: Option<*const ()>, size:usize) -> isize {
        let addr:u16 = (pos & 0xffff) as u16;
        let flags: u16 = ((pos >> 16) & 0xffff) as u16;
        return self.master_send(addr, flags, buffer.map(|ptr| ptr as *mut u8), size);
    }
    fn control(&mut self, cmd:usize, args: Option<*mut ()>) -> isize{
        match cmd.into() {
            I2cDevCtrl::BIT10 => {
                self.flags |= usize::from(I2cState::Addr10bit) as u16;
            },
            I2cDevCtrl::TIMEOUT => {
                // let num: &mut usize = (crate::wrapper::OptionMut(args)).into();
                let num: &mut usize = unsafe { &mut *(args.unwrap() as *mut usize)};
                self.timeout = *num;
            },

            I2cDevCtrl::RW => {
                let priv_data = unsafe { &mut *(args.unwrap() as *mut I2cPrivData)};
                let msg =  priv_data.msgs();
                let mut msgs:Vec<&I2cMsg> = Vec::new();
                msgs.push(msg);
                self.transfer(msgs);
            },
            _ => {
                self.ops().i2c_bus_control(cmd, args);
            },
        }
        0
    }
}


