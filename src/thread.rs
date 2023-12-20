use crate::hw::HardWare;
use crate::{scheduler,schedule};
use crate::List;



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
}

fn _thread_exit()
{
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
        scheduler!(insert_thread(self));
    }

    pub fn startup(&mut self){
        self.resume();
        if thread_self().is_some(){
            schedule!();
        }
    }
}

pub fn thread_self() -> Option<Thread>{
    scheduler!(current_thread())
}