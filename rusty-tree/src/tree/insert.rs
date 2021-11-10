use crate::tree::Node;
use std::{cmp::Ordering, fmt::Debug, marker::Send};

use crate::tree::RustyTree;


impl<K, V> RustyTree<K, V>
where
    K: Send + Ord + Clone + 'static + Debug,
    V: Send + Clone + 'static + Debug,
{
    pub fn insert(&mut self, key: K, value: V) -> Result<(), String> {
        // Start from the root node of the tree, see if there is one.
        if let Node::Empty = self.root {
            let new_tree = RustyTree::from_key_value(key, value)?;
            *self = new_tree;
            return Ok(());
        }

        // There is a root node, start from it and traverse
        let mut current: &mut Node<K, V> = &mut self.root;
        while let Node::Entry {
            entry,
            ref mut left,
            ref mut right,
        } = current
        {
            current = match key.cmp(&entry.get_key()?) {
                Ordering::Equal => {
                    entry.set_value(value)?;
                    return Ok(());
                }
                Ordering::Less => left.as_mut(),
                Ordering::Greater => right.as_mut(),
            };
        }

        *current = Node::from_key_value(key, value)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::RustyTree;

    #[test]
    fn insert_single_element() {
        let mut tree: RustyTree<i64, String> = RustyTree::new();

        assert!(tree.insert(0, "Test Value".to_string()).is_ok());
        assert_eq!(tree.get(0), Some("Test Value".to_string()));
    }

    #[test]
    fn insert_two_elements() {
        let mut tree: RustyTree<i64, String> = RustyTree::new();
        assert!(tree.insert(0, "Test Value 0".to_string()).is_ok());
        assert_eq!(tree.get(0), Some("Test Value 0".to_string()));

        assert!(tree.insert(1, "Test Value 1".to_string()).is_ok());
        assert_eq!(tree.get(1), Some("Test Value 1".to_string()));


        assert_eq!(tree.get(0), Some("Test Value 0".to_string()));
        assert_eq!(tree.get(1), Some("Test Value 1".to_string()));
    }

    #[test]
    fn insert_from_key_value() {
        let tree = RustyTree::from_key_value(1000000, -2);
        assert!(tree.is_ok());

        let tree = tree.unwrap();
        assert_eq!(tree.get(1000000), Some(-2));
    }

    #[test]
    fn overwrite_existing_values() {
        let tree = RustyTree::from_key_value(0, 100);
        assert!(tree.is_ok());

        let mut tree = tree.unwrap();
        assert_eq!(tree.get(0), Some(100));

        assert!(tree.insert(0, 200).is_ok());
        assert_eq!(tree.get(0), Some(200));

        assert!(tree.insert(0, 300).is_ok());
        assert_eq!(tree.get(0), Some(300));
    }
}
