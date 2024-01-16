
use crate::system;

#[no_mangle]
fn entry() {
    system!(startup());
    unreachable!();
}

