// sixth.rs: A Production Unsafe Deque

use std::marker::PhantomData;

use super::core::{Link, LinkedList};

/// Iterator
pub struct Iter<'a, T> {
    pub(super) front: Link<T>,
    pub(super) back: Link<T>,
    pub(super) len: usize,
    _phantom: PhantomData<&'a T>,
}

pub struct IterMut<'a, T> {
    pub(super) front: Link<T>,
    pub(super) back: Link<T>,
    pub(super) len: usize,
    _phantom: PhantomData<&'a mut T>,
}

impl<T> LinkedList<T> {
    pub fn iter(&self) -> Iter<T> {
        Iter {
            front: self.front,
            back: self.back,
            len: self.len,
            _phantom: PhantomData,
        }
    }

    pub fn iter_mut(&self) -> IterMut<T> {
        IterMut {
            front: self.front,
            back: self.back,
            len: self.len,
            _phantom: PhantomData,
        }
    }
}

impl<'a, T> IntoIterator for &'a LinkedList<T> {
    type IntoIter = Iter<'a, T>;
    type Item = &'a T;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut LinkedList<T> {
    type IntoIter = IterMut<'a, T>;
    type Item = &'a mut T;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len > 0 {
            let front = self.front.unwrap();
            unsafe {
                self.front = (*front.as_ptr()).back;
                if self.front.is_none() {
                    self.len = 0;
                } else {
                    self.len -= 1;
                }
                Some(&(*(front.as_ptr())).elem)
            }
        } else {
            None
        }
    }

    /// Returns the bounds on the remaining length of the iterator.
    ///
    /// Specifically, `size_hint()` returns a tuple where the first element
    /// is the lower bound, and the second element is the upper bound.
    ///
    /// The second half of the tuple that is returned is an <code>[Option]<[usize]></code>.
    /// A [`None`] here means that either there is no known upper bound, or the
    /// upper bound is larger than [`usize`].
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len > 0 {
            let front = self.front.unwrap();
            unsafe {
                self.front = (*front.as_ptr()).back;
                if self.front.is_none() {
                    self.len = 0;
                } else {
                    self.len -= 1;
                }
                Some(&mut (*(front.as_ptr())).elem)
            }
        } else {
            None
        }
    }

    /// Returns the bounds on the remaining length of the iterator.
    ///
    /// Specifically, `size_hint()` returns a tuple where the first element
    /// is the lower bound, and the second element is the upper bound.
    ///
    /// The second half of the tuple that is returned is an <code>[Option]<[usize]></code>.
    /// A [`None`] here means that either there is no known upper bound, or the
    /// upper bound is larger than [`usize`].
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.len > 0 {
            let back = self.back.unwrap();
            unsafe {
                self.back = (*back.as_ptr()).front;
                if self.back.is_none() {
                    self.len = 0;
                } else {
                    self.len -= 1;
                }
                Some(&(*(back.as_ptr())).elem)
            }
        } else {
            None
        }
    }
}

impl<'a, T> DoubleEndedIterator for IterMut<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.len > 0 {
            let back = self.back.unwrap();
            unsafe {
                self.back = (*back.as_ptr()).front;
                if self.back.is_none() {
                    self.len = 0;
                } else {
                    self.len -= 1;
                }
                Some(&mut (*(back.as_ptr())).elem)
            }
        } else {
            None
        }
    }
}

// provide a more efficient implementation of returning length of the list
impl<'a, T> ExactSizeIterator for Iter<'a, T> {
    fn len(&self) -> usize {
        self.len
    }
}

impl<'a, T> ExactSizeIterator for IterMut<'a, T> {
    fn len(&self) -> usize {
        self.len
    }
}

/// IntoIter
pub struct IntoIter<T> {
    list: LinkedList<T>,
}

impl<T> LinkedList<T> {
    pub fn _into_iter(self) -> IntoIter<T> {
        IntoIter { list: self }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.list.pop_front()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.list.len, Some(self.list.len))
    }
}

impl<T> IntoIterator for LinkedList<T> {
    type IntoIter = IntoIter<T>;
    type Item = T;

    fn into_iter(self) -> Self::IntoIter {
        self._into_iter()
    }
}

/// DoubleEndedIterator
impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.list.pop_back()
    }
}

/// ExactSizeIterator
impl<T> ExactSizeIterator for IntoIter<T> {
    fn len(&self) -> usize {
        self.list.len
    }
}
