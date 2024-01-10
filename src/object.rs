use crate::list::List;
use crate::system::System;
use crate::thread::Thread;
use crate::system;
use crate::libcpu;

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

#[derive(PartialEq)]
#[derive(Copy, Clone)]
pub struct Object
{
    name:[char;8],                    
    r#type:ObjectClassType,                              
    flag:u8,                              
    list:List<Self>,
}

impl Object {
    pub fn new() -> Self{
        let object = Self{
            name: [' '; 8],
            r#type: ObjectClassType::Null,
            flag: 0,
            list: List::new(), 
        };
        object 
    }
    pub fn init(&mut self, r#type:ObjectClassType, name:&str) {
        self.r#type = r#type;
        self.list.init();
        // self.name = {
        //     let mut arr: [char; 8] = [' '; 8];
        //     for (i, c) in name.chars().take(8).enumerate() {
        //         arr[i] = c;
        //     }
        //     arr
        // };
        system!(install_object(r#type,&mut self.list));
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
    pub fn init(&mut self, r#type:ObjectClassType, size:u16){
        self.object_list.init();
        self.object_class_type = r#type;
        self.object_size =  size;
    }
}

impl System {
    fn install_object(&mut self, r#type:ObjectClassType, list:&mut List<Object>){
        let level = libcpu::interrupt_disable();
        for index in 0..8 {
            if self.object_container[index].object_class_type == r#type {
                self.object_container[index].object_list.insert_after(list);
                libcpu::interrupt_enable(level);
                return;
            }
        }
        libcpu::interrupt_enable(level);
    }
}

