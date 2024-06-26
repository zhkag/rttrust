use crate::println;
use crate::object::Object;
use crate::timer::Timer;
use crate::{scheduler, schedule};
use crate::{thread_self_mut, thread_self, Error};
use crate::system;

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

/*
 * for rt_thread_suspend_with_flag()
 */

#[repr(C)]
#[allow(dead_code)]
enum SuspendWithFlag
{
    INTERRUPTIBLE = 0,
    KILLABLE,
    UNINTERRUPTIBLE,
}

#[repr(C)]
#[derive(PartialEq)]
#[derive(Copy, Clone)]
pub struct Thread
{
    sp: *mut (),
    pub(super) parent:Object,
    entry:fn(*mut ())-> Result<(),Error>,
    parameter: *mut (),
    stack_addr: *mut (),
    stack_size:u32,
    error:Error,
    number_mask:u32,
    current_priority:u8,
    init_priority:u8,
    stat: u8,
    init_tick:u8,
    remaining_tick:u8,
    thread_timer:Option<Timer>,
    timer_run:bool,
}

impl core::fmt::Display for Thread {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.parent)
    }
}

impl Thread {
    fn new(entry: fn(*mut ()) -> Result<(),Error>, parameter:*mut (), stack_start:*mut (),
           stack_size:u32, priority:u8, tick:u8) -> Self {
        let thread = Self {
            parent:Object::new(),
            entry,
            parameter,
            stack_addr:stack_start,
            stack_size,
            sp:core::ptr::null_mut(),
            error:Error::Ok,
            init_priority:priority,
            current_priority:priority,
            number_mask: 1 << priority,
            stat:Status::Init as u8,
            init_tick:tick,
            remaining_tick:tick,
            thread_timer:None,
            timer_run:false,
        };
        thread
    }
    pub fn timer_run(&self) -> bool{
        self.timer_run
    }

    pub fn timeout_tick(&self) -> usize{
        self.thread_timer.as_ref().unwrap().timeout_tick()
    }

    pub fn thread_timer_mut(&mut self) -> &mut Timer{
        self.thread_timer.as_mut().unwrap()
    }

    fn as_mut_ptr(&mut self) -> *mut Self {
        self as *mut Self
    }

    fn thread_timeout(parameter:*mut ()){
        let thread = unsafe{&mut *(parameter as *mut Thread)};
        let libcpu = system!().libcpu();
        let level = libcpu.interrupt_disable();
        thread.error = Error::TimeOut;
        libcpu.interrupt_enable(level);
        thread.timer_run = false;
        scheduler!(insert_thread(*thread));
        schedule!();
    }

    fn thread_exit(err:Result<(),Error>)
    {
        match err {
            Err(error) => {println!("\x1b[31m Thread Error  {}\x1b[0m",error)},
            Ok(()) => {}
        }
        if let Some(thread) = thread_self_mut!() {
            thread.stat = Status::Init as u8;
        }
        schedule!();
    }
    pub fn init<'a>(name:&'a str, entry: fn(*mut ()) -> Result<(),Error>, parameter:*mut (),
                stack_start:*mut (), stack_size:u32, priority:u8, tick:u8) -> Self{
        let mut thread = Self::new(entry, parameter, stack_start, stack_size, priority, tick);
        thread.parent.init(crate::object::ObjectClassType::Thread, name);
        let libcpu = system!().libcpu();
        let ptr = thread.stack_addr as u32;
        thread.sp = libcpu.stack_init(thread.entry, thread.parameter,
                             (ptr + thread.stack_size-16)as *mut (), Self::thread_exit);

        let timer_parameter = thread.as_mut_ptr() as *mut ();
        Timer::init(&mut thread.thread_timer, Self::thread_timeout, timer_parameter, 0, 0);
        thread
    }

    fn suspend_with_flag(&mut self, suspend_flag:SuspendWithFlag) -> Result<(),Error>{
        let libcpu = system!().libcpu();
        let level = libcpu.interrupt_disable();
        let stat = self.stat & Status::StatMask as u8;
        if (stat != Status::Ready as u8) && (stat != Status::Running as u8){
            libcpu.interrupt_enable(level);
            return Err(Error::Error);
        }
        // scheduler!(remove_thread(self));
        let stat = match suspend_flag {
            SuspendWithFlag::INTERRUPTIBLE => Status::SUSPEND_INTERRUPTIBLE as u8,
            _ => unreachable!(),
        };
        self.stat = stat | (self.stat & !(Status::StatMask as u8));
        libcpu.interrupt_enable(level);
        Ok(())
    }

    pub fn sleep(&mut self, tick:usize) -> Result<(),Error>{
        let result: Result<(), Error>;
        let libcpu = system!().libcpu();
        let level = libcpu.interrupt_disable();
        result = self.suspend_with_flag(SuspendWithFlag::INTERRUPTIBLE);
        match result {
            Ok(()) => {
                let timer = self.thread_timer.as_mut().unwrap();
                timer.control(tick);
                timer.start();
                self.timer_run = true;
                libcpu.interrupt_enable(level);
                schedule!();
            },
            Err(_e) => {
                libcpu.interrupt_enable(level);
            },
        }
        result
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

    fn resume(self){
        if (self.stat & Status::SuspendMask as u8) != Status::SuspendMask as u8{
            return;
        }
        scheduler!(insert_thread(self));
    }

    pub fn startup(mut self){
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
}

#[macro_export]
macro_rules! thread_self_mut {
    () => {{
        $crate::scheduler!(current_thread_mut())
    }};
}

#[macro_export]
macro_rules! thread_self {
    () => {{
        $crate::scheduler!(current_thread())
    }};
}

#[macro_export]
macro_rules! thread_sleep {
    ($tick:expr) => {{
        $crate::scheduler!(current_thread_mut()).unwrap().sleep($tick)
    }};
}
