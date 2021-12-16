mod get;
mod insert;

use crate::node::Node;
use std::{fmt::Debug, marker::Send};

pub struct RustyTree<K, V> {
    root: Node<K, V>,
}

impl<K, V> RustyTree<K, V>
where
    K: Send + Ord + Clone + 'static + Debug,
    V: Send + Clone + 'static + Debug,
{
    pub fn new() -> Result<Self, String> {
        let root = Node::empty_node()?;
        Ok(Self { root })
    }

    pub fn from_key_value(key: K, value: V) -> Result<Self, String> {
        let root = Node::new(key, value)?;
        Ok(Self { root })
    }
}

impl<K, V> std::fmt::Debug for RustyTree<K, V>
where
    K: Send + Ord + Clone + 'static + Debug,
    V: Send + Clone + 'static + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let key_value = match self.root.get_key_and_value() {
            Ok(Some((key, value))) => format!("Key: {:?}, Value: {:?}", key, value),
            Ok(None) => "Empty Node".to_string(),
            Err(e) => e.to_string(),
        };

        write!(f, "ROOT: {{ {} }}", key_value)
    }
}

#[cfg(test)]
mod tests {
    use super::RustyTree;

    #[test]
    fn create_tree() {
        let _tree: RustyTree<i64, String> = RustyTree::new().unwrap();
    }

    #[test]
    fn create_tree_from_key_value() {
        let tree = RustyTree::from_key_value("Name".to_string(), 100);
        assert!(tree.is_ok());
        assert_eq!(Some(100), tree.unwrap().get("Name".to_string()));
    }
}
