#![no_main]
#![no_std]
mod system;
mod scheduler;
mod idle;
mod global_asm;
mod context;
mod cpuport;
mod hw;
mod thread;
mod list;
mod tick;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}


const TEST_THREAD_STACK_SIZE: usize = 1024;
static mut TEST_THREAD_STACK: [u8; TEST_THREAD_STACK_SIZE] = [0; TEST_THREAD_STACK_SIZE];
static mut TEST_THREAD: Option<thread::Thread> = None;


fn test(_parameter:*mut ()) {
    let mut tick:usize = 0;
    loop {
        tick = tick!(get());
    }
}


#[no_mangle]
fn main() {
    let stack_size:u32 = core::mem::size_of::<[u8; TEST_THREAD_STACK_SIZE]>().try_into().unwrap();
    let stack_start = unsafe {TEST_THREAD_STACK.as_mut_ptr() as *mut ()};
    let thread_static = unsafe {&mut TEST_THREAD};
    let test_thread = thread::Thread::init(thread_static,test, core::ptr::null_mut(),
                                                stack_start, stack_size, 20, 32);
    test_thread.startup();
    let mut tick:usize = 0;
    loop {
        tick = tick!(get());
    }
}

#[no_mangle]
fn entry() {
    system!(startup());
    unreachable!();
}

