use crate::thread::Thread;

const IDLE_THREAD_STACK_SIZE: usize = 10240;
static mut IDLE_THREAD_STACK: [u8; IDLE_THREAD_STACK_SIZE] = [0; IDLE_THREAD_STACK_SIZE];
static mut IDLE_THREAD:Option<Thread> = None;

fn idle_fun(_parameter: *mut ()) {
    loop {}
}

pub fn rt_thread_idle_init(){
    let stack_size:u32 = core::mem::size_of::<[u8; IDLE_THREAD_STACK_SIZE]>().try_into().unwrap();
    let stack_start = unsafe {IDLE_THREAD_STACK.as_mut_ptr() as *mut ()};
    let thread_static = unsafe {&mut IDLE_THREAD};
    let idle_thread = Thread::init(thread_static,"idle", idle_fun, core::ptr::null_mut(),
                                                stack_start, stack_size, 31, 32);
    idle_thread.startup();
}
