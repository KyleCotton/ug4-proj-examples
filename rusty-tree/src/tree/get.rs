use crate::{
    entry::Entry,
    tree::{Node, RustyTree},
};
use std::{cmp::Ordering, fmt::Debug, marker::Send};

impl<K, V> RustyTree<K, V>
where
    K: Send + Ord + Clone + 'static + Debug,
    V: Send + Clone + 'static + Debug,
{
    pub fn get(&self, key: K) -> Option<V> {
        // Start from the root node of the tree, see if there is one.
        if let Node::Empty = self.root {
            return None;
        }

        // There is a root node, start from it and traverse
        let mut curr_node: Node<K, V> = self.root.clone();
        while let Node::Entry { entry } = curr_node {
            curr_node = match key.cmp(&key) {
                Ordering::Equal => return entry.get_value().ok(),
                Ordering::Less => *entry.get_left().ok()?.clone(),
                Ordering::Greater => *entry.get_right().ok()?.clone(),
            };
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::RustyTree;

    #[test]
    fn get_single_element() {
        let tree: RustyTree<i64, String> =
            RustyTree::from_key_value(0, "Test Value 0".to_string()).unwrap();
        assert!(tree.get(0).is_some());
        assert_eq!(Some("Test Value 0".to_string()), tree.get(0));
    }

    #[test]
    fn get_single_element_repeated() {
        let tree: RustyTree<i64, String> =
            RustyTree::from_key_value(0, "Test Value 0".to_string()).unwrap();
        assert!(tree.get(0).is_some());
        assert_eq!(Some("Test Value 0".to_string()), tree.get(0));
        assert_eq!(Some("Test Value 0".to_string()), tree.get(0));
        assert_eq!(Some("Test Value 0".to_string()), tree.get(0));
    }

    #[test]
    fn get_two_elements() {
        let mut tree: RustyTree<i64, String> = RustyTree::new();
        tree.insert(0, "Test Value 0".to_string()).ok();
        assert!(tree.get(0).is_some());

        tree.insert(1, "Test Value 1".to_string()).ok();
        assert!(tree.get(1).is_some());

        assert_eq!(Some("Test Value 0".to_string()), tree.get(0));
        assert_eq!(Some("Test Value 1".to_string()), tree.get(1));
    }

    #[test]
    fn get_multiple_elements() {
        let mut tree: RustyTree<i64, String> = RustyTree::new();
        tree.insert(0, "Test Value 0".to_string()).ok();
        tree.insert(1, "Test Value 1".to_string()).ok();
        tree.insert(2, "Test Value 2".to_string()).ok();
        tree.insert(3, "Test Value 3".to_string()).ok();
        tree.insert(4, "Test Value 4".to_string()).ok();

        assert_eq!(Some("Test Value 0".to_string()), tree.get(0));
        assert_eq!(Some("Test Value 1".to_string()), tree.get(1));
        assert_eq!(Some("Test Value 2".to_string()), tree.get(2));
        assert_eq!(Some("Test Value 3".to_string()), tree.get(3));
        assert_eq!(Some("Test Value 4".to_string()), tree.get(4));
    }

    #[test]
    fn get_non_existing_elements() {
        let mut tree: RustyTree<i64, String> = RustyTree::new();
        tree.insert(0, "Test Value 0".to_string()).ok();
        tree.insert(1, "Test Value 1".to_string()).ok();
        tree.insert(2, "Test Value 2".to_string()).ok();
        tree.insert(3, "Test Value 3".to_string()).ok();
        tree.insert(4, "Test Value 4".to_string()).ok();

        assert_eq!(None, tree.get(100));
        assert_eq!(None, tree.get(200));
        assert_eq!(Some("Test Value 2".to_string()), tree.get(2));
        assert_eq!(Some("Test Value 3".to_string()), tree.get(3));
        assert_eq!(Some("Test Value 4".to_string()), tree.get(4));
    }
}
