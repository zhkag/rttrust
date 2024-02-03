#![no_main]
#![no_std]
#[allow(unused_imports)]
use components;
#[allow(unused_imports)]
use libcpu;

mod board;
mod applications;
mod drivers;

use kernel::thread;
use kernel::timer;
use kernel::Error;

use components::DeviceOps;
use components::pin::{*};

const TEST_THREAD_STACK_SIZE: usize = 10240;
static mut TEST_THREAD_STACK: [u8; TEST_THREAD_STACK_SIZE] = [0; TEST_THREAD_STACK_SIZE];
static mut TEST_THREAD: Option<thread::Thread> = None;

fn test(_parameter:*mut ()) -> Result<(),Error>{
    let pin = DevicePin::find("pin").unwrap();
    let led_yellow = pin.ops().pin_get("PF.11");
    let mut mode = DevicePinMode::init(led_yellow, 0);
    let mut value = DevicePinValue::init(led_yellow, true);
    pin.control(0, mode.r#mut());
    loop {
        value.set_value(true);
        pin.write(0,value.r#const() ,core::mem::size_of::<DevicePinValue>());
        kernel::thread_sleep!(100)?;
        value.set_value(false);
        pin.write(0,value.r#const() ,core::mem::size_of::<DevicePinValue>());
        kernel::thread_sleep!(100)?;
    }
}

use crate::timer::Timer;

static mut TEST_TIMER: Option<Timer> = None;
fn timer_timeout(_parameter:*mut ()) {

}
