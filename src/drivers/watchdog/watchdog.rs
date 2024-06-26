use crate::drivers::core::device::{Device, DeviceOps, DeviceClassType};
use crate::drivers::DeviceRegister;
use crate::Box;
use crate::system;

#[repr(C)]
pub enum DeviceWatchDogCTRL
{
    Start = 0,
    Stop,
    GetTimeout,
    SetTimeout,
    Unknown,
}

impl From<usize> for DeviceWatchDogCTRL {
    fn from(value: usize) -> DeviceWatchDogCTRL {
        match value {
            0 => DeviceWatchDogCTRL::Start,
            1 => DeviceWatchDogCTRL::Stop,
            2 => DeviceWatchDogCTRL::GetTimeout,
            3 => DeviceWatchDogCTRL::SetTimeout,
            _ => DeviceWatchDogCTRL::Unknown,
        }
    }
}

pub trait WatchDogOps
{
    fn init(&mut self);
    fn control(&mut self,  cmd: DeviceWatchDogCTRL, args: Option<*mut ()>);
}

#[repr(C)]
pub struct DeviceWatchDog
{
    parent:Device,
    pub ops: Option<Box<dyn WatchDogOps>>,
}

impl DeviceWatchDog {
    pub fn new() -> Self   {
        DeviceWatchDog{
            parent:Device::new(),
            ops: None,
        }
    }
    pub fn ops(&mut self) -> &mut Box<dyn WatchDogOps>{
        self.ops.as_mut().unwrap()
    }
}

impl<T: WatchDogOps + 'static> DeviceRegister<T> for DeviceWatchDog {
    fn register(&mut self, name:&str, ops:T)
    {
        let mut hw_wdt = Some(DeviceWatchDog::new());
        let hw_wdt_mut = hw_wdt.as_mut().unwrap();
        hw_wdt_mut.ops = Some(Box::new(ops));
        hw_wdt_mut.parent.init(name, DeviceClassType::WDT);
        system!(device_register(hw_wdt.unwrap()));
    }
}
use crate::Any;

impl DeviceOps for DeviceWatchDog {
    fn name(&self) -> &str {
        self.parent.name()
    }
    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
    fn init(&mut self) -> isize{
        self.ops().init();
        0 as isize
    }
    fn open(&mut self, _oflag:u16) -> isize{
        self.ops().control(DeviceWatchDogCTRL::Start, None);
        0 as isize
    }
    fn close(&mut self) -> isize{
        self.ops().control(DeviceWatchDogCTRL::Stop, None);
        0 as isize
    }
    fn control(&mut self, cmd:usize, args: Option<*mut ()>) -> isize{
        match cmd.into() {
            DeviceWatchDogCTRL::Start | DeviceWatchDogCTRL::Stop => {
                self.ops().control(cmd.into(), None);
            },
            DeviceWatchDogCTRL::SetTimeout | DeviceWatchDogCTRL::GetTimeout => {
                self.ops().control(cmd.into(), args);
            },
            _=>{}
        }
        0
    }
}
