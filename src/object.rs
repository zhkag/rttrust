use crate::list::List;
use crate::system::System;
use crate::thread::Thread;
use crate::system;

#[derive(PartialEq)]
#[derive(Copy, Clone)]
pub enum ObjectClassType
{
    Null          = 0x00,
    Thread        = 0x01,
    Semaphore     = 0x02,
    Mutex         = 0x03,
    Event         = 0x04,
    MailBox       = 0x05,
    MessageQueue  = 0x06,
    MemHeap       = 0x07,
    MemPool       = 0x08,
    Device        = 0x09,
    Timer         = 0x0a,
    Module        = 0x0b,
    Memory        = 0x0c,
    Channel       = 0x0d,
    Custom        = 0x0e,
    Unknown       = 0x0f,
    Static        = 0x80,
}

#[derive(Copy, Clone)]
pub struct ObjectInformation
{
    object_class_type:ObjectClassType,
    object_list:List<Object>,
    object_size:u16,
}

#[derive(Copy, Clone)]
pub struct Object
{
    name:[char;8],                    
    r#type:u8,                              
    flag:u8,                              
    list:List<Self>,
}

impl Object {
    fn init(&mut self, r#type:ObjectClassType,name:&str) {
        if let Some(information) = system!(object_get_information(r#type)) {
            information.object_list.insert_after(&mut self.list);
        }
    }
}

impl ObjectInformation {
    pub fn new() -> Self {
        let information = ObjectInformation{
            object_class_type: ObjectClassType::Null, 
            object_list: List::new(),
            object_size: 0,
        };
        information
    }
    pub fn init(&mut self) -> &mut Self{
        self.object_list.init();
        self
    }
}

impl System {
    pub(crate) fn object_container_init(&mut self) {
        let mut num = 0;

        self.object_container[num].init();
        self.object_container[num].object_class_type = ObjectClassType::Thread;
        self.object_container[num].object_size =  core::mem::size_of::<Thread>().try_into().unwrap();
        num += 1;
    }
    fn object_get_information(&mut self,r#type:ObjectClassType) -> Option<&mut ObjectInformation>{
        for index in 0..8 {
            if self.object_container[index].object_class_type == r#type {
                return Some(&mut self.object_container[index]);
            }
        }
        return None;
    }
}

