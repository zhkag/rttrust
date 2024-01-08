use libcpu;
use crate::tick;
use core::arch::asm;
use crate::thread::Thread;
#[export_name = "rt_hw_context_switch_to"]
pub extern "C" fn rt_hw_context_switch_to(_sp: *mut ()) {
    unsafe{asm!("bl   rt_hw_context_switch_to_base");}
}

#[export_name = "rt_hw_context_switch"]
pub extern "C" fn rt_hw_context_switch(_from_sp: *mut (), _to_sp: *mut ()) {
    unsafe{asm!("bl   rt_hw_context_switch_base");}
}

#[export_name = "rt_hw_context_switch_interrupt"]
pub extern "C" fn rt_hw_context_switch_interrupt(_from_sp: *mut (), _to_sp: *mut (),_from_thread:&mut Thread,_to_thread:&mut Thread) {
    unsafe{asm!("bl   rt_hw_context_switch_base");}
}

pub extern "C" fn interrupt_disable() -> isize{
    let level:isize;
    unsafe{
        asm!("bl   rt_hw_interrupt_disable");
        asm!("mov {}, r0", out(reg) level);
    }
    level
}

pub extern "C" fn interrupt_enable(_level:isize){
    unsafe{asm!("bl   rt_hw_interrupt_enable");}
}

#[no_mangle]
fn kernel_sys_tick() {
    tick!(increase());
}


impl crate::hw::HardWare {
    pub fn stack_init(_entry: fn(*mut ()), _parameter:*mut (),_stack_addr:*mut (),_exit: fn())->*mut (){
        let stk:*mut ();
        unsafe{
            asm!("bl   rt_hw_stack_init");
            asm!("mov {}, r0", out(reg) stk);
        }
        stk
    }
}