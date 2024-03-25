use crate::system::System;
use crate::tick;
use crate::thread::Thread;
use crate::interrupt_leave;
use crate::interrupt_enter;
use crate::Error;
use crate::Box;

pub fn sys_tick() {
    interrupt_enter!();
    tick!(increase());
    interrupt_leave!();
}

pub trait LibcpuTrait {
    fn context_switch_to(&self, _sp: *mut ()){unreachable!();}
    fn context_switch(&self, _from_sp: *mut (), _to_sp: *mut ()){unreachable!();}
    fn context_switch_interrupt(&self, _from_sp: *mut (), _to_sp: *mut (),_from_thread:&mut Thread,_to_thread:&mut Thread){unreachable!();}
    fn interrupt_disable(&self) -> isize{unreachable!();}
    fn interrupt_enable(&self, _level:isize){unreachable!();}
    fn stack_init(&self, _entry: fn(*mut ()) -> Result<(),Error>, _parameter:*mut (),_stack_addr:*mut (),_exit: fn(_err:Result<(),Error>)) -> *mut (){unreachable!();}
}

impl System {
    pub fn libcpu(&mut self) -> &mut Box<dyn LibcpuTrait>{
        self.libcpu.as_mut().unwrap()
    }
    pub fn libcpu_trait_init(&mut self,item: impl LibcpuTrait + 'static) {
        self.libcpu = Some(Box::new(item));
    }
}
