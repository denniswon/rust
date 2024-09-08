// sixth.rs: A Production Unsafe Deque

use std::fmt::Debug;
use std::{marker::PhantomData, ptr};

// https://rust-unofficial.github.io/too-many-lists/sixth-variance.html#variance-and-phantomdata
// type Link<T> = *mut Node<T>;
pub(super) type Link<T> = Option<ptr::NonNull<Node<T>>>;

pub struct LinkedList<T> {
    pub(super) front: Link<T>,
    pub(super) back: Link<T>,
    pub(super) len: usize,

    /// We semantically store values of T by-value.
    /// In this case I don't think we actually need PhantomData,
    /// but any time you do use NonNull (or just raw pointers in general),
    /// you should always add it to be safe and make it clear to the compiler
    /// and others what you think you're doing.
    ///
    /// PhantomData is a way for us to give the compiler an extra "example" field
    /// that conceptually exists in your type but for various reasons
    /// (indirection, type erasure, ...) doesn't.
    _phantom: PhantomData<T>,
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct Node<T> {
    pub(super) front: Link<T>,
    pub(super) back: Link<T>,
    pub(super) elem: T,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            front: None,
            back: None,
            len: 0,
            _phantom: PhantomData,
        }
    }

    pub fn push_back(&mut self, elem: T) {
        let node = unsafe {
            ptr::NonNull::new_unchecked(Box::into_raw(Box::new(Node {
                front: None,
                back: None,
                elem,
            })))
        };

        match self.back {
            Some(old_back) => unsafe {
                (*(old_back.as_ptr())).back = Some(node);
                (*(node.as_ptr())).front = Some(old_back);
            },
            None => {
                // some integrity checks for testing, in case we mess up.
                // https://rust-unofficial.github.io/too-many-lists/sixth-panics.html
                // debug_assert!(self.back.is_none());
                // debug_assert!(self.front.is_none());
                // debug_assert!(self.len == 0);
                self.front = Some(node);
            }
        }

        self.back = Some(node);
        self.len += 1;
    }

    pub fn push_front(&mut self, elem: T) {
        let node = unsafe {
            ptr::NonNull::new_unchecked(Box::into_raw(Box::new(Node {
                front: None,
                back: None,
                elem,
            })))
        };

        match self.front {
            Some(old_front) => unsafe {
                (*(old_front.as_ptr())).front = Some(node);
                (*(node.as_ptr())).back = Some(old_front);
            },
            None => {
                // some integrity checks for testing, in case we mess up.
                // https://rust-unofficial.github.io/too-many-lists/sixth-panics.html
                // debug_assert!(self.back.is_none());
                // debug_assert!(self.front.is_none());
                // debug_assert!(self.len == 0);
                self.back = Some(node);
            }
        }

        self.front = Some(node);
        self.len += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.front.map(|old_front| {
            unsafe {
                self.front = (*(old_front.as_ptr())).back;
                if let Some(new_front) = self.front {
                    (*(old_front.as_ptr())).back = None;
                    (*(new_front.as_ptr())).front = None;
                } else {
                    // debug_assert!(self.len == 1);
                    self.back = None;
                }
            }
            self.len -= 1;
            unsafe { Box::from_raw(old_front.as_ptr()).elem }
        })
    }

    pub fn pop_back(&mut self) -> Option<T> {
        if let Some(mut ptr) = self.front {
            unsafe {
                self.len -= 1;

                while let Some(back) = (*ptr.as_ptr()).back {
                    if (*back.as_ptr()).back.is_none() {
                        break;
                    }
                    ptr = back;
                }
                if let Some(ret_ptr) = (*ptr.as_ptr()).back {
                    (*ptr.as_ptr()).back = None;
                    self.back = Some(ptr);
                    Some(Box::from_raw(ret_ptr.as_ptr()).elem)
                } else {
                    self.front = None;
                    self.back = None;
                    Some(Box::from_raw(ptr.as_ptr()).elem)
                }
            }
        } else {
            None
        }
    }

    pub fn front(&self) -> Option<&T> {
        unsafe { Some(&(*self.front?.as_ptr()).elem) }
    }

    pub fn front_mut(&mut self) -> Option<&mut T> {
        unsafe { Some(&mut (*self.front?.as_ptr()).elem) }
    }

    pub fn back(&self) -> Option<&T> {
        unsafe { Some(&(*self.back?.as_ptr()).elem) }
    }

    pub fn back_mut(&mut self) -> Option<&mut T> {
        unsafe { Some(&mut (*self.back?.as_ptr()).elem) }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn clear(&mut self) {
        while self.pop_front().is_some() {}
    }
}
