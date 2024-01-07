#![no_main]
#![no_std]
use kernel::system;
use kernel::scheduler;
use kernel::idle;
mod startup;
use kernel::hw;
use kernel::thread;
use kernel::list;
use kernel::tick;
use kernel::timer;
use kernel::kservice;
use kernel::println;

const TEST_THREAD_STACK_SIZE: usize = 1024;
static mut TEST_THREAD_STACK: [u8; TEST_THREAD_STACK_SIZE] = [0; TEST_THREAD_STACK_SIZE];
static mut TEST_THREAD: Option<thread::Thread> = None;

const PERIPH_BASE: u32 = 0x40000000;
const AHB1PERIPH_BASE: u32 = PERIPH_BASE + 0x00020000;
const RCC_BASE: u32 = AHB1PERIPH_BASE + 0x3800;
const AHB1ENR: u32 = RCC_BASE + 0x30;
const GPIOF_BASE: u32 = AHB1PERIPH_BASE + 0x1400;

#[repr(C)]
struct GPIOTypeDef
{
    moder:u32,
    otyper:u32,
    ospeedr:u32,
    pupdr:u32,
    idr:u32,
    odr:u32,
    bsrr:u32,
    lckr:u32,
    afr:[u32;2],
}

fn sys_gpio_set(p_gpiox: &mut GPIOTypeDef, pinx:u16, mode:u32, otype:u32, ospeed:u32, pupd:u32) {
    let mut pos:u32;
    let mut curpin:u32;
    for pinpos in 0..16 {
        pos = 1 << pinpos;
        curpin = (pinx as u32) & pos;
        if curpin == pos {
            p_gpiox.moder &= !(3 << (pinpos * 2));
            p_gpiox.moder |= mode << (pinpos * 2);
            if (mode == 1) || (mode == 2) {
                p_gpiox.ospeedr &= !(3 << (pinpos * 2));
                p_gpiox.ospeedr |= ospeed << (pinpos * 2);
                p_gpiox.otyper &= !(1 << pinpos);
                p_gpiox.otyper |= otype << pinpos;
            }
            p_gpiox.pupdr &= !(3 << (pinpos * 2));
            p_gpiox.pupdr |= pupd << (pinpos * 2);
        }
    }
}

fn sys_gpio_pin_set(p_gpiox: &mut GPIOTypeDef, pinx:u32, status:bool)
{
    if status {
        p_gpiox.bsrr |= pinx;
    }
    else {
        p_gpiox.bsrr |= pinx << 16;
    }
}

fn test(_parameter:*mut ()) {
    #[cfg(feature = "bsptest")]
    println!("test thread");
    let mut _tick = tick!(get());
    let gpiof_base_ptr: *mut GPIOTypeDef = GPIOF_BASE as *mut GPIOTypeDef;
    let gpiof_base = unsafe { &mut *gpiof_base_ptr};
    let mut led_num = 0;
    loop {
        led_num += 1;
        if led_num % 100000 == 0{
            gpiof_base.odr ^= 1 << 11;
            led_num = 0
        }
    }
}

use crate::timer::Timer;

static mut TEST_TIMER: Option<Timer> = None;
fn timer_timeout(parameter:*mut ()) {
    let mut led_num = 0;
}


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
