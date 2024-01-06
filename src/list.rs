#[derive(PartialEq)]
#[derive(Copy, Clone)]
pub struct List<T> {
    next: *mut Self,
    prev: *mut Self,
}

impl<T> List<T> {
    pub fn new() -> Self {
        let list = List{next: core::ptr::null_mut(), prev: core::ptr::null_mut()};
        list
    }
    pub fn init(&mut self) -> &mut Self {
        self.prev = self as *mut Self;
        self.next = self as *mut Self;
        self
    }

    pub fn insert_after(&mut self, node: &mut Self) {
        unsafe {&mut *self.next}.prev = node as *mut Self;
        node.next = self.next;
        self.next = node as *mut Self;
        node.prev = self as *mut Self;
    }
    pub fn insert_before(&mut self, node: &mut Self) {
        unsafe {&mut *self.prev}.next = node as *mut Self;
        node.prev = self.prev;
        self.prev = node as *mut Self;
        node.next = self as *mut Self;
    }
    
    pub fn remove(&mut self) {
        unsafe {&mut *self.next}.prev = self.prev;
        unsafe {&mut *self.prev}.next = self.next;
        self.next = self as *mut Self;
        self.prev = self as *mut Self;
    }

    pub fn isempty(&mut self) ->bool {
        if self.next == self as *mut Self{
            return true;
        }
        false
    }

    pub fn len(&mut self) ->u8 {
        let mut len:u8 = 0;
        let mut current = unsafe {&mut *(self as *mut Self)};
        while current.next != self as *mut Self{
            current = unsafe {&mut *current.next};
            len += 1;
        }
        len
    }

    pub fn iter_mut(&mut self) -> LinkedListIteratorMut<T> {
        LinkedListIteratorMut {
            head:self,
            prev: self.prev,
        }
    }
    
}

pub struct LinkedListIteratorMut<T> {
    head: *mut List<T>,
    prev: *mut List<T>,
}

impl<T> Iterator for LinkedListIteratorMut<T> {
    type Item = *mut List<T>;
    fn next(&mut self) -> Option<Self::Item> {
        let current = unsafe {&mut *self.prev}.next;
        let next = unsafe {&mut *current}.next;
        if next != self.head as Self::Item{
            self.prev = current;
            return Some(next);
        }
        None
    }
}
