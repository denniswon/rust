pub mod first;
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

        info!("=== first_test ===");
        info!("{:?}", list);

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

        info!("=== second_test ===");
        info!("{:?}", list);

        assert_eq!(list.peek(), Some(&1));
        assert_eq!(list.len(), 3);
        list.peek_mut().map(|value| *value = -1);
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

        info!("=== third_test ===");
        info!("{:?}", list);

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
}