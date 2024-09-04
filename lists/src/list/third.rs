// third.rs
// a persistent immutable singly-linked list

use std::{fmt::Debug, sync::Arc};

#[derive(Debug, PartialEq, Eq)]
pub struct List<T> {
    head: Link<T>,
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut head = self.head.take();
        while let Some(node) = head {
            // try_unwrap returns the inner value, if the `Arc` has exactly one strong reference.
            if let Ok(mut node) = Arc::try_unwrap(node) {
                head = node.next.take();
            } else {
                break;
            }
        }
    }
}

type Link<T> = Option<Arc<Node<T>>>;

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
        List { head: None }
    }

    pub fn prepend(&self, elem: T) -> Self {
        List {
            head: Some(Arc::new(Node {
                elem,
                next: self.head.clone(),
            })),
        }
    }

    // takes a list and returns the whole list with the first element removed.
    pub fn tail(&self) -> Self {
        List {
            head: self.head.as_ref().and_then(|node| node.next.clone()),
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

    pub fn get(&self, index: usize) -> Option<&T> {
        let mut i = 0;
        let mut curr = self.head.as_ref();
        while i < index {
            match curr {
                Some(node) => {
                    curr = node.next.as_ref();
                }
                None => {
                    return None;
                }
            }
            i += 1;
        }
        curr.map(|node| &node.elem)
    }

    pub fn len(&self) -> usize {
        let mut len = 0;
        let mut curr = &self.head;
        while let Some(node) = curr {
            len += 1;
            curr = &node.next;
        }
        len
    }

    pub fn iter(&self) -> ListIterator<'_, T> {
        ListIterator {
            next: self.head.as_deref(),
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
