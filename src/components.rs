
use crate::system;

#[no_mangle]
fn entry() {
    system!(startup());
    unreachable!();
}


fn rti_start() -> i32{
    return 0;
}

#[link_section = ".rti_fn.0"]
static __RT_INIT_RTI_START: fn() -> i32 = rti_start;

fn rti_board_start() -> i32{
    return 0;
}

#[link_section = ".rti_fn.0.end"]
static __RT_INIT_RTI_BOARD_START: fn() -> i32 = rti_board_start;


fn rti_board_end() -> i32{
    return 0;
}

#[link_section = ".rti_fn.1.end"]
static __RT_INIT_RTI_BOARD_END: fn() -> i32 = rti_board_end;

fn rti_end() -> i32{
    return 0;
}

#[link_section = ".rti_fn.6.end"]
static __RT_INIT_RTI_END: fn() -> i32 = rti_end;

