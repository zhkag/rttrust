pub struct Scheduler{
    ready_priority_group:u32,
}

impl Scheduler {
    pub fn new() -> Scheduler {
        let scheduler = Scheduler{
            ready_priority_group:0
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
    pub fn init(&self) {
    }
    pub fn start(&self) {

        unreachable!();
    }
}
