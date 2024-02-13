pub trait BspTrait {
    fn init(&self);
    fn putc(&self,  c: char);
}

impl crate::system::System {
    pub fn bsp(&self) -> &dyn BspTrait{
        unsafe { &*(self.bsp.unwrap())}
    }
    pub fn bsp_trait_init(&mut self,item: *mut dyn BspTrait) {
        self.bsp = Some(item);
    }
}
