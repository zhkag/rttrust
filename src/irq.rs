use crate::libcpu;

pub struct Interrupt{
    nest:usize,
}

impl Interrupt{
    pub fn init() -> Interrupt{
        Interrupt{nest:0}
    }
    pub fn enter(&mut self) {
        let level = libcpu::interrupt_disable();
        self.nest += 1;
        libcpu::interrupt_enable(level);
    }
    pub fn leave(&mut self) {
        let level = libcpu::interrupt_disable();
        self.nest -= 1;
        libcpu::interrupt_enable(level);
    }
    pub fn nest(&self) -> usize{
        self.nest
    }
}
