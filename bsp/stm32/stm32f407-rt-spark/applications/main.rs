use crate::{*};
use components::pin::*;
use kernel::{println, system};
use kernel::{drivers::i2c::{core::I2cMsg, dev::DeviceI2cBus}, Vec};
#[export_name = "HardFault_Handler"]
unsafe extern "C" fn hard_fault_handler() {

}





fn write_reg(device:&mut DeviceI2cBus, mut buffer:[u8;2]) {
    let msg: I2cMsg = I2cMsg::init(0x1e, 0x0000, 2, Some(buffer.as_mut_ptr()));
    let mut msgs:Vec<&I2cMsg> = Vec::new();
    msgs.push(&msg);
    device.transfer(msgs);
}

fn read_regs(device:&mut DeviceI2cBus, reg:u8, len:usize, buffer:&mut u8) {
    let msg1: I2cMsg = I2cMsg::init(0x1e, 0, 1, Some([reg].as_mut_ptr()));

    let msg2: I2cMsg = I2cMsg::init(0x1e, 1, len, Some(buffer));
    let mut msgs:Vec<&I2cMsg> = Vec::new();
    msgs.push(&msg1);
    msgs.push(&msg2);
    device.transfer(msgs);
}
//0x0E 1
fn read_low_and_high(device:&mut DeviceI2cBus,reg:u8,len:usize) -> u32{
    let mut buf:u8 = 0;
    let mut data:u32;
    read_regs(device, reg, len, &mut buf);
    data = buf as u32;
    read_regs(device, reg, len, &mut buf);
    data = data + (buf as u32) << len * 8;
    data
}


#[no_mangle]
fn main() -> Result<(),Error>{
    let timer_static = unsafe {&mut TEST_TIMER};

    let _timer = Timer::init(timer_static, timer_timeout, core::ptr::null_mut(), 0, 0);
    _timer.start();

    let stack_size:u32 = core::mem::size_of::<[u8; TEST_THREAD_STACK_SIZE]>().try_into().unwrap();
    let stack_start = unsafe {TEST_THREAD_STACK.as_mut_ptr() as *mut ()};
    let test_thread = thread::Thread::init( "test", test, core::ptr::null_mut(),
                                                stack_start, stack_size, 20, 4);
    test_thread.startup();

    let led_red = pin_get("PF.12");
    pin_mode(led_red,PinMode::OUTPUT);

    loop {
        pin_write(led_red, PinState::HIGH);
        kernel::thread_sleep!(500)?;
        pin_write(led_red, PinState::LOW);
        kernel::thread_sleep!(500)?;
        println!("main thread!");

        // if let Some(device) = crate::derive_find!("i2c2").unwrap().as_any().downcast_mut::<DeviceI2cBus>() {
        //     write_reg(device, [0,4]);
        //     kernel::thread_sleep!(500)?;
        //     write_reg(device, [0,3]);
        //     kernel::thread_sleep!(500)?;

        //     let read_data = read_low_and_high(device, 0x0e,1);
        //     let proximity:u16 = ((read_data & 0x000f) + (((read_data >> 8) & 0x3f) << 4)) as u16;
        //     println!("proximity {}",proximity);

        //     let read_data = read_low_and_high(device, 0x0c,1);
        //     println!("read_data {}",read_data);
        // }
    }
}

