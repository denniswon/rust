// first.rs

use std::{fmt::Debug, mem::replace};

#[derive(Debug, PartialEq, Eq)]
pub struct List {
    head: Link,
    count: usize,
}

impl Drop for List {
    fn drop(&mut self) {
        while let Link::More(node) = replace(&mut self.head, Link::Empty) {
            let node = replace(&mut self.head, node.next);
            drop(node);
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Link {
    Empty,
    More(Box<Node>),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Node {
    pub elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List {
            head: Link::Empty,
            count: 0,
        }
    }

    pub fn push_back(&mut self, elem: i32) {
        let item = Link::More(Box::new(Node {
            elem: elem,
            next: Link::Empty,
        }));

        let mut curr = &mut self.head;
        loop {
            match curr {
                Link::More(node) => {
                    curr = &mut node.next;
                }
                Link::Empty => {
                    let _ = replace(curr, item);
                    break;
                }
            }
        }
        self.count += 1;
    }

    pub fn push(&mut self, elem: i32) {
        let item = Box::new(Node {
            elem: elem,
            // temporarily replace head with Link::Empty, moving the old head into item next
            next: replace(&mut self.head, Link::Empty),
        });
        self.head = Link::More(item);
        self.count += 1;
    }

    pub fn get(&self, index: usize) -> Option<i32> {
        if index >= self.count {
            None
        } else {
            let mut i = 0;
            let mut curr = &self.head;
            while i < index {
                match curr {
                    Link::More(node) => curr = &node.next,
                    Link::Empty => break,
                }
                i += 1;
            }
            match curr {
                Link::More(node) => Some(node.elem),
                Link::Empty => None,
            }
        }
    }

    fn _get(&self, index: usize) -> Option<&Box<Node>> {
        if index >= self.count {
            None
        } else {
            let mut i = 0;
            let mut curr = &self.head;
            while i < index {
                match curr {
                    Link::More(node) => curr = &node.next,
                    Link::Empty => break,
                }
                i += 1;
            }
            match curr {
                Link::More(node) => Some(node),
                Link::Empty => None,
            }
        }
    }

    pub fn pop(&mut self) -> Option<i32> {
        match replace(&mut self.head, Link::Empty) {
            Link::More(node) => {
                self.head = node.next;
                self.count -= 1;
                Some(node.elem)
            }
            Link::Empty => None,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn iter(&self) -> Iter {
        Iter {
            inner: self,
            index: 0,
        }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a Box<Node>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.inner.len() == 0 {
            return None;
        }
        let ret = self.inner._get(self.index);
        self.index += 1;
        ret
    }
}

pub struct Iter<'a> {
    inner: &'a List,
    index: usize,
}
