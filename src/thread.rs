use crate::hw::HardWare;
use crate::object::Object;
use crate::timer::Timer;
use crate::{scheduler,schedule};
use crate::list::List;
use crate::thread_self;

// use core::ops::{BitAnd,BitOr,Not};

// #[derive(PartialEq)]
#[derive(Copy, Clone)]
pub enum Status {
    Init        = 0x00,     // Initialized status
    Close       = 0x01,     // Closed status
    Ready       = 0x02,     // Ready status
    Running     = 0x03,     // Running status
    SuspendMask = 0x04,
    StatMask    = 0x07,
    StatYield   = 0x08,     // indicate whether remaining_tick has been reloaded since last schedule

}

impl Status {
    const SUSPEND_INTERRUPTIBLE: Status = Status::SuspendMask;
    const SUSPEND: Status = Status::SUSPEND_INTERRUPTIBLE;
    pub const STAT_YIELD_MASK: Status = Status::StatYield;
}

// impl BitAnd for Status {
//     type Output = Self;
//     fn bitand(self, rhs: Self) -> Self::Output {
//         self
//     }
// }
// impl BitOr for Status {
//     type Output = Self;
//     fn bitor(self, rhs: Self) -> Self::Output {
//         self
//     }
// }
// impl Not for Status {
//     type Output = Self;
//     fn not(self) -> Self::Output {
//         self
//     }
// }

#[derive(PartialEq)]
#[derive(Copy, Clone)]
pub struct Thread
{
    parent:Object,
    sp: *mut (),
    entry:fn(*mut ()),
    parameter: *mut (),
    stack_addr: *mut (),
    stack_size:u32,
    list:List<Self>,
    number_mask:u32,
    current_priority:u8,
    init_priority:u8,
    stat: u8,
    init_tick:u8,
    remaining_tick:u8,
    thread_timer:Option<Timer>,
}

impl Thread {
    fn new(entry: fn(*mut ()), parameter:*mut (), stack_start:*mut (), 
           stack_size:u32, priority:u8, tick:u8) -> Self {
        let mut thread = Self {
            parent:Object::new(),
            entry,
            parameter,
            stack_addr:stack_start,
            stack_size,
            sp:core::ptr::null_mut(),
            list:List::new(),
            init_priority:priority,
            current_priority:priority,
            number_mask: 1 << priority,
            stat:Status::Init as u8,
            init_tick:tick,
            remaining_tick:tick,
            thread_timer:None,
        };
        let ptr = thread.stack_addr as u32;
        thread.sp = HardWare::stack_init(thread.entry, thread.parameter,
                             (ptr+thread.stack_size-16)as *mut (), Self::thread_exit);
        
        let timer_parameter = thread.as_mut_ptr() as *mut ();
        let _timer = Timer::init(&mut thread.thread_timer, Self::thread_timeout, timer_parameter, 0, 0);
        thread
    }
    fn as_mut_ptr(&mut self) -> *mut Self {
        self as *mut Self
    }

    fn thread_timeout(parameter:*mut ()){
        let thread = unsafe{&mut *(parameter as *mut Self)};
        
    }
    fn thread_exit()
    {
        if let Some(thread) = thread_self!() {
            thread.stat = Status::Init as u8;
        }
        schedule!();
    }
    pub fn init<'a>(thread: &'a mut Option<Self>, name:&'a str, entry: fn(*mut ()), parameter:*mut (),
                stack_start:*mut (), stack_size:u32, priority:u8, tick:u8) -> &'a mut Self{
        *thread = Some(Self::new(entry, parameter, stack_start, stack_size, priority, tick));
        let thread_mut = thread.as_mut().unwrap();
        thread_mut.parent.init(crate::object::ObjectClassType::Thread, name);
        thread_mut.list_mut().init();
        thread_mut
    }

    pub fn sp_mut(&mut self) ->&mut *mut (){
        &mut self.sp
    }

    pub fn current_priority(&self) ->u8 {
        self.current_priority
    }
    pub fn number_mask(&self) ->u32 {
        self.number_mask
    }
    
    fn resume(&mut self){
        if (self.stat & Status::SuspendMask as u8) != Status::SuspendMask as u8{
            return;
        }
        scheduler!(insert_thread(self));
    }

    pub fn startup(&mut self){
        self.stat = Status::SUSPEND as u8;
        self.resume();
        if thread_self!().is_some(){
            schedule!();
        }
    }

    pub fn stat(&self) -> u8 {
        self.stat
    }
    pub fn set_stat(&mut self, stat:u8) {
        self.stat = stat;
    }
    
    pub fn tick_decrease(&mut self) -> u8 {
        self.remaining_tick -= 1;
        if self.remaining_tick == 0 {
            self.remaining_tick = self.init_tick;
            return 0;
        }
        self.remaining_tick
    }

    pub fn list_mut(&mut self) -> &mut List<Self> {
        &mut self.list
    }

    pub fn list_to_thread(list: *mut List<Self>) -> &'static mut Self {
        #[allow(deref_nullptr)]
        unsafe { &mut *((list as usize - (&(&*(0 as *const Self)).list) as *const List<Self> as usize) as *mut Self) }
    }

}

#[macro_export]
macro_rules! thread_self {
    () => {{
        crate::scheduler!(current_thread())
    }};
}
