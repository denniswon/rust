use super::core::{Link, LinkedList};

/// A Cursor is like an iterator, except that it can freely seek back-and-forth,
/// and can safely mutate the list during iteration. This is because the lifetime
/// of its yielded references are tied to its own lifetime, instead of just the underlying list.
/// This means cursors cannot yield multiple elements at once.
///
/// Cursors always rest between two elements in the list, and index in a logically circular way.
/// To accomadate this, there is a "ghost" non-element that yields None
/// between the head and tail of the List.
///
/// When created, cursors start between the ghost and the front of the list.
/// That is, next will yield the front of the list, and prev will yield None.
/// Calling prev again will yield the tail.

/// Cursor can:
/// - point "between" two elements (a pointer to the current next node)
/// - as a nice little feature, keep track of what "index" is next (the current index)
/// - update the list itself to modify front/back/len. (a pointer to the list)

pub struct CursorMut<'a, T> {
    cur: Link<T>,
    list: &'a mut LinkedList<T>,
    index: Option<usize>,
}

impl<T> LinkedList<T> {
    /// O(1) list splitting, O(1) list splicing
    fn splice(&mut self, other: &mut LinkedList<T>) {}
}
