use crate::system;
use crate::schedule;
use crate::thread::Status;

pub struct Tick{
    value:usize
}

impl Tick {
    pub fn new() -> Self {
        Self { value: 0 }
    }
    pub fn increase(&mut self) {
        self.value += 1;
        let system = system!();
        let libcpu = system!().libcpu();
        let level = libcpu.interrupt_disable();
        if let Some(thread) = system.scheduler_mut().current_thread() {
            if thread.tick_decrease() == 0 {
                thread.set_stat(thread.stat() | Status::StatYield as u8);
                libcpu.interrupt_enable(level);
                schedule!();
            }
        }
        libcpu.interrupt_enable(level);
        system.check(self.value);
    }
    pub fn get(&self) -> usize {
        self.value
    }
    pub fn set(&mut self,tick:usize) {
        self.value = tick;
    }
}
