use crate::thread::{Thread,Status};
use crate::list::List;
use crate::{heaplist, thread_self_mut};
use crate::{thread_self, system};
use crate::scheduler;
use crate::interrupt_nest;
use crate::include::{*};


extern crate alloc;
pub struct Scheduler{
    ready_priority_group:usize,
    current_thread:Option<Thread>,
    priority_table_heap:[Option<heaplist::List<Thread>>;THREAD_PRIORITY_MAX],
}

impl Scheduler {
    pub fn new() -> Self {
        const ARRAY_REPEAT_VALUE: Option<heaplist::List<Thread>> = None;
        let scheduler = Self{
            ready_priority_group:0,
            current_thread:None,
            priority_table_heap:[ARRAY_REPEAT_VALUE;THREAD_PRIORITY_MAX],
        };
        scheduler
    }
    pub fn init(&mut self){
        for value in 0..THREAD_PRIORITY_MAX{
            self.priority_table_heap[value] = Some(heaplist::List::new());
        }
    }

    pub fn schedule(&mut self){
        if self.ready_priority_group == 0 {
            return;
        }
        let libcpu = system!().libcpu();
        let level = libcpu.interrupt_disable();
         /* need_insert_from_thread: need to insert from_thread to ready queue */
        let mut need_insert_from_thread = false;
        let mut highest_ready_priority = 0;
        
        let mut binding = self.get_highest_priority_thread(&mut highest_ready_priority);
        let to_thread_clone = binding.clone().unwrap();
        let mut to_thread = binding.as_mut().unwrap();
        let current_thread = thread_self_mut!().unwrap(); 

        if (current_thread.stat() & Status::StatMask as u8) == Status::Running as u8 {
            if current_thread.current_priority() < highest_ready_priority {
                to_thread = current_thread;
            }
            else if current_thread.current_priority() == highest_ready_priority && (current_thread.stat() & Status::STAT_YIELD_MASK as u8) == 0 {
                to_thread = current_thread;
            }
            else {
                need_insert_from_thread = true;
            }
        }
        if *to_thread != thread_self!().unwrap()
        {
            /* if the destination thread is not the same as current thread */
            let mut from_thread = thread_self!().unwrap();
            if (from_thread.stat() & Status::STAT_YIELD_MASK as u8) != 0{
                thread_self_mut!().unwrap().set_stat(from_thread.stat() & !(Status::STAT_YIELD_MASK as u8));
            }

            scheduler!(set_current_thread(*to_thread));
            if need_insert_from_thread {
                scheduler!(insert_thread(from_thread));
            }
            let to_thread_mut =  thread_self_mut!().unwrap();
            to_thread_mut.set_stat(Status::Running as u8 | (to_thread_mut.stat() & !(Status::StatMask as u8)));

            let from_sp = (from_thread.sp_mut()) as *mut *mut () as *mut ();
            let to_sp = (to_thread.sp_mut()) as *mut *mut () as *mut ();
            if interrupt_nest!() == 0 {
                libcpu.context_switch(from_sp, to_sp);
                libcpu.interrupt_enable(level);
                return;
            }
            else {
                libcpu.context_switch_interrupt(from_sp,to_sp,&mut from_thread,to_thread_mut);
            }

        }else {
            self.priority_table_heap[to_thread_clone.current_priority() as usize].as_mut().unwrap().push_back(to_thread_clone);
            current_thread.set_stat(Status::Running as u8 | (current_thread.stat() & !(Status::StatMask as u8)));
        }
        libcpu.interrupt_enable(level);

    }

    
    pub fn insert_thread(&mut self,mut thread: Thread){
        let libcpu = system!().libcpu();
        let level = libcpu.interrupt_disable();
        if let Some(current_thread) = self.current_thread() {
            if thread == current_thread {
                thread.set_stat(Status::Running as u8|thread.stat() & !(Status::StatMask as u8));
                libcpu.interrupt_enable(level);
                return;
            }
        }
        thread.set_stat(Status::Ready as u8 | (thread.stat() & !(Status::StatMask as u8)));
        if (thread.stat() & (Status::STAT_YIELD_MASK as u8)) != 0 {
            let current_priority = thread.current_priority() as usize;
            self.priority_table_heap[current_priority].as_mut().unwrap().push_front(thread);

        }else {
            let current_priority = thread.current_priority() as usize;
            self.priority_table_heap[current_priority].as_mut().unwrap().push_back(thread);
        }
        self.ready_priority_group |= thread.number_mask() as usize;
        libcpu.interrupt_enable(level);
    }

    pub fn current_thread_mut(&mut self) -> Option<&mut Thread> {
        self.current_thread.as_mut()
    }
    pub fn current_thread(&mut self) -> Option<Thread> {
        self.current_thread
    }
    pub fn set_current_thread(&mut self, thread:Thread){
        self.current_thread = Some(thread);
    }

    fn get_highest_priority_thread(&mut self,highest_prio: &mut u8) -> Option<Thread> {
        let highest_ready_priority:u8 = self.ready_priority_group.trailing_zeros() as u8;
        *highest_prio = highest_ready_priority;
        self.priority_table_heap[highest_ready_priority as usize].as_mut().unwrap().pop_front()
    }

    pub fn start(&mut self) {
        let system = system!();
        let mut highest_ready_priority = 0;
        let mut binding = self.get_highest_priority_thread(&mut highest_ready_priority);
        let to_thread = binding.as_mut().unwrap();
        to_thread.set_stat(Status::Running as u8);
        system.scheduler_mut().set_current_thread(*to_thread);
        let sp = to_thread.sp_mut() as *mut *mut () as *mut ();
        system.libcpu().context_switch_to(sp);
        unreachable!();
    }
}
