use crate::hw::HardWare;
use crate::{scheduler,schedule};
use crate::list::List;
use crate::thread_self;

use core::ops::{BitAnd,BitOr,Not};

#[derive(PartialEq)]
#[derive(Copy, Clone)]
pub enum Status {
    INIT        = 0x00,     // Initialized status
    CLOSE       = 0x01,     // Closed status
    READY       = 0x02,     // Ready status
    RUNNING     = 0x03,     // Running status
    SUSPEND_MASK= 0x04,
    STAT_MASK   = 0x07,
}

impl Status {
    const SUSPEND_INTERRUPTIBLE: Status = Status::SUSPEND_MASK;
    const SUSPEND: Status = Status::SUSPEND_INTERRUPTIBLE;
}

impl BitAnd for Status {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        self
    }
}
impl BitOr for Status {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        self
    }
}
impl Not for Status {
    type Output = Self;
    fn not(self) -> Self::Output {
        self
    }
}

#[derive(PartialEq)]
#[derive(Copy, Clone)]
pub struct Thread
{
    sp: *mut (),
    entry:fn(*mut ()),
    parameter: *mut (),
    stack_addr: *mut (),
    stack_size:u32,
    pub list:List<Thread>,
    number_mask:u32,
    current_priority:u8,
    init_priority:u8,
    stat: Status,
}

fn _thread_exit()
{
    let thread = thread_self!();
    thread.stat = Status::INIT;
    schedule!();
}


impl Thread {
    fn new(entry: fn(*mut ()),parameter:*mut (),stack_start:*mut (),stack_size:u32,priority:u8) -> Thread {
        let mut thread = Thread {
            entry,
            parameter,
            stack_addr:stack_start,
            stack_size,
            sp:core::ptr::null_mut(),
            list:List::init(),
            init_priority:priority,
            current_priority:priority,
            number_mask: 1 << priority,
            stat:Status::INIT,
        };
        let ptr = thread.stack_addr as u32;
        thread.sp = HardWare::stack_init(thread.entry, thread.parameter, (ptr+thread.stack_size-16)as *mut (), _thread_exit);
        thread
    }
    pub fn init(thread: &mut Option<Thread>,entry: fn(*mut ()),parameter:*mut (),stack_start:*mut (),stack_size:u32,priority:u8) -> &mut Thread{
        *thread = Some(Thread::new(entry, parameter, stack_start, stack_size, priority));
        thread.as_mut().unwrap()
    }

    pub fn sp(&self) ->*mut (){
        self.sp
    }
    pub fn current_priority(&self) ->u8 {
        self.current_priority
    }
    pub fn number_mask(&self) ->u32 {
        self.number_mask
    }
    
    fn resume(&mut self){
        if (self.stat & Status::SUSPEND_MASK) != Status::SUSPEND_MASK {
            return;
        }
        scheduler!(insert_thread(self));
    }

    pub fn startup(&mut self){
        self.stat = Status::SUSPEND;
        self.resume();
        if scheduler!(current_thread_is_some()){
            schedule!();
        }
    }

    pub fn stat(&self) -> Status {
        self.stat
    }
    pub fn set_stat(&mut self, stat:Status) {
        self.stat = stat;
    }

}

#[macro_export]
macro_rules! thread_self {
    () => {{
        crate::scheduler!(current_thread())
    }};
}
