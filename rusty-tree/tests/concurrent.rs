use rusty_tree::RustyTree;
use std::thread;
use std::time::Duration;

// TODO: Check if inserting items is thread safe
// - Check if one thread can overwrite the results of another

#[test]
fn insert_from_another_thread() {
    let mut tree: RustyTree<i64, String> = RustyTree::new();

    assert!(tree.insert(0, "One".to_string()).is_ok());
    assert_eq!(tree.get(0), Some("One".to_string()));

    let handle = thread::spawn(move || {
        assert_eq!(tree.get(0), Some("One".to_string()));

        assert!(tree.insert(0, "Two".to_string()).is_ok());
        assert_eq!(tree.get(0), Some("Two".to_string()));
        tree
    });

    let mut tree = handle.join().unwrap();

    assert_eq!(tree.get(0), Some("Two".to_string()));

    assert!(tree.insert(0, "Three".to_string()).is_ok());
    assert_eq!(tree.get(0), Some("Three".to_string()));
}

#[test]
fn insert_multiple_from_another_thread() {
    let mut handles = Vec::new();
    for i in 0..10 {
        let handle = thread::spawn(move || {
            println!("Hello from thread {}", i);
        });
        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap()
    }
}
