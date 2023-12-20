use crate::thread::Thread;

const IDLE_THREAD_STACK_SIZE: usize = 1024;
static mut IDLE_THREAD_STACK: [u8; IDLE_THREAD_STACK_SIZE] = [0; IDLE_THREAD_STACK_SIZE];

fn idle_fun(_parameter: *mut ()) {

}

static mut IDLE_THREAD:Option<Thread> = None;

pub fn rt_thread_idle_init(){
    let size:u32 = core::mem::size_of::<[u8; IDLE_THREAD_STACK_SIZE]>().try_into().unwrap();
    let stack_start = unsafe {IDLE_THREAD_STACK.as_mut_ptr() as *mut ()};
    let thread = unsafe {&mut IDLE_THREAD};
    let idle_thread = Thread::new(idle_fun, core::ptr::null_mut(), stack_start, size, 31);
    *thread = Some(idle_thread);
    thread.unwrap().startup();
}
