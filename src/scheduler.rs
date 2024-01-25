use crate::thread::{Thread,Status};
use crate::list::List;
use crate::thread_self;
use crate::scheduler;
use crate::libcpu;
use crate::interrupt_nest;
use crate::include::{*};

#[derive(Copy, Clone)]
pub struct Scheduler{
    priority_table:[List<Thread>;THREAD_PRIORITY_MAX],
    ready_priority_group:usize,
    current_thread:Option<*mut Thread>
}

impl Scheduler {
    pub fn new() -> Self {
        let scheduler = Self{
            priority_table:[List::new();THREAD_PRIORITY_MAX],
            ready_priority_group:0,
            current_thread:None
        };
        scheduler
    }
    pub fn init(&mut self){
        for value in 0..THREAD_PRIORITY_MAX{
            self.priority_table[value].init();
        }
    }

    // pub fn current_thread(&self)->Option<&Thread>{
    //     self.current_thread
    // }
    pub fn schedule(&mut self){ //rt_schedule
        if self.ready_priority_group == 0 {
            return;
        }
        let level = libcpu::interrupt_disable();
         /* need_insert_from_thread: need to insert from_thread to ready queue */
        let mut need_insert_from_thread = false;
        let mut highest_ready_priority = 0;
        let mut to_thread = self.get_highest_priority_thread_mut(&mut highest_ready_priority);
        let current_thread = thread_self!().unwrap();

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
        if to_thread != thread_self!().unwrap()
        {
            /* if the destination thread is not the same as current thread */
            let from_thread = thread_self!().unwrap();
            if (from_thread.stat() & Status::STAT_YIELD_MASK as u8) != 0{
                from_thread.set_stat(from_thread.stat() & !(Status::STAT_YIELD_MASK as u8));
            }

            scheduler!(set_current_thread(Some(to_thread)));
            if need_insert_from_thread {
                scheduler!(insert_thread(from_thread));
            }

            scheduler!(remove_thread(to_thread));
            to_thread.set_stat(Status::Running as u8 | (to_thread.stat() & !(Status::StatMask as u8)));

            let from_sp = (from_thread.sp_mut()) as *mut *mut () as *mut ();
            let to_sp = (to_thread.sp_mut()) as *mut *mut () as *mut ();

            if interrupt_nest!() == 0 {
                libcpu::rt_hw_context_switch(from_sp, to_sp);
                libcpu::interrupt_enable(level);
                return;
            }
            else {
                libcpu::rt_hw_context_switch_interrupt(from_sp,to_sp,from_thread,to_thread);
            }

        }else {
            scheduler!(remove_thread(thread_self!().unwrap()));
            current_thread.set_stat(Status::Running as u8 | (current_thread.stat() & !(Status::StatMask as u8)));
        }
        libcpu::interrupt_enable(level);

    }
    // pub fn hardware(&self)->&HardWare{
    //     self.hw
    // }

    
    pub fn insert_thread(&mut self,thread:&mut Thread){
        let level = libcpu::interrupt_disable();
        if let Some(current_thread) = self.current_thread() {
            if thread == current_thread {
                thread.set_stat(Status::Running as u8|thread.stat() & !(Status::StatMask as u8));
                libcpu::interrupt_enable(level);
                return;
            }
        }
        thread.set_stat(Status::Ready as u8 | (thread.stat() & !(Status::StatMask as u8)));
        if (thread.stat() & (Status::STAT_YIELD_MASK as u8)) != 0 {
            self.priority_table[thread.current_priority() as usize].insert_before(thread.list_mut());
        }else {
            self.priority_table[thread.current_priority() as usize].insert_after(thread.list_mut());
        }
        self.ready_priority_group |= thread.number_mask() as usize;
        libcpu::interrupt_enable(level);
    }
    pub fn remove_thread(&mut self, thread:&mut Thread){
        let level = libcpu::interrupt_disable();
        thread.list_mut().remove();
        if self.priority_table[thread.current_priority() as usize].isempty() {
            self.ready_priority_group &= !(thread.number_mask() as usize);
        }
        libcpu::interrupt_enable(level);
    }

    pub fn current_thread(&mut self) -> Option<&mut Thread> {
        if let Some(thread) = self.current_thread{
            return Some(unsafe {&mut *(thread)});
        }
        return None;
    }
    pub fn set_current_thread(&mut self, thread:Option<*mut Thread>){
        self.current_thread = thread;
    }

    fn get_highest_priority_thread_mut(&mut self,highest_prio: &mut u8) -> &mut Thread {
        let highest_ready_priority:u8 = self.ready_priority_group.trailing_zeros() as u8;
        *highest_prio = highest_ready_priority;
        let node = self.priority_table[highest_ready_priority as usize].iter_mut().next().expect("REASON");
        self.list_to_thread(node)
    }

    pub fn start(&mut self) {
        let mut highest_ready_priority = 0;
        let to_thread = self.get_highest_priority_thread_mut(&mut highest_ready_priority);
        scheduler!(remove_thread(to_thread));
        to_thread.set_stat(Status::Running as u8);
        scheduler!(set_current_thread(Some(to_thread)));
        let sp = to_thread.sp_mut() as *mut *mut () as *mut ();
        libcpu::rt_hw_context_switch_to(sp);
        unreachable!();
    }
}
