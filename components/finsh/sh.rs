use kernel::println;
use kernel::macros::sh_function_expopt;

struct ShSyscall<'a>{
    name:&'a str,
    desc:&'a str,
    func:fn(),
}


#[sh_function_expopt(cmd_test "cmd_test function")]
fn cmd_test(){
    println!("cmd_test function");
}

#[sh_function_expopt]
fn cmd_test2(){

}

fn sh_help() {
    extern {static __fsymtab_start:usize;static __fsymtab_end:usize;}
    let fsymtab_start = unsafe{&__fsymtab_start as *const usize as *const ShSyscall};
    let fsymtab_end = unsafe{&__fsymtab_end as *const usize as *const ShSyscall};
    let mut fn_ptr = fsymtab_start;
    while fn_ptr < fsymtab_end {
        println!("{} - {}", unsafe {(*fn_ptr).name}, unsafe {(*fn_ptr).desc});
        fn_ptr = unsafe {fn_ptr.offset(1)};
    }
}

fn sh_get_cmd(cmd:&str) -> Option<&ShSyscall> {
    extern {static __fsymtab_start:usize;static __fsymtab_end:usize;}
    let fsymtab_start = unsafe{&__fsymtab_start as *const usize as *const ShSyscall};
    let fsymtab_end = unsafe{&__fsymtab_end as *const usize as *const ShSyscall};
    let mut fn_ptr = fsymtab_start;
    while fn_ptr < fsymtab_end {
        if cmd == unsafe{(*fn_ptr).name}{
            return Some(unsafe{&(*fn_ptr)})
        }
        fn_ptr = unsafe {fn_ptr.offset(1)};
    }
    None
}


const SH_THREAD_STACK_SIZE: usize = 10240;
static mut SH_THREAD_STACK: [u8; SH_THREAD_STACK_SIZE] = [0; SH_THREAD_STACK_SIZE];

fn sh_fun(_parameter:*mut ()) -> Result<(),Error>{
    loop {
        // thread_sleep!(10)?;
        // println!("sh thread!");
        if let Some(c) = system!(getc()) {
            kernel::print!("{}",char::from_u32(c as u32).unwrap());
        }
    }
    Ok(())
}

use kernel::macros::init_export;
use kernel::system;
use kernel::thread::Thread;
use kernel::thread_sleep;
use kernel::Error;
#[init_export("6")]
fn sh_test(){
    sh_help();
    if let Some(test) = sh_get_cmd("cmd_test"){
        println!("test.name {}",test.name);
        println!("test.desc {}",test.desc);
        (test.func)();
    }

    let stack_size:u32 = core::mem::size_of::<[u8; SH_THREAD_STACK_SIZE]>().try_into().unwrap();
    let stack_start = unsafe {SH_THREAD_STACK.as_mut_ptr() as *mut ()};
    let sh_thread = Thread::init("sh", sh_fun, core::ptr::null_mut(),
                                                stack_start, stack_size, 23, 32);
    sh_thread.startup();
}
