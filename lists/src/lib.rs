pub mod first;
pub mod second;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn first_test() {
        use first::List;

        let mut list = List::<i32>::new();
        list.push_back(2);
        list.push(1);
        list.push_back(3);
        println!("{:?}", list);

        for (i, node) in list.iter().enumerate() {
            assert_eq!(node.elem, list.get(i).unwrap());
        }
        // we can reuse list, since it's not consumed

        for elem in list {
            println!("elem: {}", elem);
        }
        // we cannot reuse list, since the for loop consumes it
    }

    #[test]
    fn second_test() {
        use second::List;

        let mut list = List::<i32>::new();
        list.push_back(2);
        list.push(1);
        list.push_back(3);
        println!("{:?}", list);

        assert_eq!(list.peek(), Some(&1));
        assert_eq!(list.len(), 3);
        list.peek_mut().map(|value| *value = -1);
        assert_eq!(list.peek(), Some(&-1));
        assert_eq!(list.len(), 3);

        for (i, node) in list.iter().enumerate() {
            assert_eq!(*node, *list.get(i).unwrap());
        }
        // we can reuse list, since it's not consumed

        for node in list.iter_mut() {
            *node = 100;
        }
        // we can reuse list, since it's not consumed, just mutated
        for (i, node) in list.iter().enumerate() {
            assert_eq!(*node, *list.get(i).unwrap());
        }

        for elem in list {
            println!("elem: {}", elem);
        }
        // we cannot reuse list, since the for loop consumes it
    }
}
