use rusty_tree::RustyTree;
use std::thread;

#[test]
fn insert_from_another_thread() {
    let tree: RustyTree<i64, String> = RustyTree::new();

    assert!(tree.insert(0, "One".to_string()).is_some());
    assert_eq!(tree.get(0), Some("One".to_string()));

    let handle = thread::spawn(move || {
        assert_eq!(tree.get(0), Some("One".to_string()));

        assert!(tree.insert(0, "Two".to_string()).is_some());
        assert_eq!(tree.get(0), Some("Two".to_string()));
        tree
    });

    let tree = handle.join().unwrap();

    assert_eq!(tree.get(0), Some("Two".to_string()));

    assert!(tree.insert(0, "Three".to_string()).is_some());
    assert_eq!(tree.get(0), Some("Three".to_string()));
}

#[test]
fn parallel_insertion() {
    let tree: RustyTree<i32, String> = RustyTree::new();

    let handles = (0..10).into_iter().map(|i| {
        let tree_clone = tree.clone();
        std::thread::spawn(move || {
            let insert = tree_clone.insert(i, format!("Item: {i}"));
            assert!(insert.is_some());
        })
    });

    handles.into_iter().for_each(|h| {
        h.join().unwrap();
    });
}
