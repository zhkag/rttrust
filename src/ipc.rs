use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

use crate::{List, thread::Thread, thread_self};

struct Spinlock {
    lock: AtomicBool,
}

impl Spinlock {
    const fn new() -> Self {
        Spinlock {
            lock: AtomicBool::new(false),
        }
    }
    fn lock(&self) {
        while self.lock.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed).is_err(){}
    }

    fn unlock(&self) {
        self.lock.store(false, Ordering::Release);
    }
}

pub struct Mutex<T> {
    data: T,
    is_locked: AtomicBool,
}

impl<T> Mutex<T> {
    pub const fn new(data: T) -> Self {
        Mutex {
            data,
            is_locked: AtomicBool::new(false),
        }
    }

    pub fn lock(&mut self) -> Option<MutexGuard<'_, T>> {
        while self.is_locked.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst).is_ok(){
            return Some(MutexGuard { mutex: self});
        }
        None
    }
    pub fn unlock(&self) {
        self.is_locked.store(false, Ordering::SeqCst);
    }
}

pub struct MutexGuard<'a, T> {
    mutex: &'a mut Mutex<T>,
}

impl<T> MutexGuard<'_,T> {
    pub fn data(&mut self) -> &mut T {
        &mut self.mutex.data
    }
}

impl<'a, T> Drop for MutexGuard<'a, T> {
    fn drop(&mut self) {
        self.mutex.unlock();
    }
}

// // 示例使用
// fn main() {
//     let mut mutex = Mutex::new(0);

//     if let Some(guard) = mutex.lock() {
//         guard.mutex.data += 1;
//         let test = crate::system!();
//         } else {
//         };
// }


pub struct Semaphore {
    permits: usize,
    mutex: Mutex<()>,
    thread_list:List<Thread>,
}

impl Semaphore {
    pub fn new(permits: usize) -> Self {
        Semaphore {
            permits,
            mutex: Mutex::new(()),
            thread_list:List::new()
        }
    }

    pub fn acquire(&mut self) {
        let _lock = self.mutex.lock();
        if self.permits > 0 {
            self.permits -= 1;
        }
        else {
            // let thread = thread_self!();
            // self.thread_list.push_back(thread.unwrap());
        }
    }

    pub fn release(&mut self) {
        let _lock = self.mutex.lock();
        self.permits += 1;
    }
}