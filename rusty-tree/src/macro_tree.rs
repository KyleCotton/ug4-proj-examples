use rusty_junctions::{
    channels::{BidirChannel, SendChannel},
    junction,
};
use std::{cmp::Ordering, fmt::Debug};

#[derive(Clone)]
struct Node<K, V>
where
    K: Send + Sync + 'static + Clone + Debug + Ord,
    V: Send + Sync + 'static + Clone + Debug,
{
    pub get: BidirChannel<K, Option<V>>,
    pub set: SendChannel<(K, V)>,
}

impl<K, V> Node<K, V>
where
    K: Send + Sync + 'static + Clone + Debug + Ord,
    V: Send + Sync + 'static + Clone + Debug,
{
    pub fn new() -> Self {
        junction! {
            junction as Junction,
            key as Send::K,
            value as Send::V,
            left as Send::Node<K, V>,
            right as Send::Node<K, V>,
            empty as Send::(),

            get as Bidir::(K, Option<V>),
            set as Send::(K, V),

            |empty, set| {
                let (new_key, new_value) = set;
                println!("Macro Insert: Node Empty - Initialising");
                key_super.send(new_key).unwrap();
                value_super.send(new_value).unwrap();
                left_super.send(Self::new()).unwrap();
                right_super.send(Self::new()).unwrap();
            },

            |key, value, left, right, set| {
                let (search_key, new_value) = set;
                println!("Macro Insert for: {search_key:?}, currently at Key: {key:?}, Value: {value:?}");

                let node_key = key.clone();
                let node_left = left.clone();
                let node_right = left.clone();

                key_super.send(key).unwrap();
                left_super.send(left).unwrap();
                right_super.send(right).unwrap();

                match search_key.cmp(&node_key) {
                    Ordering::Equal => {
                        println!("Macro Equal - Overwriting value");
                        value_super.send(new_value).unwrap();
                    },
                    Ordering::Less => {
                        println!("Macro Less - Traverse Left");
                        value_super.send(value).unwrap();
                        node_left.set.send((search_key, new_value)).unwrap();
                    },
                    Ordering::Greater => {
                        println!("Macro Greater - Traverse Right");
                        value_super.send(value).unwrap();
                        node_right.set.send((search_key, new_value)).unwrap();
                    },
                }
            },

            |key, value, left, right, get| {
                let search_key = get;
                println!("Macro Searching for: {search_key:?}, currently at Key: {key:?}, Value: {value:?}");

                let node_key = key.clone();
                let node_value = value.clone();
                let node_left = left.clone();
                let node_right = left.clone();

                key_super.send(key).unwrap();
                value_super.send(value).unwrap();
                left_super.send(left).unwrap();
                right_super.send(right).unwrap();

                match search_key.cmp(&node_key) {
                    Ordering::Equal => {
                        println!("Macro Equal - Returning this node value");
                        Some(node_value)
                    },
                    Ordering::Less => {
                        println!("Macro Less - Traverse Left");
                        node_left.get.send_recv(search_key).unwrap()
                    },
                    Ordering::Greater => {
                        println!("Macro Greater - Traverse Right");
                        node_right.get.send_recv(search_key).unwrap()
                    },
                }
            },

            |empty, get| {
                println!("Macro Called get on an empty node");
                empty_super.send(()).unwrap();
                None
            }
        }

        empty.send(()).expect("Setting node as empty");

        // Take the controller handle to keep the junction alive
        let _controller_handle = junction.controller_handle();

        Node { get, set }
    }
}

#[derive(Clone)]
pub struct RustyTree<K, V>
where
    K: 'static + Send + Sync + Clone + Debug + Ord,
    V: 'static + Send + Sync + Clone + Debug,
{
    root: Node<K, V>,
}

impl<K, V> RustyTree<K, V>
where
    K: 'static + Send + Sync + Clone + Debug + Ord,
    V: 'static + Send + Sync + Clone + Debug,
{
    pub fn new() -> Self {
        Self { root: Node::new() }
    }

    pub fn insert(&self, key: K, value: V) -> Option<V> {
        let v = value.clone();
        self.root.set.send((key, value)).ok().map(|_| v)
    }

    pub fn get(&self, key: K) -> Option<V> {
        self.root.get.send_recv(key).ok().flatten()
    }
}
