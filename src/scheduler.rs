use crate::Thread;
use crate::List;
use crate::context;
use crate::offset_of;

const THREAD_PRIORITY_MAX: usize = 32;
pub struct Scheduler{
    priority_table:[List<Thread>;THREAD_PRIORITY_MAX],
    ready_priority_group:usize,
    current_thread:Option<Thread>
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

    }
    // pub fn hardware(&self)->&HardWare{
    //     self.hw
    // }

    
    pub fn insert_thread(&self,thread:Thread){

    }
    pub fn remove_thread(&self,thread:Thread){

    }
    pub fn current_thread(&self) ->Option<Thread> {
        self.current_thread
    }
    pub fn init(&self) {
    }

    fn get_highest_priority_thread(&self,highest_prio: &mut usize) -> &mut Thread {
        let highest_ready_priority:usize = self.ready_priority_group.trailing_zeros() as usize;
        *highest_prio = highest_ready_priority;
        let node = self.priority_table[highest_ready_priority].iter_node().next().expect("REASON");
        offset_of!(node,Thread,list)
    }

    pub fn start(&mut self) {
        let mut highest_ready_priority = 0;
        let to_thread = self.get_highest_priority_thread(&mut highest_ready_priority);
        // self.current_thread = Some(*to_thread);
        // self.remove_thread(to_thread);
        unsafe{context::rt_hw_context_switch_to(&mut to_thread.sp() as *mut *mut () as *mut ());};
        unreachable!();
    }
}
