mod macro_tree {
    use rusty_junctions::{
        channels::{BidirChannel, SendChannel},
        junction,
    };
    use std::{cmp::Ordering, fmt::Debug};
    struct Node<K, V>
    where
        K: Send + Sync + 'static + Clone + Debug + Ord,
        V: Send + Sync + 'static + Clone + Debug,
    {
        pub get: BidirChannel<K, Option<V>>,
        pub set: SendChannel<(K, V)>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<K: ::core::clone::Clone, V: ::core::clone::Clone> ::core::clone::Clone for Node<K, V>
    where
        K: Send + Sync + 'static + Clone + Debug + Ord,
        V: Send + Sync + 'static + Clone + Debug,
    {
        #[inline]
        fn clone(&self) -> Node<K, V> {
            match *self {
                Node {
                    get: ref __self_0_0,
                    set: ref __self_0_1,
                } => Node {
                    get: ::core::clone::Clone::clone(&(*__self_0_0)),
                    set: ::core::clone::Clone::clone(&(*__self_0_1)),
                },
            }
        }
    }
    impl<K, V> Node<K, V>
    where
        K: Send + Sync + 'static + Clone + Debug + Ord,
        V: Send + Sync + 'static + Clone + Debug,
    {
        pub fn new() -> Self {
            let junction_c20038b80be547e381c97e4fc7631553 = rusty_junctions::Junction::new();
            let key = junction_c20038b80be547e381c97e4fc7631553.send_channel::<K>();
            let value = junction_c20038b80be547e381c97e4fc7631553.send_channel::<V>();
            let left = junction_c20038b80be547e381c97e4fc7631553.send_channel::<Node<K, V>>();
            let right = junction_c20038b80be547e381c97e4fc7631553.send_channel::<Node<K, V>>();
            let empty = junction_c20038b80be547e381c97e4fc7631553.send_channel::<()>();
            let get = junction_c20038b80be547e381c97e4fc7631553.bidir_channel::<K, Option<V>>();
            let set = junction_c20038b80be547e381c97e4fc7631553.send_channel::<(K, V)>();
            #[allow(unused_variables)]
            let key_super = key.clone();
            #[allow(unused_variables)]
            let value_super = value.clone();
            #[allow(unused_variables)]
            let left_super = left.clone();
            #[allow(unused_variables)]
            let right_super = right.clone();
            #[allow(unused_variables)]
            let empty_super = empty.clone();
            #[allow(unused_variables)]
            let get_super = get.clone();
            #[allow(unused_variables)]
            let set_super = set.clone();
            junction_c20038b80be547e381c97e4fc7631553
                .when(&empty)
                .and(&set)
                .then_do(move |empty, set| {
                    let (new_key, new_value) = set;
                    {
                        ::std::io::_print(::core::fmt::Arguments::new_v1(
                            &["Macro Insert: Node Empty - Initialising\n"],
                            &[],
                        ));
                    };
                    key_super.send(new_key).unwrap();
                    value_super.send(new_value).unwrap();
                    left_super.send(Self::new()).unwrap();
                    right_super.send(Self::new()).unwrap();
                });
            #[allow(unused_variables)]
            let key_super = key.clone();
            #[allow(unused_variables)]
            let value_super = value.clone();
            #[allow(unused_variables)]
            let left_super = left.clone();
            #[allow(unused_variables)]
            let right_super = right.clone();
            #[allow(unused_variables)]
            let empty_super = empty.clone();
            #[allow(unused_variables)]
            let get_super = get.clone();
            #[allow(unused_variables)]
            let set_super = set.clone();
            junction_c20038b80be547e381c97e4fc7631553
                .when(&key)
                .and(&value)
                .and(&left)
                .and(&right)
                .and(&set)
                .then_do(move |key, value, left, right, set| {
                    let (search_key, new_value) = set;
                    {
                        ::std::io::_print(::core::fmt::Arguments::new_v1(
                            &[
                                "Macro Insert for: ",
                                ", currently at Key: ",
                                ", Value: ",
                                "\n",
                            ],
                            &[
                                ::core::fmt::ArgumentV1::new_debug(&search_key),
                                ::core::fmt::ArgumentV1::new_debug(&key),
                                ::core::fmt::ArgumentV1::new_debug(&value),
                            ],
                        ));
                    };
                    let node_key = key.clone();
                    let node_left = left.clone();
                    let node_right = left.clone();
                    key_super.send(key).unwrap();
                    left_super.send(left).unwrap();
                    right_super.send(right).unwrap();
                    match search_key.cmp(&node_key) {
                        Ordering::Equal => {
                            {
                                ::std::io::_print(::core::fmt::Arguments::new_v1(
                                    &["Macro Equal - Overwriting value\n"],
                                    &[],
                                ));
                            };
                            value_super.send(new_value).unwrap();
                        }
                        Ordering::Less => {
                            {
                                ::std::io::_print(::core::fmt::Arguments::new_v1(
                                    &["Macro Less - Traverse Left\n"],
                                    &[],
                                ));
                            };
                            value_super.send(value).unwrap();
                            node_left.set.send((search_key, new_value)).unwrap();
                        }
                        Ordering::Greater => {
                            {
                                ::std::io::_print(::core::fmt::Arguments::new_v1(
                                    &["Macro Greater - Traverse Right\n"],
                                    &[],
                                ));
                            };
                            value_super.send(value).unwrap();
                            node_right.set.send((search_key, new_value)).unwrap();
                        }
                    }
                });
            #[allow(unused_variables)]
            let key_super = key.clone();
            #[allow(unused_variables)]
            let value_super = value.clone();
            #[allow(unused_variables)]
            let left_super = left.clone();
            #[allow(unused_variables)]
            let right_super = right.clone();
            #[allow(unused_variables)]
            let empty_super = empty.clone();
            #[allow(unused_variables)]
            let get_super = get.clone();
            #[allow(unused_variables)]
            let set_super = set.clone();
            junction_c20038b80be547e381c97e4fc7631553
                .when(&key)
                .and(&value)
                .and(&left)
                .and(&right)
                .and_bidir(&get)
                .then_do(move |key, value, left, right, get| {
                    let search_key = get;
                    {
                        ::std::io::_print(::core::fmt::Arguments::new_v1(
                            &[
                                "Macro Searching for: ",
                                ", currently at Key: ",
                                ", Value: ",
                                "\n",
                            ],
                            &[
                                ::core::fmt::ArgumentV1::new_debug(&search_key),
                                ::core::fmt::ArgumentV1::new_debug(&key),
                                ::core::fmt::ArgumentV1::new_debug(&value),
                            ],
                        ));
                    };
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
                            {
                                ::std::io::_print(::core::fmt::Arguments::new_v1(
                                    &["Macro Equal - Returning this node value\n"],
                                    &[],
                                ));
                            };
                            Some(node_value)
                        }
                        Ordering::Less => {
                            {
                                ::std::io::_print(::core::fmt::Arguments::new_v1(
                                    &["Macro Less - Traverse Left\n"],
                                    &[],
                                ));
                            };
                            node_left.get.send_recv(search_key).unwrap()
                        }
                        Ordering::Greater => {
                            {
                                ::std::io::_print(::core::fmt::Arguments::new_v1(
                                    &["Macro Greater - Traverse Right\n"],
                                    &[],
                                ));
                            };
                            node_right.get.send_recv(search_key).unwrap()
                        }
                    }
                });
            #[allow(unused_variables)]
            let key_super = key.clone();
            #[allow(unused_variables)]
            let value_super = value.clone();
            #[allow(unused_variables)]
            let left_super = left.clone();
            #[allow(unused_variables)]
            let right_super = right.clone();
            #[allow(unused_variables)]
            let empty_super = empty.clone();
            #[allow(unused_variables)]
            let get_super = get.clone();
            #[allow(unused_variables)]
            let set_super = set.clone();
            junction_c20038b80be547e381c97e4fc7631553
                .when(&empty)
                .and_bidir(&get)
                .then_do(move |empty, get| {
                    {
                        ::std::io::_print(::core::fmt::Arguments::new_v1(
                            &["Macro Called get on an empty node\n"],
                            &[],
                        ));
                    };
                    empty_super.send(()).unwrap();
                    None
                });
            let mut junction = junction_c20038b80be547e381c97e4fc7631553;
            empty.send(()).expect("Setting node as empty");
            let _controller_handle = junction.controller_handle();
            Node { get, set }
        }
    }
    pub struct RustyTree<K, V>
    where
        K: 'static + Send + Sync + Clone + Debug + Ord,
        V: 'static + Send + Sync + Clone + Debug,
    {
        root: Node<K, V>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<K: ::core::clone::Clone, V: ::core::clone::Clone> ::core::clone::Clone for RustyTree<K, V>
    where
        K: 'static + Send + Sync + Clone + Debug + Ord,
        V: 'static + Send + Sync + Clone + Debug,
    {
        #[inline]
        fn clone(&self) -> RustyTree<K, V> {
            match *self {
                RustyTree {
                    root: ref __self_0_0,
                } => RustyTree {
                    root: ::core::clone::Clone::clone(&(*__self_0_0)),
                },
            }
        }
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
}

fn main() {
    let tree = RustyTree::new();
}
