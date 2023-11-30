
#![no_std]
use core::arch::asm;
pub fn rt_hw_interrupt_disable() {
    unsafe {
        asm!(
            "MRS     r0, PRIMASK",
            "CPSID   I",
            "BX      LR"
        );
    }
}