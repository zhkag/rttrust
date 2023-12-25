use crate::thread::{Thread,Status};
use crate::list::List;
use crate::context;
use crate::{offset_of,offset_of_mut,thread_self};
use crate::scheduler;

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
    pub fn schedule(&mut self){ //rt_schedule
        if self.ready_priority_group == 0 {
            return;
        }
        // let level = unsafe{context::rt_hw_interrupt_disable()};
         /* need_insert_from_thread: need to insert from_thread to ready queue */
         let mut need_insert_from_thread = false;
         let mut highest_ready_priority = 0;
         let mut to_thread = self.get_highest_priority_thread_mut(&mut highest_ready_priority);
         let current_thread = thread_self!().unwrap();

         if (current_thread.stat() & Status::STAT_MASK as u8) == Status::RUNNING as u8 {
             if current_thread.current_priority() < highest_ready_priority {
                 to_thread = current_thread;
             }
             else if current_thread.current_priority() == highest_ready_priority && (current_thread.stat() & Status::STAT_YIELD_MASK as u8) == 0 {
                 to_thread = current_thread;
             }
             else {
                 need_insert_from_thread = true;
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
                    // self.insert_thread(from_thread);
                    scheduler!(insert_thread(from_thread));
                }

                scheduler!(remove_thread(to_thread));
                to_thread.set_stat(Status::RUNNING as u8 | (to_thread.stat() & !(Status::STAT_MASK as u8)));


                let from_sp = (&mut from_thread.sp()) as *mut *mut () as *mut ();
                let to_sp = (&mut to_thread.sp()) as *mut *mut () as *mut ();

                unsafe{context::rt_hw_context_switch_interrupt(from_sp,to_sp,from_thread,to_thread);};

             }else {
                scheduler!(remove_thread(thread_self!().unwrap()));
                current_thread.set_stat(Status::RUNNING as u8 | (current_thread.stat() & !(Status::STAT_MASK as u8)));
             }
         }


    }
    // pub fn hardware(&self)->&HardWare{
    //     self.hw
    // }

    
    pub fn insert_thread(&mut self,thread:&mut Thread){
        // let level = unsafe{context::rt_hw_interrupt_disable()};
        if let Some(current_thread) = self.current_thread() {
            if thread == current_thread {
                thread.set_stat(Status::RUNNING as u8|thread.stat() & !(Status::STAT_MASK as u8));
                // unsafe {context::rt_hw_interrupt_enable(level)};
                return;
            }
        }
        thread.set_stat(Status::READY as u8 | (thread.stat() & !(Status::STAT_MASK as u8)));
        if (thread.stat() & (Status::STAT_YIELD_MASK as u8)) != 0 {
            self.priority_table[thread.current_priority() as usize].push_back(&mut thread.list);
        }else {
            self.priority_table[thread.current_priority() as usize].push_front(&mut thread.list);
        }
        self.ready_priority_group |= thread.number_mask() as usize;
        // unsafe {context::rt_hw_interrupt_enable(level)};
    }
    pub fn remove_thread(&mut self, thread:&mut Thread){
        // let level = unsafe{context::rt_hw_interrupt_disable()};
        self.priority_table[thread.current_priority() as usize].remove(&mut thread.list);
        if self.priority_table[thread.current_priority() as usize].isempty() {
            self.ready_priority_group &= !(thread.number_mask() as usize);
        }
        // unsafe {context::rt_hw_interrupt_enable(level)};
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

    fn get_highest_priority_thread_mut(&self,highest_prio: &mut u8) -> &mut Thread {
        let highest_ready_priority:u8 = self.ready_priority_group.trailing_zeros() as u8;
        *highest_prio = highest_ready_priority;
        let node = self.priority_table[highest_ready_priority as usize].iter_mut().next().expect("REASON");
        offset_of_mut!(node,Thread,list)
    }
    // fn get_highest_priority_thread(&self,highest_prio: &mut usize) -> Thread {
    //     let highest_ready_priority:usize = self.ready_priority_group.trailing_zeros() as usize;
    //     *highest_prio = highest_ready_priority;
    //     let node = self.priority_table[highest_ready_priority].iter_mut().next().expect("REASON");
    //     offset_of!(node,Thread,list)
    // }

    pub fn start(&mut self) {
        let mut highest_ready_priority = 0;
        let to_thread = self.get_highest_priority_thread_mut(&mut highest_ready_priority);
        let sp = &mut to_thread.sp();
        scheduler!(remove_thread(to_thread));
        to_thread.set_stat(Status::RUNNING as u8);
        self.set_current_thread(Some(to_thread));
        unsafe{context::rt_hw_context_switch_to(sp as *mut *mut () as *mut ());};
        unreachable!();
    }
}
