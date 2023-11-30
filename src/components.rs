extern crate libcpu;

pub fn entry(){
    rtthread_startup();
}

fn rtthread_startup(){
    libcpu::rt_hw_interrupt_disable();

}

