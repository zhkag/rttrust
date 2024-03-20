use crate::Vec;
use crate::system::System;
use crate::thread::Thread;
use crate::Error;

const IDLE_THREAD_STACK_SIZE: usize = 10240;
static mut IDLE_THREAD_STACK: [u8; IDLE_THREAD_STACK_SIZE] = [0; IDLE_THREAD_STACK_SIZE];
static mut IDLE_THREAD:Option<Thread> = None;

fn idle_fun(_parameter: *mut ()) -> Result<(),Error>{
    loop {
        // for hook in crate::system!(idle_hook_list_mut()).iter() {
        //     hook()
        // }
    }
}

pub fn rt_thread_idle_init(){
    let stack_size:u32 = core::mem::size_of::<[u8; IDLE_THREAD_STACK_SIZE]>().try_into().unwrap();
    let stack_start = unsafe {IDLE_THREAD_STACK.as_mut_ptr() as *mut ()};
    let thread_static = unsafe {&mut IDLE_THREAD};
    let idle_thread = Thread::init(thread_static,"idle", idle_fun, core::ptr::null_mut(),
                                                stack_start, stack_size, 31, 32);
    idle_thread.startup();
}

impl System<'_> {
    #[allow(dead_code)]
    fn idle_hook_list_mut(&mut self) -> &mut Vec<fn()>{
        &mut self.idle_hook_list
    }
    pub fn idle_sethook(&mut self, hook:fn()){
        self.idle_hook_list.push(hook);
    }
    pub fn idle_delhook(&mut self, hook:fn()){
        self.idle_hook_list.retain(|&x| x != hook);
    }
}
