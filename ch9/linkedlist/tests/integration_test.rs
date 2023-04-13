#[test]
fn test_linkedlist() {
    use linkedlist::LinkedList;

    let mut linked_list = LinkedList::new();
    linked_list.append("first item");
    linked_list.append("second item");
    linked_list.append("third item");
    linked_list.append("fourth item");
    linked_list.append("fifth and last item");
    linked_list
        .iter()
        .for_each(|data| println!("data={}", data));
    linked_list
        .iter_mut()
        .for_each(|data| println!("data={}", data));

    for data in &linked_list {
        println!("with for loop: data={}", data);
    }
}

#[test]
fn test_linkedlist_iter() {
    use linkedlist::LinkedList;
    let test_data = vec!["first", "second", "third", "fourth", "fifth and last"];
    let mut linked_list = LinkedList::new();
    test_data
        .iter()
        .for_each(|s| linked_list.append(s.to_string()));
    assert_eq!(
        test_data,
        linked_list
            .iter()
            .map(|s| s.as_str())
            .collect::<Vec<&str>>()
    );
}

#[test]
fn test_linkedlist_iter_mut() {
    use linkedlist::LinkedList;
    let test_data = vec!["first", "second", "third", "fourth", "fifth and last"];
    let mut linked_list = LinkedList::new();
    test_data
        .iter()
        .for_each(|s| linked_list.append(s.to_string()));

    assert_eq!(
        test_data,
        linked_list
            .iter_mut()
            .map(|s| s.as_str())
            .collect::<Vec<&str>>()
    );
}

#[test]
fn test_linkedlist_into_iter() {
    use linkedlist::LinkedList;
    let test_data = vec!["first", "second", "third", "fourth", "fifth and last"];
    let mut linked_list = LinkedList::new();
    test_data
        .iter()
        .for_each(|s| linked_list.append(s.to_string()));
    assert_eq!(
        test_data
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>(),
        linked_list.into_iter().collect::<Vec<String>>()
    );
}

#[test]
fn test_linkedlist_cloned() {
    use linkedlist::LinkedList;
    let test_data = vec!["first", "second", "third", "fourth", "fifth and last"];
    let mut linked_list = LinkedList::new();
    test_data
        .iter()
        .for_each(|s| linked_list.append(s.to_string()));

    let cloned_list = linked_list.clone();

    linked_list
        .into_iter()
        .zip(cloned_list.into_iter())
        .for_each(|(left, right)| {
            assert_eq!(left, right);
            assert!(!std::ptr::eq(&left, &right));
        });
}
