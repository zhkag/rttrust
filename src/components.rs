
use crate::system;

#[no_mangle]
fn entry() {
    system!(startup());
    unreachable!();
}

use macros::init_export;

#[init_export("0")]
fn rti_start() {}

#[init_export("0.end")]
fn rti_board_start() {}

#[init_export("1.end")]
fn rti_board_end() {}

#[init_export("6.end")]
fn rti_end() {}

pub fn board_init(){
    let start_ptr = &__rt_init_rti_board_start as *const fn();
    let end_ptr = &__rt_init_rti_board_end as *const fn();
    let mut fn_ptr = start_ptr;
    while fn_ptr < end_ptr {
        unsafe {(*fn_ptr)();}
        fn_ptr = unsafe {fn_ptr.offset(1)};
    }
}
