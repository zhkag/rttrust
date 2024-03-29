#![no_main]
#![no_std]
#[allow(unused_imports)]
use components;
use kernel::println;
#[allow(unused_imports)]
use libcpu;

mod board;
mod applications;
mod drivers;

use kernel::thread;
use kernel::timer;
use kernel::Error;

use components::pin::{*};
use kernel::To;
use kernel::derive_find;

const TEST_THREAD_STACK_SIZE: usize = 10240;
static mut TEST_THREAD_STACK: [u8; TEST_THREAD_STACK_SIZE] = [0; TEST_THREAD_STACK_SIZE];

fn test(_parameter:*mut ()) -> Result<(),Error>{
    let mut led_yellow = 0;
    let mut pin_ops = derive_find!("pin");
    if let Some(ref mut pin) = pin_ops {
        if let Some(pin) = pin.as_any().downcast_mut::<DevicePin>() {
            led_yellow = pin.ops().pin_get("PF.11");
        }
        let mut mode = DevicePinMode::init(led_yellow, PinMode::OUTPUT);
        pin.control(0, mode.to_mut());
    }

    let mut value = DevicePinValue::init(led_yellow, PinState::HIGH);
    loop {
        if let Some(ref mut pin) = pin_ops {
            value.set_value(PinState::HIGH);
            pin.write(0,value.to_const() ,core::mem::size_of::<DevicePinValue>());
            kernel::thread_sleep!(100)?;

            value.set_value(PinState::LOW);
            pin.write(0,value.to_const() ,core::mem::size_of::<DevicePinValue>());
            kernel::thread_sleep!(100)?;
        }
        // println!("test thread!");
    }
}

use crate::timer::Timer;

static mut TEST_TIMER: Option<Timer> = None;
fn timer_timeout(_parameter:*mut ()) {

}
