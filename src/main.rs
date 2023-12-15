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


use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

fn main_fun(_parameter:*mut ()) {
    loop {}
}

const MAIN_THREAD_STACK_SIZE: usize = 1024;
static mut MAIN_THREAD_STACK: [u8; MAIN_THREAD_STACK_SIZE] = [0; MAIN_THREAD_STACK_SIZE];


#[no_mangle]
fn entry() {

    // let num: u32 = 0;
    // let _lowest_bit = num.trailing_zeros();

    let size:u32 = core::mem::size_of::<[u8; MAIN_THREAD_STACK_SIZE]>().try_into().unwrap();
    let main_thread = 
        thread::Thread::new(main_fun, core::ptr::null_mut(), unsafe {MAIN_THREAD_STACK.as_mut_ptr() as *mut ()}, size);
    
    unsafe{context::rt_hw_context_switch_to(&mut main_thread.sp() as *mut *mut () as *mut ());};
    system!(startup());
    unreachable!();
}

