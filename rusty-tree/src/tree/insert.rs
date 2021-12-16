use crate::tree::Node;
use std::{cmp::Ordering, fmt::Debug, marker::Send};

use crate::tree::RustyTree;

impl<K, V> RustyTree<K, V>
where
    K: Send + Ord + Clone + 'static + Debug,
    V: Send + Clone + 'static + Debug,
{
    pub fn insert(&self, key: K, value: V) -> Result<(), String> {
        // // Start from the root node of the tree, and traverse
        // let curr_node: &Node<K, V> = &self.root;
        // while let Node::Entry { entry } = curr_node {
        //     curr_node = match key.cmp(&entry.get_key()?) {
        //         Ordering::Equal => {
        //             entry.set_value(value)?;
        //             return Ok(());
        //         }
        //         Ordering::Less => &*entry.get_left()?,
        //         Ordering::Greater => &*entry.get_right()?,
        //     };
        // }

        // *curr_node = Node::from_key_value(key, value)?;
        // Ok(())

        // If there is a root node, start from it and traverse
        // let mut curr_node: Node<K, V> = root.clone();
        let mut curr_node: Node<K, V> = self.root.clone();
        while let node = curr_node {
            // If the key value is none, the node is empty
            let curr_key = match node.get_key()? {
                None => {
                    node.set_key(key)?;
                    node.set_value(value)?;
                    return Ok(());
                }
                Some(k) => k,
            };

            curr_node = match key.cmp(&curr_key) {
                Ordering::Equal => {
                    node.set_value(value)?;
                    return Ok(());
                }
                Ordering::Less => node
                    .get_left()?
                    .ok_or_else(|| "Failed to get Left")?
                    .clone(),
                Ordering::Greater => node
                    .get_right()?
                    .ok_or_else(|| "Failed to get Right")?
                    .clone(),
            };
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::RustyTree;

    #[test]
    fn insert_single_element() {
        let mut tree: RustyTree<i64, String> = RustyTree::new().unwrap();

        assert!(tree.insert(0, "Test Value".to_string()).is_ok());
        assert_eq!(tree.get(0), Some("Test Value".to_string()));
    }

    #[test]
    fn insert_two_elements() {
        let mut tree: RustyTree<i64, String> = RustyTree::new().unwrap();
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
