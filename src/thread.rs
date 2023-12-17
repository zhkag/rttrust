use crate::hw::HardWare;
use crate::schedule;


#[derive(Copy)]
pub struct Thread
{
    sp: *mut (),
    entry:fn(*mut ()),
    parameter: *mut (),
    stack_addr: *mut (),
    stack_size:u32,
}

impl Clone for Thread {
    fn clone(&self) -> Self {
        Thread {
            sp: self.sp,
            entry:self.entry,
            parameter: self.parameter,
            stack_addr: self.stack_addr,
            stack_size:self.stack_size,
        }
    }
}

fn _thread_exit()
{
    schedule!();
}



impl Thread {
    pub fn new(entry: fn(*mut ()),parameter:*mut (),stack_start:*mut (),stack_size:u32) -> Thread {
        let mut thread = Thread {
            entry,
            parameter,
            stack_addr:stack_start,
            stack_size,
            sp:core::ptr::null_mut(),
        };
        let ptr = thread.stack_addr as u32;
        thread.sp = HardWare::stack_init(thread.entry, thread.parameter, (ptr+thread.stack_size-16)as *mut (), _thread_exit);
        thread
    }
    pub fn sp(&self) ->*mut (){
        self.sp
    }

    pub fn startup(&self){
    }
}