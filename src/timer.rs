use crate::system;
use crate::system::System;
use crate::tick;
use crate::list::List;
use crate::libcpu;

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
    pub fn start(&mut self){
        let level = libcpu::interrupt_disable();
        self.timeout_tick = tick!(get()) + self.init_tick;
        let system = system!();
        let timer_list = system.timer_list_mut();
        let mut current = timer_list as *mut List<Self>;
        for node in timer_list.iter_mut() {
            current = node;
            let timer = system.list_to_timer(node);
            if self.timeout_tick > timer.timeout_tick {
                continue;
            }
            break;
        }
        unsafe{&mut *current}.insert_after(&mut self.list);
        libcpu::interrupt_enable(level);
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

    pub fn check(&self, tick:usize){
        let level = libcpu::interrupt_disable();
        let timer_list = system!(timer_list_mut());
        let mut _current = timer_list as *mut List<Timer>;
        for node in timer_list.iter_mut() {
            _current = node;
            let timer = self.list_to_timer(node);
            if tick >= timer.timeout_tick {
                timer.list_mut().remove();
                (timer.timeout_func)(timer.parameter);
            }
        }
        libcpu::interrupt_enable(level);
    }
}
