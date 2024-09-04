// fifth.rs: Unsafe Queue

use std::ptr;

pub struct List<T> {
    head: Link<T>,
    tail: *mut Node<T>,
    count: usize,
}

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug, PartialEq, Eq)]
pub struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> Default for List<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            head: None,
            tail: ptr::null_mut(),
            count: 0,
        }
    }

    pub fn push_back(&mut self, elem: T) {
        let mut new_tail = Box::new(Node {
            elem,
            next: None,
        });

        let raw_tail: *mut _ = &mut *new_tail;

        if self.tail.is_null() {
            self.head = Some(new_tail);
        } else {
            unsafe {
                (*self.tail).next = Some(new_tail);
            }
        }
        self.tail = raw_tail;
        self.count += 1;
    }

    pub fn push(&mut self, elem: T) {
        let mut new_head = Box::new(Node {
            elem,
            next: self.head.take(),
        });

        let raw_head: *mut _ = &mut *new_head;

        if self.tail.is_null() {
            self.tail = raw_head;
        }
        self.head = Some(new_head);
        self.count += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            if self.head.is_none() {
                self.tail = ptr::null_mut();
            }
            self.count -= 1;
            node.elem
        })
    }

    pub fn pop_back(&mut self) -> Option<T> {
        todo!()
    }

    pub fn peek(&self) -> Option<&T> {
        todo!()
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        todo!()
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.count == 0
    }
}
