pub mod first;

#[cfg(test)]
mod tests {
    use first::List;

    use super::*;

    #[test]
    fn first_test() {
        let mut list = List::<i32>::new();
        list.push_back(2);
        list.push(1);
        list.push_back(3);
        println!("{:?}", list);

        for node in list.iter() {
            println!("node: {}", node.elem);
        }
        // we can reuse list, since it's not consumed

        for elem in list {
            println!("elem: {}", elem);
        }
        // we cannot reuse list, since the for loop consumes it
    }
}
