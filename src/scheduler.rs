use crate::thread::{Thread,Status};
use crate::{List, thread_self_mut};
use crate::{thread_self, system};
use crate::scheduler;
use crate::interrupt_nest;
use crate::include::{*};

pub struct Scheduler{
    ready_priority_group:usize,
    last_thread:Option<Thread>,
    current_thread:Option<Thread>,
    priority_table_heap:[Option<List<Thread>>;THREAD_PRIORITY_MAX],
    thread_timer_list:List<Thread>,
}

impl Scheduler {
    pub fn new() -> Self {
        const ARRAY_REPEAT_VALUE: Option<List<Thread>> = None;
        let scheduler = Self{
            ready_priority_group:0,
            last_thread:None,
            current_thread:None,
            priority_table_heap:[ARRAY_REPEAT_VALUE;THREAD_PRIORITY_MAX],
            thread_timer_list:List::new(),
        };
        scheduler
    }
    pub fn init(&mut self){
        for value in 0..THREAD_PRIORITY_MAX{
            self.priority_table_heap[value] = Some(List::new());
        }
    }

    pub fn thread_timer_list_mut(&mut self) ->&mut List<Thread>{
        &mut self.thread_timer_list
    }

    pub fn solve_last_thread(&mut self){
        if let Some(thread) = self.last_thread {
            self.last_thread = None;
            if thread.timer_run(){
                self.thread_timer_list.insert_with_cmp(thread, |a, b|
                    a.timeout_tick() < b.timeout_tick());
            }
            else {
                self.insert_thread(thread);
            }
        }
    }

    pub fn schedule(&mut self){
        self.solve_last_thread();
        if self.ready_priority_group == 0 {
            return;
        }
        let libcpu = system!().libcpu();
        let level = libcpu.interrupt_disable();
        let mut highest_ready_priority = 0;

        let binding = self.get_highest_priority_thread(&mut highest_ready_priority);
        let next_thread = binding.unwrap();
        let mut prev_thread = thread_self!().unwrap();

        let mut to_thread = next_thread.clone();

        if (prev_thread.stat() & Status::StatMask as u8) == Status::Running as u8 {
            if prev_thread.current_priority() < highest_ready_priority {
                to_thread = prev_thread;
            }
            else if prev_thread.current_priority() == highest_ready_priority && (prev_thread.stat() & Status::STAT_YIELD_MASK as u8) == 0 {
                to_thread = prev_thread;
            }
        }

        if to_thread != thread_self!().unwrap()
        {
            if (prev_thread.stat() & Status::STAT_YIELD_MASK as u8) != 0{
                prev_thread.set_stat(prev_thread.stat() & !(Status::STAT_YIELD_MASK as u8));
            }
            scheduler!(set_current_thread(next_thread));
            self.set_last_thread(prev_thread);
            let to_thread_mut =  thread_self_mut!().unwrap();
            to_thread_mut.set_stat(Status::Running as u8 | (to_thread_mut.stat() & !(Status::StatMask as u8)));

            let from_thread_mut =  self.last_thread_mut().unwrap();

            let from_sp = (from_thread_mut.sp_mut()) as *mut *mut () as *mut ();
            let to_sp = (to_thread_mut.sp_mut()) as *mut *mut () as *mut ();
            if interrupt_nest!() == 0 {
                libcpu.context_switch(from_sp, to_sp);
                libcpu.interrupt_enable(level);
                return;
            }
            else {
                libcpu.context_switch_interrupt(from_sp,to_sp,from_thread_mut,to_thread_mut);
            }
        }
        else {
            self.ready_priority_group |= next_thread.number_mask() as usize;
            self.priority_table_heap[next_thread.current_priority() as usize].as_mut().unwrap().push_back(next_thread);
            prev_thread.set_stat(Status::Running as u8 | (prev_thread.stat() & !(Status::StatMask as u8)));
            self.set_current_thread(prev_thread);
        }
        libcpu.interrupt_enable(level);
    }


    pub fn insert_thread(&mut self,mut thread: Thread){
        self.solve_last_thread();
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

    pub fn last_thread_mut(&mut self) -> Option<&mut Thread> {
        self.last_thread.as_mut()
    }
    pub fn set_last_thread(&mut self, thread:Thread){
        self.last_thread = Some(thread);
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
        let list = self.priority_table_heap[highest_ready_priority as usize].as_mut().unwrap();
        let thread = list.pop_front();
        if list.is_empty() {
            self.ready_priority_group &= !(thread.as_ref().unwrap().number_mask() as usize);
        }
        thread
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
    pub fn list_thread<F>(&mut self, mut f: F)
    where
        F: FnMut(&Thread),
    {
        if let Some(thread) = self.current_thread {
            f(&thread);
        }
        if let Some(thread) = self.last_thread {
            f(&thread);
        }
        for threads in self.priority_table_heap.as_ref() {
            if let Some(threads) = threads {
                for thread in threads.clone().into_iter() {
                    f(&thread);
                }
            }
        }
    }
}
