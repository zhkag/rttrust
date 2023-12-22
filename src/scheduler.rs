use crate::thread::{Thread,Status};
use crate::list::List;
use crate::context;
use crate::{offset_of,offset_of_mut};

const THREAD_PRIORITY_MAX: usize = 32;

#[derive(Copy, Clone)]
pub struct Scheduler{
    priority_table:[List<Thread>;THREAD_PRIORITY_MAX],
    ready_priority_group:usize,
    current_thread:Option<*mut Thread>
}

impl Scheduler {
    pub fn new() -> Scheduler {
        let scheduler = Scheduler{
            priority_table:[List::new();THREAD_PRIORITY_MAX],
            ready_priority_group:0,
            current_thread:None
        };
        scheduler
    }

    // pub fn current_thread(&self)->Option<&Thread>{
    //     self.current_thread
    // }
    pub fn schedule(&self){ //rt_schedule
        if self.ready_priority_group == 0
        {
            return;
        }

        let mut highest_ready_priority = 0;
        let to_thread = self.get_highest_priority_thread_mut(&mut highest_ready_priority);

    }
    // pub fn hardware(&self)->&HardWare{
    //     self.hw
    // }

    
    pub fn insert_thread(&mut self,thread:&mut Thread){
        if let Some(current_thread) = self.current_thread() {
            if thread == current_thread {
                thread.set_stat(Status::RUNNING|thread.stat() & !Status::STAT_MASK);
                return;
            }
        }
        self.priority_table[thread.current_priority() as usize].push_front(&mut thread.list);
        self.ready_priority_group |= thread.number_mask() as usize;
    }
    pub fn remove_thread(&self,thread:&mut Thread){
        // thread.list.
        // self.ready_priority_group&= ~thread->number_mask
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
    pub fn init(&self) {
    }

    fn get_highest_priority_thread_mut(&self,highest_prio: &mut usize) -> &mut Thread {
        let highest_ready_priority:usize = self.ready_priority_group.trailing_zeros() as usize;
        *highest_prio = highest_ready_priority;
        let node = self.priority_table[highest_ready_priority].iter_mut().next().expect("REASON");
        offset_of_mut!(node,Thread,list)
    }
    fn get_highest_priority_thread(&self,highest_prio: &mut usize) -> Thread {
        let highest_ready_priority:usize = self.ready_priority_group.trailing_zeros() as usize;
        *highest_prio = highest_ready_priority;
        let node = self.priority_table[highest_ready_priority].iter_mut().next().expect("REASON");
        offset_of!(node,Thread,list)
    }

    pub fn start(&mut self) {
        let mut highest_ready_priority = 0;
        let to_thread = self.get_highest_priority_thread_mut(&mut highest_ready_priority);
        let sp = &mut to_thread.sp();
        to_thread.set_stat(Status::RUNNING);
        self.remove_thread(to_thread);
        self.set_current_thread(Some(to_thread));
        unsafe{context::rt_hw_context_switch_to(sp as *mut *mut () as *mut ());};
        unreachable!();
    }
}
