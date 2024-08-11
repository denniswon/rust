// second.rs

use std::{fmt::Debug, mem::replace, ops::Deref};

#[derive(Debug, PartialEq, Eq)]
pub struct List<T> {
    head: Link<T>,
    count: usize,
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut curr = self.head.take();
        while let Some(mut node) = curr {
            curr = node.next.take();
        }
    }
}

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug, PartialEq, Eq)]
pub struct Node<T> {
    pub elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            head: None,
            count: 0,
        }
    }

    pub fn push_back(&mut self, elem: T) {
        let item = Some(Box::new(Node {
            elem: elem,
            next: None,
        }));

        let mut curr = &mut self.head;
        loop {
            match curr {
                Some(node) => {
                    curr = &mut node.next;
                }
                None => {
                    let _ = replace(curr, item);
                    break;
                }
            }
        }
        self.count += 1;
    }

    pub fn push(&mut self, elem: T) {
        let item = Box::new(Node {
            elem: elem,
            // temporarily replace head with None, moving the old head into item next
            // take() is an idiom for mem::replace(&mut option, None)
            // take() takes the value of the option and replaces it with None
            next: self.head.take(),
        });
        self.head = Some(item);
        self.count += 1;
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.count {
            None
        } else {
            let mut i = 0;
            let mut curr = &self.head;
            while i < index {
                match curr {
                    Some(node) => curr = &node.next,
                    None => break,
                }
                i += 1;
            }
            match curr {
                Some(node) => Some(&node.elem),
                None => None,
            }
        }
    }

    fn _get(&self, index: usize) -> Option<&Box<Node<T>>> {
        if index >= self.count {
            None
        } else {
            let mut i = 0;
            let mut curr = &self.head;
            while i < index {
                match curr {
                    Some(node) => curr = &node.next,
                    None => break,
                }
                i += 1;
            }
            match curr {
                Some(node) => Some(node),
                None => None,
            }
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.head.take() {
            Some(node) => {
                self.head = node.next;
                self.count -= 1;
                Some(node.elem)
            }
            None => None,
        }
    }

    pub fn peek(&self) -> Option<&T> {
        /*
         * map() maps an `Option<T>` to `Option<U>` by applying a function
         * to a contained value (if `Some`) or returns `None` (if `None`).
         *
         * as_ref() converts from `&Option<Box<Node<T>>>` to `Option<&Box<Node<T>>`.
         */
        self.head.as_ref().map(|node| &node.elem)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem)
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn iter<'a>(&'a self) -> ListIterator<'a, T> {
        ListIterator {
            next: self.head.as_ref().map(|node| &**node),
        }
    }

    pub fn iter_mut<'a>(&'a mut self) -> ListIterMut<'a, T> {
        ListIterMut {
            next: self.head.as_mut().map(|node| &mut **node),
        }
    }
}

// Iterator
pub struct ListIterator<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for ListIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            // same as node.next.as_ref().map(|n| n.deref());
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

// IntoIterator
pub struct ListIntoIterator<T> {
    inner: List<T>,
}

impl<T> Iterator for ListIntoIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.pop()
    }
}

impl<T> IntoIterator for List<T> {
    type Item = T;

    type IntoIter = ListIntoIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        ListIntoIterator { inner: self }
    }
}

// IterMut
pub struct ListIterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<'a, T> Iterator for ListIterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            // same as node.next.as_mut().map(|n| &mut **n);
            self.next = node.next.as_deref_mut();
            &mut node.elem
        })
    }
}
