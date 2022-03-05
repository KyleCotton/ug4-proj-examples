use rusty_tree::RustyTree;

#[test]
fn create_tree() {
    let _tree: RustyTree<i32, i32> = RustyTree::new();
}

#[test]
fn get_on_empty_tree() {
    let tree: RustyTree<i32, i32> = RustyTree::new();
    assert!(tree.get(100).is_none());
}

#[test]
fn insert_one_item() {
    let tree: RustyTree<i32, String> = RustyTree::new();
    assert_eq!(
        tree.insert(0, "Hello".to_string()),
        Some("Hello".to_string())
    );
}

#[test]
fn get_one_item() {
    let tree: RustyTree<i32, String> = RustyTree::new();
    assert_eq!(
        tree.insert(0, "Hello".to_string()),
        Some("Hello".to_string())
    );

    assert_eq!(tree.get(0), Some("Hello".to_string()));
}

#[test]
fn insert_same_keyed_item() {
    let tree: RustyTree<i32, String> = RustyTree::new();

    assert_eq!(
        tree.insert(0, "Hello".to_string()),
        Some("Hello".to_string())
    );

    assert_eq!(tree.get(0), Some("Hello".to_string()));

    assert_eq!(
        tree.insert(0, "There".to_string()),
        Some("There".to_string())
    );

    assert_eq!(tree.get(0), Some("There".to_string()));
}

#[test]
fn get_nonexistant_item() {
    let tree: RustyTree<i32, String> = RustyTree::new();
    assert_eq!(
        tree.insert(0, "Hello".to_string()),
        Some("Hello".to_string())
    );

    assert_eq!(tree.get(100), None);
}

#[test]
fn insert_get_multiple() {
    let tree: RustyTree<i32, String> = RustyTree::new();
    assert_eq!(
        tree.insert(0, "Hello".to_string()),
        Some("Hello".to_string())
    );
    assert_eq!(tree.get(0), Some("Hello".to_string()));

    assert_eq!(
        tree.insert(1, "There".to_string()),
        Some("There".to_string())
    );
    assert_eq!(tree.get(1), Some("There".to_string()));

    assert_eq!(tree.insert(2, "You".to_string()), Some("You".to_string()));
    assert_eq!(tree.get(2), Some("You".to_string()));
}
