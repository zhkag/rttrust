#![no_main]
#![no_std]
#[allow(unused_imports)]
use components;
use kernel::system;
#[allow(unused_imports)]
use libcpu;

mod board;
mod applications;
mod drivers;

use kernel::thread;
use kernel::timer;
use kernel::Error;

use kernel::drivers::core::device::DeviceSelf;
use components::pin::{*};
use kernel::To;

const TEST_THREAD_STACK_SIZE: usize = 10240;
static mut TEST_THREAD_STACK: [u8; TEST_THREAD_STACK_SIZE] = [0; TEST_THREAD_STACK_SIZE];
static mut TEST_THREAD: Option<thread::Thread> = None;

fn test(_parameter:*mut ()) -> Result<(),Error>{
    let mut led_yellow = 0;
    let mut pin_opt = system!(device_list_mut()).get_mut("pin");
    if let Some(ref mut pin) = pin_opt {
        if let DeviceSelf::Pin(pin) = pin.device_self().unwrap() {
            led_yellow = pin.ops().pin_get("PF.11");
        }
        let mut mode = DevicePinMode::init(led_yellow, 0);
        pin.control(0, mode.to_mut());
    }

    let mut value = DevicePinValue::init(led_yellow, true);
    loop {
        if let Some(ref mut pin) = pin_opt {
            value.set_value(true);
            pin.write(0,value.to_const() ,core::mem::size_of::<DevicePinValue>());
            kernel::thread_sleep!(100)?;

            value.set_value(false);
            pin.write(0,value.to_const() ,core::mem::size_of::<DevicePinValue>());
            kernel::thread_sleep!(100)?;
        }
    }
}

use crate::timer::Timer;

static mut TEST_TIMER: Option<Timer> = None;
fn timer_timeout(_parameter:*mut ()) {

}
