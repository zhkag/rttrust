use kernel::{macros::init_export, println, scheduler};

#[init_export("6")]
fn list_thread() {
    scheduler!(list_thread(|thread|{println!("{}",thread)}));
}
