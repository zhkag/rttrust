use crate::system;
use core::sync::atomic::{AtomicUsize, Ordering};

pub struct Interrupt{
    nest:AtomicUsize,
}

impl Interrupt{
    pub fn init() -> Interrupt{
        Interrupt{nest: AtomicUsize::new(0)}
    }
    pub fn enter(&mut self) {
        self.nest.fetch_add(1, Ordering::SeqCst);
    }
    pub fn leave(&mut self) {
        self.nest.fetch_sub(1, Ordering::SeqCst);
    }
    pub fn nest(&self) -> usize{
        self.nest.load(Ordering::SeqCst)
    }
}

use crate::Box;
use crate::LibcpuTrait;

pub struct InterruptGuard<'a, T> {
    data: &'a mut T,
    libcpu:&'a mut Box<dyn LibcpuTrait>,
    level:isize,
}

impl<'a, T> InterruptGuard<'_,T> {
    pub fn new(data: &'a mut T) -> InterruptGuard<'a, T>{
        let mut guard = InterruptGuard { data, libcpu: system!().libcpu(), level: 0 };
        guard.level = guard.libcpu.interrupt_disable();
        guard
    }
    pub fn data(&mut self) -> &mut T {
        self.data
    }
}

impl<'a, T> Drop for InterruptGuard<'a, T> {
    fn drop(&mut self) {
        self.libcpu.interrupt_enable(self.level);
    }
}