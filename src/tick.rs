use crate::thread_self;

pub struct Tick{
    value:usize
}

impl Tick {
    pub fn new() -> Tick {
        Tick { value: 0 }
    }
    pub fn increase(&mut self) {
        self.value += 1;
        let thread = thread_self!();
    }
    pub fn get(&self) -> usize {
        self.value
    }
    pub fn set(&mut self,tick:usize) {
        self.value = tick;
    }
}
