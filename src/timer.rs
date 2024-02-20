use crate::system;
use crate::system::System;
use crate::scheduler::Scheduler;
use crate::tick;
use crate::list::List;

#[derive(PartialEq)]
#[derive(Copy, Clone)]
pub struct Timer
{
    timeout_func:fn(*mut ()),
    parameter: *mut (),
    init_tick:usize,
    timeout_tick:usize,
    list:List<Self>,
    flag:u8,
}

impl Timer {
    pub fn init(timer: &mut Option<Self>, timeout_func: fn(*mut ()), parameter:*mut (), time:usize, flag:u8) -> &mut Self{
        let timer_init = Self {
            timeout_func,
            parameter,
            init_tick:time,
            timeout_tick:0,
            list:List::new(),
            flag,
        };
        *timer = Some(timer_init);
        timer.as_mut().unwrap().list_mut().init();
        timer.as_mut().unwrap()
    }

    pub fn timeout_tick(&self) -> usize{
        self.timeout_tick
    }

    pub fn start(&mut self){
        let libcpu = system!().libcpu();
        let level = libcpu.interrupt_disable();
        self.timeout_tick = tick!(get()) + self.init_tick;
        libcpu.interrupt_enable(level);
    }

    pub fn control(&mut self,tick:usize){
        self.init_tick = tick;
    }

    pub fn list_mut(&mut self) -> &mut List<Self> {
        &mut self.list
    }
}

impl System {
    pub fn list_to_timer(&self, list: *mut List<Timer>) -> &mut Timer {
        #[allow(deref_nullptr)]
        unsafe { &mut *((list as usize - (&(&*(0 as *const Timer)).list) as *const List<Timer> as usize) as *mut Timer) }
    }

    pub fn timer_check(&mut self, tick:usize){
        self.scheduler_mut().thread_timer_check(tick); // 这里只处理了线程定时器，没有原生定时器
    }
}

use crate::alloc::boxed::Box;

impl Scheduler {
    pub fn thread_timer_check(&mut self, tick:usize){
        let list = self.thread_timer_list_mut();
        if let Some(mut thread) = list.pop_front() {
            if tick > thread.thread_timer_mut().timeout_tick{
                let timer_parameter = Box::into_raw(Box::new(thread)) as *mut ();
                (thread.thread_timer_mut().timeout_func)(timer_parameter);
            }else {
                list.push_front(thread);
            }
        }
    }
}
