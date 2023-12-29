#[derive(PartialEq)]
#[derive(Copy, Clone)]
pub struct List<T> {
    next: Option<*mut Self>,
    prev: Option<*mut Self>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        let mut list = List{next: None, prev: None};
        list.prev = Some(&mut list as *mut Self);
        list.next = Some(&mut list as *mut Self);
        list
    }

    pub fn insert_after(&mut self, node: &mut Self) {
        unsafe {&mut *self.next.unwrap()}.prev = Some(node as *mut Self);
        node.next = Some(self.next.unwrap());
        self.next = Some(node as *mut Self);
        node.prev = Some(self as *mut Self);
    }
    pub fn insert_before(&mut self, node: &mut Self) {
        unsafe {&mut *self.prev.unwrap()}.next = Some(node as *mut Self);
        node.prev = Some(self.prev.unwrap());
        self.prev = Some(node as *mut Self);
        node.next = Some(self as *mut Self);
    }
    
    pub fn remove(&mut self) {
        unsafe {&mut *self.next.unwrap()}.prev = Some(self.prev.unwrap());
        unsafe {&mut *self.prev.unwrap()}.next = Some(self.next.unwrap());
        self.next = Some(self as *mut Self);
        self.prev = Some(self as *mut Self);
    }

    pub fn isempty(&mut self) ->bool {
        if self.next.unwrap() == self{
            return true;
        }
        false
    }

    pub fn len(&mut self) ->u8 {
        let mut len:u8 = 0;
        let mut current = unsafe {&mut *(self as *mut Self)};
        while current.next.unwrap() != self as *mut Self{
            current = unsafe {&mut *current.next.unwrap()};
            len += 1;
        }
        len
    }

    pub fn iter_mut(&mut self) -> LinkedListIteratorMut<T> {
        LinkedListIteratorMut {
            head:self,
            current: self,
        }
    }
    
}

pub struct LinkedListIteratorMut<T> {
    head:*mut List<T>,
    current: *mut List<T>,
}

impl<T> Iterator for LinkedListIteratorMut<T> {
    type Item = *mut List<T>;
    fn next(&mut self) -> Option<Self::Item> {
        let current = unsafe {&mut *(self.current as Self::Item)}.next.unwrap();
        if current != self.head as Self::Item{
            self.current = current;
            return Some(current);
        }
        None
    }
}
