#[derive(PartialEq)]
#[derive(Copy, Clone)]
pub struct List<T> {
    next: Option<*mut List<T>>,
    prev: Option<*mut List<T>>,
}

impl<T> List<T> {

    pub fn static_init(static_self:&mut Option<List<T>>) -> &mut Option<List<T>>{
        *static_self=Some(List{prev:None,next:None});
        static_self  
    }

    pub fn init() -> List<T>{
        List{prev:None,next:None}
    }
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            next: None,
            prev: None,
        }
    }
    
    pub fn push_front(&mut self, new_node: &mut List<T>) {
        let raw_node = new_node as *mut List<T>;
        if let Some(next) = self.next {
            unsafe {
                (*next).prev = Some(raw_node);
                new_node.next = Some(next);
            }
        } else {
            self.prev = Some(raw_node);
        }
        self.next = Some(raw_node);
    }
    
    pub fn push_back(&mut self, new_node: &mut List<T>) {
        let raw_node = new_node as *mut List<T>;
        if let Some(tail) = self.prev {
            unsafe {
                (*tail).next = Some(raw_node);
                new_node.prev = Some(tail);
            }
        } else {
            self.next = Some(raw_node);
        }
        self.prev = Some(raw_node);
    }
    pub fn pop_front(&mut self) -> Option<*mut List<T>> {
        self.next.take().map(|head: *mut List<T>| {
            if let Some(next) = unsafe { &mut *head }.next.as_mut() {
                unsafe {
                    (*(*next)).prev = None;
                }
                self.next = Some(*next);
            } else {
                self.prev = None;
            }
            head
        })
    }
    
    pub fn pop_back(&mut self) -> Option<*mut List<T>> {
        self.prev.take().map(|tail: *mut List<T>| {
            if let Some(prev) = unsafe { &mut *tail }.prev.as_mut() {
                unsafe {
                    (*(*prev)).next = None;
                }
                self.prev = Some(*prev);
            } else {
                self.next = None;
            }
            tail
        })
    }
    
    pub fn remove(&mut self, node: &mut List<T>) {

        if let Some(prev) = node.prev.as_mut() {
            unsafe {
                (*(*prev)).next = node.next;
            }
        } else {
            self.next = node.next;
        }
        if let Some(next) = node.next.as_mut() {
            unsafe {
                (*(*next)).prev = node.prev;
            }
        } else {
            self.prev = node.prev;
        }
    }

    pub fn isempty(&self) ->bool {
        if self.next.is_none() && self.prev.is_none(){
            return true;
        }
        false
    }

    pub fn len(&self) ->u8 {
        let mut len:u8 = 0;        
        let mut current = self.next.as_ref();

        while let Some(node) = current {
            len += 1;
            unsafe {
                current = (*(*node)).next.as_ref();
            }
        }
        len
    }

    pub fn iter_mut(&self) -> LinkedListIteratorMut<T> {
        LinkedListIteratorMut {
            current: self.next,
        }
    }
}

pub struct LinkedListIteratorMut<T> {
    current: Option<*mut List<T>>,
}

impl<T> Iterator for LinkedListIteratorMut<T> {
    type Item = *mut List<T>;
    fn next(&mut self) -> Option<Self::Item> {
        self.current.take().map(|node| {
            let next: Option<*mut List<T>> = unsafe { &mut *node }.next;
            self.current = next;
            node
        })
    }
}

#[macro_export]
macro_rules! offset_of_mut {
    ($node:ident, $type:ty, $member:ident) => {{
        #[allow(deref_nullptr)]
        unsafe { &mut *(($node as usize - (&(&*(0 as *const $type)).$member) as *const List<$type> as usize) as *mut $type) }
    }};
}

#[macro_export]
macro_rules! offset_of {
    ($node:ident, $type:ty, $member:ident) => {{
        #[allow(deref_nullptr)]
        unsafe { *(($node as usize - (&(&*(0 as *const $type)).$member) as *const List<$type> as usize) as *mut $type) }
    }};
}

// struct TestListU8
// {
//     value:u8,
//     list:List<TestListU8>
// }
// impl TestListU8 {

//     pub fn static_init(static_self:&mut Option<TestListU8>) -> &mut Option<TestListU8>{
//         *static_self=Some(TestListU8{
//             value:0,
//             list:List::init()
//         });
//         static_self  
//     }

//     pub fn set_value(&mut self,value:u8) -> &mut TestListU8{
//         self.value = value;
//         self  
//     }

//     pub fn value(&self) -> &u8{
//         &self.value
//     }

//     pub fn list(&mut self) -> &mut List<TestListU8> {
//         &mut self.list
//     }
// }

// static mut TEST1: Option<TestListU8> = None;
// static mut TEST2: Option<TestListU8> = None;
// static mut TEST3: Option<TestListU8> = None;


// fn main() {
//     let mut list: List<TestListU8> = List::new();
//     let test1 = unsafe{TestListU8::static_init(&mut TEST1)};
//     let test2 = unsafe{TestListU8::static_init(&mut TEST2)};
//     let test3 = unsafe{TestListU8::static_init(&mut TEST3)};

//     test1.as_mut().expect("REASON").set_value(1);
//     test2.as_mut().expect("REASON").set_value(2);
//     test3.as_mut().expect("REASON").set_value(3);
//     test1.as_mut().expect("REASON").list();
//     list.push_front(test1.as_mut().expect("REASON").list());
//     list.push_front(test2.as_mut().expect("REASON").list());
//     list.push_front(test3.as_mut().expect("REASON").list());
    
//     let mut _len = list.len();
//     println!("_len : {}",_len);
//     list.remove(test3.as_mut().expect("REASON").list());
//     _len = list.len();
//     println!("_len : {}",_len);
//     let mut _value_sum = 0;
//     for value in list.iter_mut() {
//         _value_sum += offset_of!(value,TestListU8,list).value();

//         println!("_value_sum : {}",_value_sum);
//     }
    
//     while let Some(value) = list.pop_front() { //pop_back  pop_front
//         _value_sum += offset_of!(value,TestListU8,list).value();
//         println!("_value_sum : {}",_value_sum);
//     }
//     let _isempty = list.isempty();
//     println!("_isempty : {}",_isempty);
// }
