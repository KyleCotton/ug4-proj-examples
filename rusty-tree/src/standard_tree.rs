use rusty_junctions::{
    channels::{BidirChannel, SendChannel},
    Junction,
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
        let mut junction = Junction::new();
        let key = junction.send_channel::<K>();
        let value = junction.send_channel::<V>();
        let left = junction.send_channel::<Node<K, V>>();
        let right = junction.send_channel::<Node<K, V>>();
        let empty = junction.send_channel::<()>();

        let get = junction.bidir_channel::<K, Option<V>>();
        let set = junction.send_channel::<(K, V)>();

        let key_clone = key.clone();
        let value_clone = value.clone();
        let left_clone = left.clone();
        let right_clone = right.clone();
        junction
            .when(&empty)
            .and(&set)
            .then_do(move |_empty, (new_key, new_value)| {
                println!("Insert: Node Empty - Initialising");
                key_clone.send(new_key).unwrap();
                value_clone.send(new_value).unwrap();
                left_clone.send(Self::new()).unwrap();
                right_clone.send(Self::new()).unwrap();
            });

        let key_clone = key.clone();
        let value_clone = value.clone();
        let left_clone = left.clone();
        let right_clone = right.clone();
        junction
            .when(&key)
            .and(&value)
            .and(&left)
            .and(&right)
            .and(&set)
            .then_do(move |current_key, current_value, current_left, current_right, ( search_key, new_value ) | {
                println!("Insert for: {search_key:?}, currently at Key: {current_key:?}, Value: {current_value:?}");

                let node_key = current_key.clone();
                let node_left = current_left.clone();
                let node_right = current_left.clone();

                key_clone.send(current_key).unwrap();
                left_clone.send(current_left).unwrap();
                right_clone.send(current_right).unwrap();

                match search_key.cmp(&node_key) {
                    Ordering::Equal => {
                        println!("Equal - Overwriting value");
                        value_clone.send(new_value).unwrap();
                    },
                    Ordering::Less => {
                        println!("Less - Traverse Left");
                        value_clone.send(current_value).unwrap();
                        node_left.set.send((search_key, new_value)).unwrap();
                    },
                    Ordering::Greater => {
                        println!("Greater - Traverse Right");
                        value_clone.send(current_value).unwrap();
                        node_right.set.send((search_key, new_value)).unwrap();
                    },
                }
            });

        let key_clone = key.clone();
        let value_clone = value.clone();
        let left_clone = left.clone();
        let right_clone = right.clone();
        junction
            .when(&key)
            .and(&value)
            .and(&left)
            .and(&right)
            .and_bidir(&get)
            .then_do(move |current_key, current_value, current_left, current_right, search_key | {
                println!("Searching for: {search_key:?}, currently at Key: {current_key:?}, Value: {current_value:?}");
                let node_key = current_key.clone();
                let node_value = current_value.clone();
                let node_left = current_left.clone();
                let node_right = current_left.clone();

                key_clone.send(current_key).unwrap();
                value_clone.send(current_value).unwrap();
                left_clone.send(current_left).unwrap();
                right_clone.send(current_right).unwrap();

                match search_key.cmp(&node_key) {
                    Ordering::Equal => {
                        println!("Equal - Returning this node value");
                        Some(node_value)
                    },
                    Ordering::Less => {
                        println!("Less - Traverse Left");
                        node_left.get.send_recv(search_key).unwrap()
                    },
                    Ordering::Greater => {
                        println!("Greater - Traverse Right");
                        node_right.get.send_recv(search_key).unwrap()
                    },
                }
            });

        let empty_clone = empty.clone();
        junction
            .when(&empty)
            .and_bidir(&get)
            .then_do(move |_empty, _search_key| {
                println!("Called get on an empty node");
                empty_clone.send(()).unwrap();
                None
            });

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
