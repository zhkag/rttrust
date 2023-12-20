use crate::scheduler::Scheduler;
use crate::hw::HardWare;
use crate::Thread;
use crate::List;

static mut SYSTREM: Option<System> = None;

fn main_fun(_parameter:*mut ()) {

}

const MAIN_THREAD_STACK_SIZE: usize = 1024;
static mut MAIN_THREAD_STACK: [u8; MAIN_THREAD_STACK_SIZE] = [0; MAIN_THREAD_STACK_SIZE];
static mut MAIN_THREAD: Option<Thread> = None;


pub struct System{
    scheduler:Option<Scheduler>,
}

impl System {
    pub fn global_mut() -> &'static mut System{
        unsafe {
            if (&mut SYSTREM).is_none(){
                SYSTREM = Some(System::new());
            }
            return SYSTREM.as_mut().unwrap();
        }
    }

    fn new() -> System {
        let systerm = System{
            scheduler:None,
        };
        systerm
    }
    fn main_app_init(&mut self) {
        let size:u32 = core::mem::size_of::<[u8; MAIN_THREAD_STACK_SIZE]>().try_into().unwrap();
        let stack_start = unsafe {MAIN_THREAD_STACK.as_mut_ptr() as *mut ()};
        let thread = unsafe {&mut MAIN_THREAD};
        let main_thread = Thread::new(main_fun, core::ptr::null_mut(), stack_start, size, 20);
        *thread = Some(main_thread);
        thread.as_mut().unwrap().startup(); 
    }
    fn init(&mut self)  {
        HardWare::board_init();
        self.scheduler = Scheduler::new();
        self.main_app_init();
        // rt_system_timer_init();
        // rt_system_scheduler_init();
        crate::idle::rt_thread_idle_init();
    }
    pub fn scheduler_mut(&mut self) ->&mut Scheduler {
        self.scheduler.as_mut().unwrap()
    }

    pub fn scheduler(&mut self) ->Scheduler {
        self.scheduler.unwrap()
    }

    pub fn startup(&mut self) {
        self.init();

        self.scheduler().start();
        unreachable!();
    }
}

#[macro_export]
macro_rules! system {
    ($($tokens:tt)*) => {{
        crate::system::System::global_mut().$($tokens)*
    }};
}

#[macro_export]
macro_rules! scheduler {
    ($($tokens:tt)*) => {{
        crate::system::System::global_mut().scheduler_mut().$($tokens)*
    }};
}


#[macro_export]
macro_rules! schedule {
    ()=>{crate::system::System::global_mut().scheduler_mut().schedule()};
}

