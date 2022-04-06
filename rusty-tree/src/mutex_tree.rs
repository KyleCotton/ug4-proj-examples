use std::{
    cmp::Ordering,
    fmt::Debug,
    sync::{Arc, Mutex},
};

#[derive(Clone)]
enum Node<K, V>
where
    K: Send + Sync + 'static + Clone + Debug + Ord,
    V: Send + Sync + 'static + Clone + Debug,
{
    Empty,
    NonEmpty {
        key: K,
        value: V,
        left: Box<Node<K, V>>,
        right: Box<Node<K, V>>,
    },
}

impl<K, V> Node<K, V>
where
    K: Send + Sync + 'static + Clone + Debug + Ord,
    V: Send + Sync + 'static + Clone + Debug,
{
    pub fn insert(&mut self, new_key: K, new_value: V) -> Option<V> {
        std::thread::sleep(std::time::Duration::from_nanos(1));
        match self {
            &mut Node::NonEmpty {
                ref key,
                ref mut value,
                ref mut left,
                ref mut right,
            } => match new_key.cmp(key) {
                Ordering::Less => left.insert(new_key, new_value),
                Ordering::Greater => right.insert(new_key, new_value),
                Ordering::Equal => {
                    *value = new_value.clone();
                    Some(new_value)
                }
            },
            &mut Node::Empty => {
                *self = Node::NonEmpty {
                    key: new_key,
                    value: new_value.clone(),
                    left: Box::new(Node::Empty),
                    right: Box::new(Node::Empty),
                };
                Some(new_value)
            }
        }
    }

    pub fn get(&self, search_key: K) -> Option<V> {
        std::thread::sleep(std::time::Duration::from_nanos(1));
        match self {
            &Node::NonEmpty {
                ref key,
                ref value,
                ref left,
                ref right,
            } => match search_key.cmp(key) {
                Ordering::Less => left.get(search_key),
                Ordering::Greater => right.get(search_key),
                Ordering::Equal => Some(value.clone()),
            },
            &Node::Empty => None,
        }
    }
}

#[derive(Clone)]
pub struct RustyTree<K, V>
where
    K: 'static + Send + Sync + Clone + Debug + Ord,
    V: 'static + Send + Sync + Clone + Debug,
{
    root: Arc<Mutex<Node<K, V>>>,
}

impl<K, V> RustyTree<K, V>
where
    K: 'static + Send + Sync + Clone + Debug + Ord,
    V: 'static + Send + Sync + Clone + Debug,
{
    pub fn new() -> Self {
        let root = Arc::new(Mutex::new(Node::Empty));
        Self { root }
    }

    pub fn insert(&self, key: K, value: V) -> Option<V> {
        let mut root = self.root.lock().ok()?;
        root.insert(key, value)
    }

    pub fn get(&self, key: K) -> Option<V> {
        let root = self.root.lock().ok()?;
        root.get(key)
    }
}
