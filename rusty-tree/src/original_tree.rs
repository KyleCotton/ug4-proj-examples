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

#[derive(Clone)]
struct Content<K, V>
where
    K: Send + Sync + 'static + Clone + Debug + Ord,
    V: Send + Sync + 'static + Clone + Debug,
{
    pub key: K,
    pub value: V,
    pub left: Node<K, V>,
    pub right: Node<K, V>,
}

impl<K, V> Node<K, V>
where
    K: Send + Sync + 'static + Clone + Debug + Ord,
    V: Send + Sync + 'static + Clone + Debug,
{
    pub fn new() -> Self {
        let mut junction = Junction::new();
        let content = junction.send_channel::<Content<K, V>>();
        let empty = junction.send_channel::<()>();

        let get = junction.bidir_channel::<K, Option<V>>();
        let set = junction.send_channel::<(K, V)>();

        let content_clone = content.clone();
        junction
            .when(&empty)
            .and(&set)
            .then_do(move |_empty, (key, value)| {
                println!("Insert: Node Empty - Initialising");
                let content = Content {
                    key,
                    value,
                    left: Self::new(),
                    right: Self::new(),
                };
                content_clone.send(content).unwrap();
            });

        let content_clone = content.clone();
        junction
            .when(&content)
            .and(&set)
            .then_do(move |mut content, (search_key, new_value)| {
                println!(
                    "Insert for: {search_key:?}, currently at Key: {:?}, Value: {:?}",
                    content.key, content.value
                );

                match search_key.cmp(&content.key) {
                    Ordering::Equal => {
                        println!("Equal - Overwriting value");
                        content.value = new_value;
                        content_clone.send(content).unwrap();
                    }
                    Ordering::Less => {
                        println!("Less - Traverse Left");
                        let left = content.left.clone();
                        content_clone.send(content).unwrap();
                        left.set.send((search_key, new_value)).unwrap();
                    }
                    Ordering::Greater => {
                        println!("Greater - Traverse Right");
                        let right = content.right.clone();
                        content_clone.send(content).unwrap();
                        right.set.send((search_key, new_value)).unwrap();
                    }
                }
            });

        let content_clone = content.clone();
        junction
            .when(&content)
            .and_bidir(&get)
            .then_do(move |content, search_key| {
                println!(
                    "Searching for: {search_key:?}, currently at Key: {:?}, Value: {:?}",
                    content.key, content.value
                );
                let key = content.key.clone();
                let value = content.value.clone();
                let left = content.left.clone();
                let right = content.right.clone();
                content_clone.send(content).unwrap();

                match search_key.cmp(&key) {
                    Ordering::Equal => {
                        println!("Equal - Returning this node value");
                        Some(value)
                    }
                    Ordering::Less => {
                        println!("Less - Traverse Left");
                        left.get.send_recv(search_key).unwrap()
                    }
                    Ordering::Greater => {
                        println!("Greater - Traverse Right");
                        right.get.send_recv(search_key).unwrap()
                    }
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
