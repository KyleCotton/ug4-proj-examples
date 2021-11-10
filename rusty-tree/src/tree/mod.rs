use crate::entry::Entry;
use std::{fmt::Debug, marker::Send};

mod get;
mod insert;

// type ChildEntry<K, V> = Option<Box<RustyTree<K, V>>>;
pub enum Node<K, V> {
    Empty,
    Entry {
        entry: Entry<K, V>,
        left: Box<Node<K, V>>,
        right: Box<Node<K, V>>,
    },
}

impl<K, V> Node<K, V>
where
    K: Send + Ord + Clone + 'static + Debug,
    V: Send + Clone + 'static + Debug,
{
    pub fn from_key_value(key: K, value: V) -> Result<Self, String> {
        let entry = Entry::new(key, value)?;
        let left = Box::new(Node::Empty);
        let right = Box::new(Node::Empty);

        Ok(Node::Entry { entry, left, right })
    }
}

pub struct RustyTree<K, V> {
    root: Node<K, V>,
}

impl<K, V> RustyTree<K, V>
where
    K: Send + Ord + Clone + 'static + Debug,
    V: Send + Clone + 'static + Debug,
{
    pub fn new() -> Self {
        Self { root: Node::Empty }
    }

    pub fn from_key_value(key: K, value: V) -> Result<Self, String> {
        let root = Node::from_key_value(key, value)?;
        Ok(Self { root })
    }
}

impl<K, V> std::fmt::Debug for RustyTree<K, V>
where
    K: Send + Ord + Clone + 'static + Debug,
    V: Send + Clone + 'static + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let value = match &self.root {
            Node::Empty => "Tree {{ EMPTY }}".to_string(),
            Node::Entry { entry, .. } => {
                format!("Tree {{ {:?} }}", entry)
            }
        };

        write!(f, "{}", value)
    }
}

#[cfg(test)]
mod tests {
    use super::RustyTree;

    #[test]
    fn create_tree() {
        let _tree: RustyTree<i64, String> = RustyTree::new();
    }

    #[test]
    fn create_tree_from_key_value() {
        let tree = RustyTree::from_key_value("Name".to_string(), 100);
        assert!(tree.is_ok());
        assert_eq!(Some(100), tree.unwrap().get("Name".to_string()));
    }
}
