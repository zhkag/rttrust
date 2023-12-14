use crate::hw::HardWare;

pub struct Thread
{
    sp: *mut (),
    entry:fn(*mut ()),
    parameter: *mut (),
    stack_addr: *mut (),
    stack_size:u32,
}


fn _thread_exit()
{

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
}