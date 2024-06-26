use crate::object::{ObjectInformation,ObjectClassType,ObjectInfoType};
use crate::{Error, To};
use crate::scheduler::Scheduler;
use crate::bsp::BspTrait;
use crate::thread::Thread;
use crate::tick::Tick;
use crate::timer::Timer;
use crate::irq::Interrupt;
use crate::kservice;
use crate::components;
use crate::libcpu::LibcpuTrait;
use crate::mem::SmallMem;
use crate::List;
use crate::Box;
use crate::Vec;
use crate::String;
use crate::BTreeMap;

use crate::DeviceOps;

static mut SYSTREM: Option<System> = None;
fn main_fun(_parameter:*mut ()) -> Result<(),Error>{
    components::init();
    extern {#[allow(improper_ctypes)]fn main() -> Result<(),Error>;}
    unsafe{main()}
}

const MAIN_THREAD_STACK_SIZE: usize = 10240;
static mut MAIN_THREAD_STACK: [u8; MAIN_THREAD_STACK_SIZE] = [0; MAIN_THREAD_STACK_SIZE];

pub struct System{
    scheduler:Option<Scheduler>,
    tick:Tick,
    timer_list:List<Timer>,
    pub(super) object_container:[ObjectInformation; ObjectInfoType::Unknown as usize],
    interrupt:Interrupt,
    pub libcpu: Option<Box<dyn LibcpuTrait>>,
    pub bsp: Option<Box<dyn BspTrait>>,
    pub device_list:BTreeMap<String,Box<dyn DeviceOps>>,
    pub idle_hook_list:Vec<fn()>,
    heap: Option<&'static mut SmallMem>,
    console_device: Option<String>,
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
            bsp:None,
            device_list:BTreeMap::new(),
            idle_hook_list:Vec::new(),
            heap:None,
            console_device:None,
        };
        systerm
    }
    fn main_app_init(&mut self) {
        let stack_size:u32 = core::mem::size_of::<[u8; MAIN_THREAD_STACK_SIZE]>().try_into().unwrap();
        let stack_start = unsafe {MAIN_THREAD_STACK.as_mut_ptr() as *mut ()};
        let main_thread = Thread::init("main", main_fun, core::ptr::null_mut(),
                                                    stack_start, stack_size, 20, 32);

        main_thread.startup();
    }
    fn init(&mut self)  {
        self.object_container_init();
        self.bsp().unwrap().init();
        self.set_console("uart1".into());
        components::board_init();
        kservice::show_version();
        self.scheduler_init();
        self.main_app_init();
        crate::idle::rt_thread_idle_init();
    }

    fn object_container_init(&mut self) {
        self.object_container[ObjectInfoType::Thread as usize].init(ObjectClassType::Thread,core::mem::size_of::<Thread>().try_into().unwrap());
        self.object_container[ObjectInfoType::Device as usize].init(ObjectClassType::Device,core::mem::size_of::<Thread>().try_into().unwrap());
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
    pub fn set_heap(&mut self, heap:&'static mut SmallMem){
        self.heap = Some(heap);
    }
    pub fn heap(&'static mut self) -> &'static mut SmallMem{
        self.heap.as_mut().unwrap()
    }
    pub fn set_console(&mut self,console:String){
        self.console_device = Some(console);
    }
    pub fn console_device(&mut self) -> Option<&mut Box<dyn DeviceOps>>{
        if let Some(console_device_name) = self.console_device.clone(){
            return self.device_list_mut().get_mut(&console_device_name);
        }
        None
    }
    pub fn putc(&mut self,  c: char){
        if let Some(console) = self.console_device(){
            console.write(0, (&c).to_const(), 1);
        }
    }
    pub fn puts(&mut self,  s: &str){
        if let Some(console) = self.console_device(){
            console.write(0, s.to_const(), s.len());
        }
    }
    pub fn getc(&mut self) -> u8{
        let mut c:char = ' ';
        if let Some(console) = self.console_device(){
            console.read(0, (&mut c).to_mut(), 1);
        }
        return c as u8;
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
