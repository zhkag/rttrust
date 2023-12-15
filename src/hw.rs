pub struct HardWare {
    
}


#[repr(C)]
struct ExceptionStackFrame {
    r0: u32,
    r1: u32,
    r2: u32,
    r3: u32,
    r12: u32,
    lr: u32,
    pc: u32,
    psr: u32,
}
#[repr(C)]
struct StackFrame {
    flag: u32,
    r4: u32,
    r5: u32,
    r6: u32,
    r7: u32,
    r8: u32,
    r9: u32,
    r10: u32,
    r11: u32,
    exception_stack_frame: ExceptionStackFrame,
}


const SCS_BASE: u32 = 0xE000E000;
const SYS_TICK_BASE: u32 = SCS_BASE + 0x0010;

#[repr(C)]
struct SysTickType {
    ctrl:u32,
    load:u32,
    val:u32,
    valib:u32,
}

fn systick_init() {
    let systick_ptr: *mut SysTickType = SYS_TICK_BASE as *mut SysTickType;
    unsafe {
    let systick = &mut *systick_ptr;
    systick.ctrl |= 1 << 2;
    systick.ctrl |= 1 << 0;                 /* 使能Systick */
    systick.load = 0x0fffffff;              /* 注意systick计数器24位，所以这里设置最大重装载值 */
    systick.ctrl |= 1 << 1;                 /* 开启SYSTICK中断 */
    systick.load = 100000;                  /* 每1/delay_ostickspersec秒中断一次 */
    }
}


impl HardWare {
    pub fn board_init() {
        systick_init();
    }
    pub fn stack_init(entry: fn(*mut ()), parameter:*mut (),stack_addr:*mut (),exit: fn())->*mut (){
        let mut stk: *mut () = (stack_addr as usize + core::mem::size_of::<u32>()) as *mut ();
        stk = ((stk as usize) & !7) as *mut ();
        stk = (stk as usize - core::mem::size_of::<StackFrame>()) as *mut ();
    
        let stack_frame = stk as *mut StackFrame;
    

        let stack_frame_ref = unsafe { &mut *stack_frame };
        let stack_frame_ptr = stack_frame as *mut u32;
        let stack_frame_len = core::mem::size_of::<StackFrame>() / core::mem::size_of::<u32>();
        for i in 0..stack_frame_len {
            unsafe {
                *stack_frame_ptr.offset(i as isize) = 0xdeadbeef;
            }
        }
    
        stack_frame_ref.exception_stack_frame.r0 = parameter as u32; // r0 : argument
        stack_frame_ref.exception_stack_frame.r1 = 0; // r1
        stack_frame_ref.exception_stack_frame.r2 = 0; // r2
        stack_frame_ref.exception_stack_frame.r3 = 0; // r3
        stack_frame_ref.exception_stack_frame.r12 = 0; // r12
        stack_frame_ref.exception_stack_frame.lr = exit as u32; // lr
        stack_frame_ref.exception_stack_frame.pc = entry  as u32; // entry point, pc
        stack_frame_ref.exception_stack_frame.psr = 0x01000000; // PSR
 
        stack_frame_ref.flag = 0;
        stk
    }
}