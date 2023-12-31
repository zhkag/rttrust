use crate::thread_self;
use crate::schedule;
use crate::thread::Status;
use crate::timer::Timer;

pub struct Tick{
    value:usize
}

impl Tick {
    pub fn new() -> Self {
        Self { value: 0 }
    }
    pub fn increase(&mut self) {
        self.value += 1;
        if let Some(thread) = thread_self!() {
            if thread.tick_decrease() == 0 {
                thread.set_stat(thread.stat() | Status::StatYield as u8);
                schedule!();
            }
        }
        Timer::check(self.value);
    }
    pub fn get(&self) -> usize {
        self.value
    }
    pub fn set(&mut self,tick:usize) {
        self.value = tick;
    }
}
