use crate::system;
use crate::schedule;
use crate::thread::Status;
use core::sync::atomic::{AtomicUsize, Ordering};

pub struct Tick{
    value: AtomicUsize
}

impl Tick {
    pub fn new() -> Self {
        Self { value: AtomicUsize::new(0)}
    }
    pub fn increase(&mut self) {
        self.value.fetch_add(1, Ordering::SeqCst);
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
        system.check(self.value.load(Ordering::SeqCst));
    }
    pub fn get(&self) -> usize {
        self.value.load(Ordering::SeqCst)
    }
    pub fn set(&mut self,tick:usize) {
        self.value.store(tick, Ordering::SeqCst);
    }
}
