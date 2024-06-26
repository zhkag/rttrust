use crate::system::System;
use crate::system;
use crate::include::NAME_MAX;

#[allow(dead_code)]
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

#[allow(dead_code)]
#[repr(C)]
pub enum ObjectInfoType
{
    Thread = 0,                         //< The object is a thread. */
    Semaphore,                          //< The object is a semaphore. */
    Mutex,                              //< The object is a mutex. */
    Event,                              //< The object is a event. */
    MailBox,                            //< The object is a mail box. */
    MessageQueue,                       //< The object is a message queue. */
    MemHeap,                            //< The object is a memory heap */
    MemPool,                            //< The object is a memory pool. */
    Device,                             //< The object is a device */
    Timer,                              //< The object is a timer. */
    Module,                             //< The object is a module. */
    Memory,                            //< The object is a memory. */
    Channel,                            //< The object is a IPC channel */
    Custom,                             //< The object is a custom object */
    Unknown,                            //< The object is unknown. */
}

impl core::fmt::Display for ObjectClassType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Type:")?;
        let str = match self {
            Self::Null => "Null",
            Self::Thread => "thread",
            Self::Timer => "timer",
            _ => "Unknown"
        };
        write!(f, "{}",str)
    }
}

#[derive(Copy, Clone)]
pub struct ObjectInformation
{
    object_class_type:ObjectClassType,
    object_size:u16,
}

#[derive(PartialEq)]
#[derive(Copy, Clone)]
pub struct Object
{
    name:[u8;NAME_MAX],
    r#type:ObjectClassType,
    flag:u8,
}

impl core::fmt::Display for Object {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "[")?;
        for byte in self.name.iter() {
            write!(f, "{}", *byte as char)?;
        }
        write!(f, " {}", self.r#type)?;
        write!(f, "]")
    }
}

impl Object {
    pub fn new() -> Self{
        let object = Self{
            name: [b'\0'; NAME_MAX],
            r#type: ObjectClassType::Null,
            flag: 0,
        };
        object
    }
    pub fn init(&mut self, r#type:ObjectClassType, name:&str) {
        self.r#type = r#type;
        for index in 0..NAME_MAX {
            if let Some(char) = name.as_bytes().get(index){
                self.name[index] = *char;
            }else {
                break;
            }
        }
    }
}

impl ObjectInformation {
    pub fn new() -> Self {
        let information = ObjectInformation{
            object_class_type: ObjectClassType::Null,
            object_size: 0,
        };
        information
    }
    pub fn init(&mut self, r#type:ObjectClassType, size:u16){
        self.object_class_type = r#type;
        self.object_size =  size;
    }
}

impl System {
    pub fn get_object_information(&mut self, r#type:ObjectClassType) -> Option<&mut ObjectInformation>{
        let libcpu = system!().libcpu();
        let level = libcpu.interrupt_disable();
        for index in 0..ObjectInfoType::Unknown as usize {
            if self.object_container[index].object_class_type == r#type {
                libcpu.interrupt_enable(level);
                return Some(&mut self.object_container[index]);
            }
        }
        libcpu.interrupt_enable(level);
        None
    }
}

