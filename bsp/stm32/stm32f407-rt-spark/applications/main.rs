use crate::{*};

#[no_mangle]
fn main() {
    let timer_static = unsafe {&mut TEST_TIMER};
    
    let _timer = Timer::init(timer_static, timer_timeout, core::ptr::null_mut(), 0, 0);
    _timer.start();

    let stack_size:u32 = core::mem::size_of::<[u8; TEST_THREAD_STACK_SIZE]>().try_into().unwrap();
    let stack_start = unsafe {TEST_THREAD_STACK.as_mut_ptr() as *mut ()};
    let thread_static = unsafe {&mut TEST_THREAD};
    let test_thread = thread::Thread::init(thread_static,test, core::ptr::null_mut(),
                                                stack_start, stack_size, 20, 4);
    test_thread.startup();

    let ahb1enr_ptr: *mut u32 = AHB1ENR as *mut u32;
    unsafe {
        let ahb1enr = &mut *ahb1enr_ptr;
        *ahb1enr |= 1 << 5;
    }

    let gpiof_base_ptr: *mut GPIOTypeDef = GPIOF_BASE as *mut GPIOTypeDef;
    let gpiof_base = unsafe { &mut *gpiof_base_ptr};
    sys_gpio_set(gpiof_base, 1 << 11,1, 0, 1, 1);
    sys_gpio_set(gpiof_base, 1 << 12,1, 0, 1, 1);
    sys_gpio_pin_set(gpiof_base, 1 << 11,false);
    sys_gpio_pin_set(gpiof_base, 1 << 12,true);
    let mut led_num = 0;
    loop {
        led_num += 1;
        if led_num % 100000 == 0{
            gpiof_base.odr ^= 1 << 12;
            led_num = 0
        }
    }
}
