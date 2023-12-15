use crate::scheduler::Scheduler;
use crate::hw::HardWare;

static mut SYSTREM: Option<System> = None;

pub struct System{
    scheduler:Option<Scheduler>,
}

impl System {
    pub fn global() -> &'static mut System{
        unsafe {
            if (&mut SYSTREM).is_none(){
                SYSTREM = Some(System::new());
            }
            return SYSTREM.as_mut().unwrap();
        }
    }
    fn new() -> System {
        let systerm = System{
            scheduler:None,
        };
        systerm
    }
    fn init(&mut self)  {
        HardWare::board_init();
        self.scheduler = Some(Scheduler::new());
        self.scheduler().init();
        // rt_system_timer_init();
        // rt_system_scheduler_init();
        crate::idle::rt_thread_idle_init();
    }
    pub fn scheduler(&mut self) ->&mut Scheduler {
        self.scheduler.as_mut().unwrap()
    }

    pub fn startup(&mut self) {
        self.init();

        self.scheduler().start();
        unreachable!();
    }
}

#[macro_export]
macro_rules! system {
    ($($tokens:tt)*) => {{
        crate::system::System::global().$($tokens)*;
    }};
}

#[macro_export]
macro_rules! schedule {
    ()=>{crate::system::System::global().scheduler().schedule();}
}

