use crate::alloc::rc::Rc;
use core::cell::RefCell;

struct Node<T> {
    value: T,
    prev: Option<Rc<RefCell<Node<T>>>>,
    next: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Node<T> {
    fn new(value: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            value,
            prev: None,
            next: None,
        }))
    }
}

#[derive(Clone)]
pub struct List<T> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
}

#[allow(dead_code)]
impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None, tail: None }
    }

    pub fn push_front(&mut self, value: T) {
        let new_node = Node::new(value);
        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(new_node.clone());
                new_node.borrow_mut().next = Some(old_head);
                self.head = Some(new_node);
            }
            None => {
                self.head = Some(new_node.clone());
                self.tail = Some(new_node);
            }
        }
    }

    pub fn push_back(&mut self, value: T) {
        let new_node = Node::new(value);
        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_node.clone());
                new_node.borrow_mut().prev = Some(old_tail);
                self.tail = Some(new_node);
            }
            None => {
                self.head = Some(new_node.clone());
                self.tail = Some(new_node);
            }
        }
    }

    pub fn insert_with_cmp<F>(&mut self, value: T, mut compare: F)
    where
        F: FnMut(&T, &T) -> bool,
    {
        let mut current = self.head.clone();
        while let Some(node) = current {
            if compare(&value, &node.borrow().value) {
                let new_node = Node::new(value);
                let prev = node.borrow_mut().prev.take();
                if let Some(prev_node) = prev {
                    prev_node.borrow_mut().next = Some(new_node.clone());
                    new_node.borrow_mut().prev = Some(prev_node);
                } else {
                    // 如果没有前驱节点，说明当前节点是头部节点
                    self.head = Some(new_node.clone());
                }
                new_node.borrow_mut().next = Some(node.clone());
                node.borrow_mut().prev = Some(new_node);
                return;
            }
            current = node.borrow().next.clone();
        }
        // 如果遍历完链表仍未找到合适位置，则将节点插入到链表尾部
        self.push_back(value);
    }

    pub fn pop_with_cmp<V,C,F>(&mut self, value: &V, mut compare: C,mut f: F)
    where
        C: FnMut(&V, &mut T) -> bool,
        F: FnMut(T),
    {
        while let Some(old_head) = self.head.take() {
            if compare(&value, &mut (&mut *old_head.borrow_mut()).value) {
                if let Some(new_head) = old_head.borrow_mut().next.take() {
                    new_head.borrow_mut().prev = None;
                    self.head = Some(new_head);
                } else {
                    self.tail = None;
                }
                f(Rc::try_unwrap(old_head).ok().unwrap().into_inner().value);
            } else {
                self.head = Some(old_head);
                break;
            }
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            if let Some(new_head) = old_head.borrow_mut().next.take() {
                new_head.borrow_mut().prev = None;
                self.head = Some(new_head);
            } else {
                self.tail = None;
            }
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().value
        })
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|old_tail| {
            if let Some(new_tail) = old_tail.borrow_mut().prev.take() {
                new_tail.borrow_mut().next = None;
                self.tail = Some(new_tail);
            } else {
                self.head = None;
            }
            Rc::try_unwrap(old_tail).ok().unwrap().into_inner().value
        })
    }
    pub fn len(&self) -> usize {
        let mut count = 0;
        let mut current = self.head.clone(); // 从链表头部开始
        while let Some(node) = current {
            count += 1;
            current = node.borrow().next.clone(); // 移动到下一个节点
        }
        count
    }
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

}

pub struct Iter<T> {
    current: Option<Rc<RefCell<Node<T>>>>,
}

impl<T: Clone> IntoIterator for List<T> {
    type Item = T;
    type IntoIter = Iter<T>;

    fn into_iter(self) -> Self::IntoIter {
        Iter { current: self.head }
    }
}

impl<T: Clone> Iterator for Iter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let current_ref = self.current.take();
        if let Some(current_ref) = current_ref {
            let current = current_ref.borrow();
            let value = current.value.clone();
            self.current = current.next.clone();
            Some(value)
        } else {
            None
        }
    }
}
