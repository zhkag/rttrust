use crate::Box;

pub trait BspTrait {
    fn init(&self);
}

impl crate::system::System<'_> {
    pub fn bsp(&mut self) -> Option<&mut Box<dyn BspTrait>>{
        self.bsp.as_mut()
    }
    pub fn bsp_trait_init(&mut self,item: impl BspTrait + 'static) {
        self.bsp = Some(Box::new(item));
    }
}
