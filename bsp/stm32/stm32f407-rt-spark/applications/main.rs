use crate::{*};
use components::pin::{*};

#[no_mangle]
fn main() {
    let timer_static = unsafe {&mut TEST_TIMER};
    
    let _timer = Timer::init(timer_static, timer_timeout, core::ptr::null_mut(), 0, 0);
    _timer.start();

    let stack_size:u32 = core::mem::size_of::<[u8; TEST_THREAD_STACK_SIZE]>().try_into().unwrap();
    let stack_start = unsafe {TEST_THREAD_STACK.as_mut_ptr() as *mut ()};
    let thread_static = unsafe {&mut TEST_THREAD};
    let test_thread = thread::Thread::init(thread_static, "test", test, core::ptr::null_mut(),
                                                stack_start, stack_size, 20, 4);
    test_thread.startup();

    let led_red = pin_get("PF.12");
    pin_mode(led_red,0);
    loop {
        pin_write(led_red, true);
        kernel::thread_sleep!(500);
        pin_write(led_red, false);
        kernel::thread_sleep!(500);
    }
}

