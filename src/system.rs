use crate::object::{ObjectInformation,ObjectClassType,ObjectInfoType};
use crate::{println, Error};
use crate::scheduler::Scheduler;
use crate::hw::HardWare;
use crate::thread::Thread;
use crate::tick::Tick;
use crate::list::List;
use crate::timer::Timer;
use crate::irq::Interrupt;
use crate::kservice;
use crate::components;
use crate::libcpu::LibcpuTrait;
use crate::mem::SmallMem;

static mut SYSTREM: Option<System> = None;
fn main_fun(_parameter:*mut ()) -> Result<(),Error>{
    components::init();
    extern {#[allow(improper_ctypes)]fn main() -> Result<(),Error>;}
    unsafe{main()}
}

const MAIN_THREAD_STACK_SIZE: usize = 10240;
static mut MAIN_THREAD_STACK: [u8; MAIN_THREAD_STACK_SIZE] = [0; MAIN_THREAD_STACK_SIZE];
static mut MAIN_THREAD: Option<Thread> = None;

pub struct System{
    scheduler:Option<Scheduler>,
    tick:Tick,
    timer_list:List<Timer>,
    pub(super) object_container:[ObjectInformation; ObjectInfoType::Unknown as usize],
    interrupt:Interrupt,
    pub libcpu: Option<*mut dyn LibcpuTrait>,
    heap: Option<*mut SmallMem>,
}

impl System {
    pub fn global_mut() -> &'static mut Self{
        unsafe {
            if SYSTREM.is_none(){
                SYSTREM = Some(Self::new());
            }
            SYSTREM.as_mut().unwrap()
        }
    }

    fn new() -> Self {
        let systerm = Self{
            scheduler:None,
            tick:Tick::new(),
            timer_list:List::new(),
            object_container:[ObjectInformation::new(); ObjectInfoType::Unknown as usize],
            interrupt:Interrupt::init(),
            libcpu:None,
            heap:None,
        };
        systerm
    }
    fn main_app_init(&mut self) {
        let stack_size:u32 = core::mem::size_of::<[u8; MAIN_THREAD_STACK_SIZE]>().try_into().unwrap();
        let stack_start = unsafe {MAIN_THREAD_STACK.as_mut_ptr() as *mut ()};
        let thread_static = unsafe {&mut *core::ptr::addr_of_mut!(MAIN_THREAD)};
        let main_thread = Thread::init(thread_static,"main", main_fun, core::ptr::null_mut(),
                                                    stack_start, stack_size, 20, 32);

        println!("{}",main_thread.parent);
        main_thread.startup();
    }
    fn init(&mut self)  {
        self.object_container_init();
        HardWare::board_init();
        components::board_init();
        kservice::show_version();
        self.timer_init();
        self.scheduler_init();
        self.main_app_init();
        crate::idle::rt_thread_idle_init();
    }

    fn object_container_init(&mut self) {
        self.object_container[ObjectInfoType::Thread as usize].init(ObjectClassType::Thread,core::mem::size_of::<Thread>().try_into().unwrap());
        self.object_container[ObjectInfoType::Device as usize].init(ObjectClassType::Device,core::mem::size_of::<Thread>().try_into().unwrap());
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
    pub fn interrupt_mut(&mut self) ->&mut Interrupt {
        &mut self.interrupt
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
    pub fn set_heap(&mut self, heap:*mut SmallMem){
        self.heap = Some(heap);
    }
    pub fn heap(&mut self) -> &mut SmallMem{
        unsafe {&mut *self.heap.unwrap()}
    }
}

#[macro_export]
macro_rules! system {
    () => {{
        $crate::system::System::global_mut()
    }};
    ($($tokens:tt)*) => {{
        $crate::system::System::global_mut().$($tokens)*
    }};
}

#[macro_export]
macro_rules! scheduler {
    () => {{
        $crate::system::System::global_mut().scheduler_mut();
    }};
    ($($tokens:tt)*) => {{
        $crate::system::System::global_mut().scheduler_mut().$($tokens)*
    }};
}

#[macro_export]
macro_rules! schedule {
    ()=>{$crate::system::System::global_mut().scheduler_mut().schedule()};
}


#[macro_export]
macro_rules! tick {
    ($($tokens:tt)*) => {{
        $crate::system::System::global_mut().tick_mut().$($tokens)*
    }};
}

#[macro_export]
macro_rules! interrupt_enter {
    () => {{$crate::system::System::global_mut().interrupt_mut().enter()}};
}

#[macro_export]
macro_rules! interrupt_leave {
    () => {{$crate::system::System::global_mut().interrupt_mut().leave()}};
}

#[macro_export]
macro_rules! interrupt_nest {
    () => {{$crate::system::System::global_mut().interrupt_mut().nest()}};
}
