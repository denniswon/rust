// fifth.rs: Unsafe Queue

use std::ptr;

type Link<T> = *mut Node<T>;

pub struct List<T> {
    head: Link<T>,
    tail: *mut Node<T>,
    count: usize,
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while self.pop().is_some() {}
    }
}


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
            head: ptr::null_mut(),
            tail: ptr::null_mut(),
            count: 0,
        }
    }

    pub fn push_back(&mut self, elem: T) {
        let raw_tail = Box::into_raw(Box::new(Node {
            elem,
            next: ptr::null_mut(),
        }));

        if self.tail.is_null() {
            self.head = raw_tail;
        } else {
            unsafe {
                (*self.tail).next = raw_tail;
            }
        }

        self.tail = raw_tail;
        self.count += 1;
    }

    pub fn push(&mut self, elem: T) {
        self.head = if self.head.is_null() {
            Box::into_raw(Box::new(Node {
                elem,
                next: ptr::null_mut(),
            }))
        } else {
            Box::into_raw(Box::new(Node {
                elem,
                next: self.head,
            }))
        };

        self.count += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.head.is_null() {
            return None;
        }

        unsafe {
            if (*self.head).next.is_null() {
                self.tail = ptr::null_mut();
            }

            let old_head = self.head;

            self.head = (*self.head).next;
            self.count -= 1;

            Some(Box::from_raw(old_head).elem)
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        if self.head.is_null() {
            return None;
        }

        self.count -= 1;

        let mut ptr = self.head;
        unsafe {
            while !(*ptr).next.is_null() {
                ptr = (*ptr).next;
            }

            (*ptr).next = ptr::null_mut();
            self.tail = ptr;
            Some(Box::from_raw(ptr).elem)
        }
    }

    pub fn peek(&self) -> Option<&T> {
        if self.head.is_null() {
            None
        } else {
            Some(unsafe { &(*self.head).elem })
        }
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        if self.head.is_null() {
            None
        } else {
            Some(unsafe { &mut (*self.head).elem })
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.count == 0
    }
}

pub struct ListIntoIter<T>(List<T>);

pub struct ListIterator<'a, T> {
    next: Option<&'a Node<T>>,
}

pub struct ListIterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

// Iterator
impl<T> Iterator for ListIntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

// Iterator
impl<'a, T> Iterator for ListIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            self.next.map(|node| {
                self.next = node.next.as_ref();
                &node.elem
            })
        }
    }
}

// IterMut
impl<'a, T> Iterator for ListIterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            self.next.take().map(|node| {
                self.next = node.next.as_mut();
                &mut node.elem
            })
        }
    }
}

// IntoIterator
impl<T> IntoIterator for List<T> {
    type Item = T;

    type IntoIter = ListIntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        ListIntoIter(self)
    }
}

impl<T> List<T> {
    pub fn into_iter(self) -> ListIntoIter<T> {
        ListIntoIter(self)
    }

    pub fn iter(&self) -> ListIterator<'_, T> {
        ListIterator { next: unsafe { self.head.as_ref() } }
    }

    pub fn iter_mut(&mut self) -> ListIterMut<'_, T> {
        ListIterMut { next: unsafe { self.head.as_mut() } }
    }
}