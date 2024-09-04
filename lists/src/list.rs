pub mod fifth;
pub mod first;
pub mod fourth;
pub mod second;
pub mod third;

#[cfg(test)]
mod tests {



    use super::*;
    use log::{debug, info};

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test_log::test]
    fn first_test() {
        use first::List;
        init();

        let mut list = List::<i32>::new();
        list.push_back(2);
        list.push(1);
        list.push_back(3);

        info!("=== first_test {:?}", list);

        for (i, node) in list.iter().enumerate() {
            assert_eq!(node.elem, list.get(i).unwrap(), "index: {i}");
        }
        // we can reuse list, since it's not consumed

        for elem in list {
            debug!("elem: {}", elem);
        }
        // we cannot reuse list, since the for loop consumes it
    }

    #[test_log::test]
    fn second_test() {
        use second::List;
        init();

        let mut list = List::<i32>::new();
        list.push_back(2);
        list.push(1);
        list.push_back(3);

        info!("=== second_test {:?}", list);

        assert_eq!(list.peek(), Some(&1));
        assert_eq!(list.len(), 3);
        if let Some(value) = list.peek_mut() {
            *value = -1;
        }
        assert_eq!(list.peek(), Some(&-1));
        assert_eq!(list.len(), 3);

        for (i, node) in list.iter().enumerate() {
            assert_eq!(*node, *list.get(i).unwrap(), "index: {i}");
        }
        // we can reuse list, since it's not consumed

        for node in list.iter_mut() {
            *node = 100;
        }
        // we can reuse list, since it's not consumed, just mutated
        for (i, node) in list.iter().enumerate() {
            assert_eq!(*node, *list.get(i).unwrap(), "index: {i}");
        }

        for elem in list {
            debug!("elem: {}", elem);
        }
        // we cannot reuse list, since the for loop consumes it
    }

    #[test_log::test]
    fn third_test() {
        use third::List;
        init();

        let mut list = List::<i32>::new();
        list = list.prepend(3);
        list = list.prepend(2);
        list = list.prepend(1);

        info!("=== third_test {:?}", list);

        assert_eq!(list.peek(), Some(&1));
        assert_eq!(list.len(), 3);

        for (i, node) in list.iter().enumerate() {
            assert_eq!(*node, *list.get(i).unwrap(), "index: {i}");
        }
        // we can reuse list, since it's not consumed

        let list2 = list.tail();
        assert_eq!(list2.len(), 2);

        // we can reuse list2, since it's not consumed, just mutated
        for (i, node) in list2.iter().enumerate() {
            assert_eq!(*node, *list2.get(i).unwrap());
        }
    }

    #[test_log::test]
    fn fourth_test() {
        use fourth::List;
        init();

        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop_front(), None);

        // Populate list
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        // info!("=== fourth_test {:?}", list);

        // Check normal removal
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push_front(4);
        list.push_front(5);

        // Check normal removal
        assert_eq!(list.pop_front(), Some(5));
        assert_eq!(list.pop_front(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);

        // ---- back -----

        // Check empty list behaves right
        assert_eq!(list.pop_back(), None);

        // Populate list
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        // Check normal removal
        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_back(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push_back(4);
        list.push_back(5);

        // Check normal removal
        assert_eq!(list.pop_back(), Some(5));
        assert_eq!(list.pop_back(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), None);

        // peek
        let mut list = List::new();
        assert!(list.peek_front().is_none());
        assert!(list.peek_back().is_none());
        assert!(list.peek_front_mut().is_none());
        assert!(list.peek_back_mut().is_none());

        list.push_front(1); list.push_front(2); list.push_front(3);

        assert_eq!(&*list.peek_front().unwrap(), &3);
        assert_eq!(&mut *list.peek_front_mut().unwrap(), &mut 3);
        assert_eq!(&*list.peek_back().unwrap(), &1);
        assert_eq!(&mut *list.peek_back_mut().unwrap(), &mut 1);

        // into_iter
        let mut list = List::new();
        list.push_front(1); list.push_front(2); list.push_front(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next_back(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.next(), None);
    }
}
