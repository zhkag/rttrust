use crate::scheduler::Scheduler;
use crate::hw::HardWare;
use crate::thread::Thread;
use crate::tick::Tick;
use crate::list::List;
use crate::timer::Timer;

static mut SYSTREM: Option<System> = None;

fn main_fun(_parameter:*mut ()) {
    unsafe{core::arch::asm!("bl main");}
}

const MAIN_THREAD_STACK_SIZE: usize = 1024;
static mut MAIN_THREAD_STACK: [u8; MAIN_THREAD_STACK_SIZE] = [0; MAIN_THREAD_STACK_SIZE];
static mut MAIN_THREAD: Option<Thread> = None;

pub struct System{
    scheduler:Option<Scheduler>,
    tick:Tick,
    timer_list:List<Timer>,
}

impl System {
    pub fn global_mut() -> &'static mut Self{
        unsafe {
            if (&mut SYSTREM).is_none(){
                SYSTREM = Some(Self::new());
            }
            return SYSTREM.as_mut().unwrap();
        }
    }

    fn new() -> Self {
        let systerm = Self{
            scheduler:None,
            tick:Tick::new(),
            timer_list:List::new(),
        };
        systerm
    }
    fn main_app_init(&mut self) {
        let stack_size:u32 = core::mem::size_of::<[u8; MAIN_THREAD_STACK_SIZE]>().try_into().unwrap();
        let stack_start = unsafe {MAIN_THREAD_STACK.as_mut_ptr() as *mut ()};
        let thread_static = unsafe {&mut MAIN_THREAD};
        let main_thread = Thread::init(thread_static,main_fun, core::ptr::null_mut(),
                                                    stack_start, stack_size, 20, 32);
        main_thread.startup();
    }
    fn init(&mut self)  {
        HardWare::board_init();
        self.timer_init();
        self.scheduler_init();
        self.main_app_init();
        crate::idle::rt_thread_idle_init();
    }
    
    fn timer_init(&mut self) {
        self.timer_list.init();
    }

    fn scheduler_init(&mut self) {
        self.scheduler = Some(Scheduler::new());
        self.scheduler_mut().init();
    }

    pub fn scheduler_mut(&mut self) ->&mut Scheduler {
        self.scheduler.as_mut().unwrap()
    }

    pub fn tick_mut(&mut self) ->&mut Tick {
        &mut self.tick
    }

    pub fn timer_list_mut(&mut self) ->&mut List<Timer>{
        &mut self.timer_list
    }

    pub fn startup(&mut self) {
        self.init();
        self.scheduler_mut().start();
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


#[macro_export]
macro_rules! tick {
    ($($tokens:tt)*) => {{
        crate::system::System::global_mut().tick_mut().$($tokens)*
    }};
}