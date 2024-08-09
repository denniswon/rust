pub mod first;

#[cfg(test)]
mod tests {
    use first::List;

    use super::*;

    #[test]
    fn first_test() {
        let mut list: List = List::new();
        list.push_back(2);
        list.push(1);
        list.push_back(3);
        println!("{:?}", list);

        for node in list.iter() {
            println!("node: {}", node.elem);
        }

        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(3));
        println!("{:?}", list);
    }
}
