use core::arch::asm;
use core::arch::global_asm;
use crate::arm::cortex_m4::cpuport;

global_asm!(".equ  SCB_VTOR,           0xE000ED08");
global_asm!(".equ  NVIC_INT_CTRL,      0xE000ED04");
global_asm!(".equ  NVIC_SYSPRI2,       0xE000ED20");
global_asm!(".equ  NVIC_PENDSV_PRI,    0xFFFF0000");
global_asm!(".equ  NVIC_PENDSVSET,     0x10000000");

#[export_name = "rt_hw_interrupt_disable"]
pub extern "C" fn rt_hw_interrupt_disable(){
    unsafe{
        asm!("MRS     r0, PRIMASK");
        asm!("CPSID   I");
    }
}

#[export_name = "rt_hw_interrupt_enable"]
pub extern "C" fn rt_hw_interrupt_enable(_level:isize){
    unsafe{
        asm!("MSR     PRIMASK, r0");
    }
}

#[no_mangle]
unsafe extern "C" fn rt_hw_context_switch_base() {
    asm!("LDR   r2, ={}",sym cpuport::RT_THREAD_SWITCH_INTERRUPT_FLAG);
    asm!("LDR   r3, [r2]");
    asm!("CMP   r3, #1");
    asm!("BEQ   0f");
    asm!("MOV   r3, #1");
    asm!("STR   r3, [r2]");

    asm!("LDR   r2, ={}",sym cpuport::RT_INTERRUPT_FROM_THREAD);
    asm!("STR   r0, [r2]");

    asm!("0:");
    asm!("LDR   r2, ={}",sym cpuport::RT_INTERRUPT_TO_THREAD);
    asm!("STR   r1, [r2]");

    asm!("LDR   r0, =NVIC_INT_CTRL");
    asm!("LDR   r1, =NVIC_PENDSVSET");
    asm!("STR   r1, [r0]");
}

#[no_mangle]
unsafe extern "C" fn rt_hw_context_switch_to_base() {
    asm!("LDR   r1, ={}",sym cpuport::RT_INTERRUPT_TO_THREAD);
    asm!("STR   r0, [r1]");
    asm!("MRS   r2, CONTROL");
    asm!("BIC   r2, #0x04   ");
    asm!("MSR   CONTROL, r2 ");
    asm!("LDR   r1, ={}",sym cpuport::RT_INTERRUPT_FROM_THREAD);
    asm!("MOV   r0, #0x0");
    asm!("STR   r0, [r1]");
    asm!("LDR   r1, ={}",sym cpuport::RT_THREAD_SWITCH_INTERRUPT_FLAG);
    asm!("MOV   r0, #1");
    asm!("STR   r0, [r1]");
    asm!("LDR   r0, =NVIC_SYSPRI2");
    asm!("LDR   r1, =NVIC_PENDSV_PRI");
    asm!("LDR.W r2, [r0,#0x00]");
    asm!("ORR   r1,r1,r2");
    asm!("STR   r1, [r0]");
    asm!("LDR   r0, =NVIC_INT_CTRL");
    asm!("LDR   r1, =NVIC_PENDSVSET");
    asm!("STR   r1, [r0]");
    asm!("LDR   r0, =SCB_VTOR");
    asm!("LDR   r0, [r0]");
    asm!("LDR   r0, [r0]");
    asm!("NOP");
    asm!("MSR   msp, r0");
    asm!("CPSIE F");
    asm!("CPSIE I");
    asm!("DSB");
    asm!("ISB");
}

#[export_name = "SysTick_Handler"]
unsafe extern "C" fn sys_tick_handler() {
    asm!("bl kernel_sys_tick");
}

#[export_name = "PendSV_Handler"]
unsafe extern "C" fn pend_sv_handler() {
    asm!("pop   {{r7, lr}}");  //rust 函数会添加汇编压栈，但是此函数并不需要，如果没有这行的话会导致系统栈一直压栈
    asm!("MRS   r2, PRIMASK");
    asm!("CPSID I");
    asm!("LDR   r0, ={}",sym cpuport::RT_THREAD_SWITCH_INTERRUPT_FLAG);
    asm!("LDR   r1, [r0]");
    asm!("CBZ   r1, 1f");      
    asm!("MOV   r1, #0x00");
    asm!("STR   r1, [r0]");
    asm!("LDR   r0, ={}",sym cpuport::RT_INTERRUPT_FROM_THREAD);
    asm!("LDR   r1, [r0]");
    asm!("CBZ   r1, 0f");
    asm!("MRS   r1, psp");
    asm!("TST   lr, #0x10");
    asm!("IT    EQ", "VSTMDBEQ r1!, {{d8 - d15}}");
    asm!("STMFD r1!, {{r4 - r11}}");
    asm!("MOV   r4, #0x00");  
    asm!("TST   lr, #0x10");
    asm!("IT    EQ", "MOVEQ   r4, #0x01");
    asm!("STMFD r1!, {{r4}}");
    asm!("LDR   r0, [r0]");
    asm!("STR   r1, [r0]");
    asm!("0:");
    asm!("LDR   r1, ={}",sym cpuport::RT_INTERRUPT_TO_THREAD);
    asm!("LDR   r1, [r1]");
    asm!("LDR   r1, [r1]");
    asm!("LDMFD r1!, {{r3}}");
    asm!("LDMFD r1!, {{r4 - r11}}");
    asm!("CMP   r3, #0");
    asm!("IT    NE", "VLDMIANE  r1!, {{d8 - d15}}");
    asm!("MSR   psp, r1");
    asm!("ORR   lr, lr, #0x10");
    asm!("CMP   r3, #0");
    asm!("IT    NE", "BICNE   lr, lr, #0x10");
    asm!("1:");
    asm!("MSR   PRIMASK, r2");
    asm!("ORR   lr, lr, #0x04");
    asm!("BX    lr");
}

#[export_name = "SystemInit"]
extern "C" fn system_init() {

}

#[no_mangle]
fn __libc_init_array() {
    //TODO link with newlib properly
}

#[export_name = "Reset_Handler"]
unsafe extern "C" fn reset_handler() {
    asm!("ldr   sp, =_estack");
    asm!("bl    SystemInit");
    asm!("ldr   r0, =_sdata");
    asm!("ldr   r1, =_edata");
    asm!("ldr   r2, =_sidata");
    asm!("movs  r3, #0");
    asm!("b     1f");
    asm!("0:");
    asm!("ldr   r4, [r2, r3]");
    asm!("str   r4, [r0, r3]");
    asm!("adds  r3, r3, #4");
    asm!("1:");
    asm!("adds  r4, r0, r3");
    asm!("cmp   r4, r1");
    asm!("bcc   0b");
    asm!("ldr   r2, =_sbss");
    asm!("ldr   r4, =_ebss");
    asm!("movs  r3, #0");
    asm!("b     3f");
    asm!("2:");
    asm!("str   r3, [r2]");
    asm!("adds  r2, r2, #4");
    asm!("3:");
    asm!("cmp   r2, r4");
    asm!("bcc   2b");
    asm!("bl    __libc_init_array");
    asm!("bl    entry");
    asm!("bx    lr");
}
