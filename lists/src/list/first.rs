// first.rs

use std::{fmt::Debug, mem::replace};

#[derive(Debug, PartialEq, Eq)]
pub struct List<T: Copy> {
    head: Link<T>,
    count: usize,
}

impl<T: Copy> Drop for List<T> {
    fn drop(&mut self) {
        let mut curr = replace(&mut self.head, Link::Empty);
        while let Link::More(mut node) = curr {
            curr = replace(&mut node.next, Link::Empty);
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Link<T: Copy> {
    Empty,
    More(Box<Node<T>>),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Node<T: Copy> {
    pub elem: T,
    next: Link<T>,
}

impl<T: Copy> Default for List<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Copy> List<T> {
    pub fn new() -> Self {
        List {
            head: Link::Empty,
            count: 0,
        }
    }

    pub fn push_back(&mut self, elem: T) {
        let item = Link::More(Box::new(Node {
            elem,
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

    pub fn push(&mut self, elem: T) {
        let item = Box::new(Node {
            elem,
            // temporarily replace head with Link::Empty, moving the old head into item next
            next: replace(&mut self.head, Link::Empty),
        });
        self.head = Link::More(item);
        self.count += 1;
    }

    pub fn get(&self, index: usize) -> Option<T> {
        self._get(index).map(|node| node.elem)
    }

    fn _get(&self, index: usize) -> Option<&Box<Node<T>>> {
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

    pub fn pop(&mut self) -> Option<T> {
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

    pub fn iter(&self) -> ListIterator<T> {
        ListIterator {
            inner: self,
            index: 0,
        }
    }
}

pub struct ListIterator<'a, T: Copy> {
    inner: &'a List<T>,
    index: usize,
}

impl<'a, T> Iterator for ListIterator<'a, T>
where
    T: Copy,
{
    type Item = &'a Box<Node<T>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.inner.len() == 0 {
            return None;
        }
        let ret = self.inner._get(self.index);
        self.index += 1;
        ret
    }
}

pub struct ListIntoIterator<T: Copy> {
    inner: List<T>,
    index: usize,
}

impl<T> Iterator for ListIntoIterator<T>
where
    T: Copy,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.inner.len() == 0 {
            return None;
        }
        let ret = self.inner.get(self.index);
        self.index += 1;
        ret
    }
}

impl<T> IntoIterator for List<T>
where
    T: Copy,
{
    type Item = T;

    type IntoIter = ListIntoIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        ListIntoIterator {
            inner: self,
            index: 0,
        }
    }
}
