pub struct Node<T> {
    value: Option<T>,
    prev: Option<*mut Node<T>>,
    next: Option<*mut Node<T>>,
}

pub struct List<T> {
    head: Option<*mut Node<T>>,
    tail: Option<*mut Node<T>>,
}

impl<T> Node<T> {

    pub fn static_init(static_self:&mut Option<Node<T>>) -> &mut Option<Node<T>>{
        *static_self=Some(Node{value:None,prev:None,next:None});
        static_self  
    }
    pub fn set_value(&mut self,value:T) -> &mut Node<T>{
        self.value = Some(value);
        self  
    }

    pub fn value(&mut self) -> &mut Option<T>{
        &mut self.value
    }
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            head: None,
            tail: None,
        }
    }
    
    pub fn push_front(&mut self, new_node: &mut Node<T>) {
        let raw_node = new_node as *mut Node<T>;
        if let Some(head) = self.head {
            unsafe {
                (*head).prev = Some(raw_node);
                new_node.next = Some(head);
            }
        } else {
            self.tail = Some(raw_node);
        }
        self.head = Some(raw_node);
    }
    
    pub fn push_back(&mut self, new_node: &mut Node<T>) {
        let raw_node = new_node as *mut Node<T>;
        if let Some(tail) = self.tail {
            unsafe {
                (*tail).next = Some(raw_node);
                new_node.prev = Some(tail);
            }
        } else {
            self.head = Some(raw_node);
        }
        self.tail = Some(raw_node);
    }
    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|head: *mut Node<T>| {
            let head = unsafe { &mut *head };
            if let Some(next) = head.next.as_mut() {
                unsafe {
                    (*(*next)).prev = None;
                }
                self.head = Some(*next);
            } else {
                self.tail = None;
            }
            let value: T = unsafe { core::ptr::read((*head).value.as_ref().expect("REASON")) };
            value
        })
    }
    
    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|tail: *mut Node<T>| {
            let tail = unsafe { &mut *tail };
            if let Some(prev) = tail.prev.as_mut() {
                unsafe {
                    (*(*prev)).next = None;
                }
                self.tail = Some(*prev);
            } else {
                self.head = None;
            }
            let value: T = unsafe { core::ptr::read((*tail).value.as_ref().expect("REASON")) };
            value
        })
    }
    
    pub fn remove(&mut self, node: &mut Node<T>) {

        if let Some(prev) = node.prev.as_mut() {
            unsafe {
                (*(*prev)).next = node.next;
            }
        } else {
            self.head = node.next;
        }
        if let Some(next) = node.next.as_mut() {
            unsafe {
                (*(*next)).prev = node.prev;
            }
        } else {
            self.tail = node.prev;
        }
    }

    pub fn isempty(&self) ->bool {
        if self.head.is_none() && self.tail.is_none(){
            return true;
        }
        false
    }

    pub fn len(&self) ->u8 {
        let mut len:u8 = 0;        
        let mut current = self.head.as_ref();

        while let Some(node) = current {
            len += 1;
            unsafe {
                current = (*(*node)).next.as_ref();
            }
        }
        len
    }
    
    pub fn iter(&self) -> LinkedListIterator<T> {
        LinkedListIterator {
            current: self.head,
        }
    }
}

pub struct LinkedListIterator<T> {
    current: Option<*mut Node<T>>,
}

impl<T> Iterator for LinkedListIterator<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.current.take().map(|node| {
            let node = unsafe { &mut *node };
            let next = node.next;
            self.current = next;
            let value:T = unsafe { core::ptr::read((*node).value.as_ref().expect("REASON")) };
            value
        })
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     static mut TEST1: Option<Node<u8>> = None;
//     static mut TEST2: Option<Node<u8>> = None;
//     static mut TEST3: Option<Node<u8>> = None;
//     #[test]
//     fn test_count_nodes() {
//         let mut list: List<u8> = List::new();
//         let test1 = unsafe{Node::static_init(&mut TEST1)};
//         let test2 = unsafe{Node::static_init(&mut TEST2)};
//         let test3 = unsafe{Node::static_init(&mut TEST3)};
//         test1.as_mut().expect("REASON").set_value(1);
//         test2.as_mut().expect("REASON").set_value(2);
//         test3.as_mut().expect("REASON").set_value(3);
//         list.push_front(test1.as_mut().expect("REASON"));
//         list.push_front(test2.as_mut().expect("REASON"));
//         list.push_front(test3.as_mut().expect("REASON"));
        
//         let mut _len = list.len();
//         list.remove(test3.as_mut().expect("REASON"));
//         _len = list.len();
//         let mut _value_sum = 0;
//         for value in list.iter() {
//             _value_sum += value;
//         }
        
//         while let Some(value) = list.pop_front() { //pop_back  pop_front
//             _value_sum += value;
//         }
//         let _isempty = list.isempty();
//     }
// }
