use crate::Box;

pub trait BspTrait {
    fn init(&self);
    fn putc(&mut self,  c: char);
}

impl crate::system::System {
    pub fn bsp(&mut self) -> &mut Box<dyn BspTrait>{
        self.bsp.as_mut().unwrap()
    }
    pub fn bsp_trait_init(&mut self,item: impl BspTrait + 'static) {
        self.bsp = Some(Box::new(item));
    }
}
