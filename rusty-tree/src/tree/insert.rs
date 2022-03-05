use crate::tree::Node;
use std::{cmp::Ordering, fmt::Debug, marker::Send};

use crate::tree::RustyTree;

impl<K, V> RustyTree<K, V>
where
    K: Send + Ord + Clone + 'static + Debug,
    V: Send + Clone + 'static + Debug,
{
    pub fn insert(&self, key: K, value: V) -> Result<(), String> {
        // If there is a root node, start from it and traverse
        let mut curr_node: Node<K, V> = self.root.clone();
        loop {
            // If the key value is none, the node is empty
            let curr_key = match curr_node.get_key()? {
                None => {
                    log::debug!("Node is None");
                    curr_node.set_key(key)?;
                    curr_node.set_value(value)?;
                    return Ok(());
                }
                Some(k) => {
                    log::debug!("Node is {k:?}");
                    k
                },
            };

            curr_node = match key.cmp(&curr_key) {
                Ordering::Equal => {
                    curr_node.set_value(value)?;
                    return Ok(());
                }
                Ordering::Less => curr_node
                    .get_left()?
                    .ok_or_else(|| "Failed to get Left")?
                    .clone(),
                Ordering::Greater => curr_node
                    .get_right()?
                    .ok_or_else(|| "Failed to get Right")?
                    .clone(),
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::RustyTree;

    #[test]
    fn insert_single_element() {
        let tree: RustyTree<i64, String> = RustyTree::new().unwrap();

        assert!(tree.insert(0, "Test Value".to_string()).is_ok());
        assert_eq!(tree.get(0), Some("Test Value".to_string()));
    }

    #[test]
    fn insert_two_elements() {
        let tree: RustyTree<i64, String> = RustyTree::new().unwrap();
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

        let tree = tree.unwrap();
        assert_eq!(tree.get(0), Some(100));

        assert!(tree.insert(0, 200).is_ok());
        assert_eq!(tree.get(0), Some(200));

        assert!(tree.insert(0, 300).is_ok());
        assert_eq!(tree.get(0), Some(300));
    }
}
