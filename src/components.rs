
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

fn section_fn_run(start_ptr: *const fn(), end_ptr: *const fn()){
    let mut fn_ptr = start_ptr;
    while fn_ptr < end_ptr {
        unsafe {(*fn_ptr)();}
        fn_ptr = unsafe {fn_ptr.offset(1)};
    }
}

pub fn board_init(){
    section_fn_run(&__rt_init_rti_board_start,&__rt_init_rti_board_end);
}

pub fn init(){
    section_fn_run(&__rt_init_rti_board_end,&__rt_init_rti_end);
}