use kernel::drivers::watchdog::watchdog::DeviceWatchDog;
use components::drivers::DeviceRegister;
use kernel::drivers::watchdog::watchdog::WatchDogOps;
use kernel::drivers::watchdog::watchdog::DeviceWatchDogCTRL;

use crate::board::board::APB1PERIPH_BASE;
const IWDG_BASE: u32 = APB1PERIPH_BASE + 0x3000;

#[repr(C)]
pub struct IWDGTypeDef
{
    kr:u32,
    pr:u32,
    rlr:u32,
    sr:u32,
}

impl IWDGTypeDef{
    pub fn new<'a>() -> &'a mut Self{
        unsafe {&mut *(IWDG_BASE as *mut IWDGTypeDef)}
    }
}

struct StmWdg<'a>{
    wdt:&'a mut IWDGTypeDef,
}

impl WatchDogOps for StmWdg<'_> {
    fn init(&mut self){}
    fn control(&mut self,  cmd: DeviceWatchDogCTRL, args: Option<*mut ()>){
        match cmd {
            DeviceWatchDogCTRL::Start => self.wdt.kr = 0xCCCC,
            DeviceWatchDogCTRL::SetTimeout => {
                let num = unsafe { &mut *(args.unwrap() as *mut u32)};
                self.wdt.kr = 0x5555;
                self.wdt.pr = 4 as u32;
                self.wdt.rlr = *num * 125;
                self.wdt.kr = 0xAAAA;
            },
            DeviceWatchDogCTRL::GetTimeout => {
                let num = unsafe { &mut *(args.unwrap() as *mut u32)};
                *num = self.wdt.rlr / 125
            },
            _ => {},
        }
    }
}

use kernel::macros::init_export;
#[init_export("2")]
fn device_pin() {
    let stm_wdg = StmWdg{wdt:IWDGTypeDef::new()};
    DeviceWatchDog::new().register("wdt",stm_wdg);
}

