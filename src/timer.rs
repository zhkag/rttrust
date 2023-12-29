use crate::system;
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
        timer.as_mut().unwrap()
    }
    pub fn start(&mut self){
        self.timeout_tick = tick!(get()) + self.init_tick;
        let timer_list = system!(timer_list_mut());
        if timer_list.isempty() {
            timer_list.push_front(&mut self.list);
        }
        for node in timer_list.iter_mut() {
            let timer = Self::list_to_timer(node);
            if self.timeout_tick > timer.timeout_tick {
                continue;
            }
            else {
                
            }
        }
    }

    pub fn list_mut(&mut self) -> &mut List<Self> {
        &mut self.list
    }

    pub fn list_to_timer(list: *mut List<Self>) -> &'static mut Self {
        #[allow(deref_nullptr)]
        unsafe { &mut *((list as usize - (&(&*(0 as *const Self)).list) as *const List<Self> as usize) as *mut Self) }
    }

    pub fn check(tick:usize){

    }
}

