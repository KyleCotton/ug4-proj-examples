use rusty_junctions::{
    channels::{RecvChannel, SendChannel},
    types::ControllerHandle,
    Junction,
};
use std::{fmt::Debug, marker::Send};

// TODO: Try using the Node without a box
type InnerNode<K, V> = Option<Node<K, V>>;

pub type Key<K> = Option<K>;
pub type Value<V> = Option<V>;

#[derive(Clone)]
pub struct Node<K, V> {
    key: SendChannel<Key<K>>,
    key_put: SendChannel<Key<K>>,
    key_get: RecvChannel<Key<K>>,

    value: SendChannel<Value<V>>,
    value_put: SendChannel<Value<V>>,
    value_get: RecvChannel<Value<V>>,

    key_and_value_get: RecvChannel<Option<(K, V)>>,

    left: SendChannel<InnerNode<K, V>>,
    left_put: SendChannel<InnerNode<K, V>>,
    left_get: RecvChannel<InnerNode<K, V>>,

    right: SendChannel<InnerNode<K, V>>,
    right_put: SendChannel<InnerNode<K, V>>,
    right_get: RecvChannel<InnerNode<K, V>>,
}

impl<K, V> Node<K, V>
where
    // TODO: In the future it maybe possible to remove the Debug
    K: Send + Ord + Clone + 'static + Debug,
    V: Send + Clone + 'static + Debug,
{
    pub fn construct_node(
        initial_key: Option<K>,
        initial_value: Option<V>,
    ) -> Result<Self, String> {

        // We require that the Junction is immortal.
        // When the `Junction` is `drop`ped it will send a `ShutDownRequest` to
        // the `Controller` by the `ControllerHandle`, if there is one.  To allow
        // the `Junctions` to be immortal we simply take the `ControllerHandle`
        // from the `Junction` and let it get dropped.
        // So that the `ShutDownRequest` is never sent to the `Controller`.
        let mut junction = Junction::new();
        let _controller_handle = junction
            .controller_handle()
            .ok_or_else(|| "Failed to get ControllerHandle".to_string())?;
        let junction = junction;

        let key = junction.send_channel::<Key<K>>();
        let key_put = junction.send_channel::<Key<K>>();
        let key_get = junction.recv_channel::<Key<K>>();

        let value = junction.send_channel::<Value<V>>();
        let value_put = junction.send_channel::<Value<V>>();
        let value_get = junction.recv_channel::<Value<V>>();

        let left = junction.send_channel::<InnerNode<K, V>>();
        let left_put = junction.send_channel::<InnerNode<K, V>>();
        let left_get = junction.recv_channel::<InnerNode<K, V>>();

        let right = junction.send_channel::<InnerNode<K, V>>();
        let right_put = junction.send_channel::<InnerNode<K, V>>();
        let right_get = junction.recv_channel::<InnerNode<K, V>>();

        let key_and_value_get = junction.recv_channel::<Option<(K, V)>>();

        // Update the Key value
        let key_clone = key.clone();
        junction.when(&key).and(&key_put).then_do(move |old, new| {
            println!("Updating the Key Value {:?} --> {:?}", old, new);
            key_clone.send(new).unwrap();
        });

        // Get the Key value
        let key_clone = key.clone();
        junction.when(&key).and_recv(&key_get).then_do(move |key| {
            println!("Getting the Key Value {:?}", key);
            key_clone.send(key.clone()).unwrap();
            key
        });

        // Update the Value value
        let value_clone = value.clone();
        junction
            .when(&value)
            .and(&value_put)
            .then_do(move |old, new| {
                println!("Updating the Value {:?} --> {:?}", old, new);
                value_clone.send(new).unwrap();
            });

        // Get the Value value
        let value_clone = value.clone();
        junction
            .when(&value)
            .and_recv(&value_get)
            .then_do(move |value| {
                println!("Getting the Value {:?}", value);
                value_clone.send(value.clone()).unwrap();
                value
            });

        // Simultaneously get the Key and Value
        let key_clone = key.clone();
        let value_clone = value.clone();
        junction
            .when(&key)
            .and(&value)
            .and_recv(&key_and_value_get)
            .then_do(move |key, value| {
                println!("Getting the Key & Value ({:?}, {:?})", key, value);
                key_clone.send(key.clone()).unwrap();
                value_clone.send(value.clone()).unwrap();
                Some((key.unwrap(), value.unwrap()))
            });

        // Update the left value
        let left_clone = left.clone();
        junction
            .when(&left)
            .and(&left_put)
            .then_do(move |_old, new| {
                println!("Updating the Left");
                left_clone.send(new).unwrap();
            });

        // Get the left value
        let left_clone = left.clone();
        junction
            .when(&left)
            .and_recv(&left_get)
            .then_do(move |value| {
                println!("Getting the Left");
                left_clone.send(value.clone()).unwrap();
                value
            });

        // Update the right value
        let right_clone = right.clone();
        junction
            .when(&right)
            .and(&right_put)
            .then_do(move |_old, new| {
                println!("Updating the Right");
                right_clone.send(new).unwrap();
            });

        // Get the right value
        let right_clone = right.clone();
        junction
            .when(&right)
            .and_recv(&right_get)
            .then_do(move |value| {
                println!("Getting the Right");
                right_clone.send(value.clone()).unwrap();
                value
            });

        // Set the value of the key, value, left, and right
        key.send(initial_key).map_err(|_| "Error setting Key")?;
        value
            .send(initial_value)
            .map_err(|_| "Error setting Value")?;
        left.send(None).map_err(|_| "Error setting Left")?;
        right.send(None).map_err(|_| "Error setting Right")?;

        Ok(Self {
            key,
            key_put,
            key_get,
            value,
            value_put,
            value_get,
            key_and_value_get,
            left,
            left_get,
            left_put,
            right,
            right_get,
            right_put,
        })
    }

    // Need to be able to support an empty tree, with a non-mutable reference
    pub fn empty_node() -> Result<Self, String> {
        Self::construct_node(None, None)
    }

    pub fn new(initial_key: K, initial_value: V) -> Result<Self, String> {
        Self::construct_node(Some(initial_key), Some(initial_value))
    }

    pub fn get_key_and_value(&self) -> Result<Option<(K, V)>, String> {
        self.key_and_value_get
            .recv()
            .map_err(|_| "Failed to get value".to_string())
    }

    pub fn get_key(&self) -> Result<Key<K>, String> {
        self.key_get
            .recv()
            .map_err(|_| "Failed to get value".to_string())
    }

    pub fn get_value(&self) -> Result<Value<V>, String> {
        self.value_get
            .recv()
            .map_err(|_| "Failed to get value".to_string())
    }

    pub fn get_left(&self) -> Result<InnerNode<K, V>, String> {
        self.left_get
            .recv()
            .map_err(|_| "Failed to get left".to_string())
    }

    pub fn get_right(&self) -> Result<InnerNode<K, V>, String> {
        self.right_get
            .recv()
            .map_err(|_| "Failed to get right".to_string())
    }

    pub fn set_key(&self, key: K) -> Result<(), String> {
        self.key_put
            .send(Some(key))
            .map_err(|_| "Failed to get value".to_string())
    }

    pub fn set_value(&self, value: V) -> Result<(), String> {
        self.value_put
            .send(Some(value))
            .map_err(|_| "Failed to get value".to_string())
    }
}

impl<K, V> std::fmt::Debug for Node<K, V>
where
    K: Send + Ord + Clone + 'static + Debug,
    V: Send + Clone + 'static + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let key_value = match self.get_key_and_value() {
            Ok(Some((key, value))) => format!("Key: {:?}, Value: {:?}", key, value),
            Ok(None) => "Empty Node".to_string(),
            Err(e) => e.to_string(),
        };

        write!(f, "NODE: {{ {} }}", key_value)
    }
}

#[cfg(test)]
mod tests {
    use super::Node;

    #[test]
    fn create_node() {
        let node = Node::new(1, "Hello".to_string());
        assert!(node.is_ok());
    }

    #[test]
    fn get_key() {
        let node = Node::new(1, "Hello".to_string()).unwrap();
        assert_eq!(Ok(Some(1)), node.get_key());
    }

    #[test]
    fn get_value() {
        let node = Node::new(1, "Hello".to_string()).unwrap();
        assert_eq!(Ok(Some("Hello".to_string())), node.get_value());
    }

    #[test]
    fn set_key_value() {
        let node = Node::new(1, "Hello".to_string()).unwrap();
        assert_eq!(Ok(Some("Hello".to_string())), node.get_value());
        assert!(node.set_key(10).is_ok());
        assert_eq!(Ok(Some(10)), node.get_key());
    }

    #[test]
    fn set_value() {
        let node = Node::new(1, "Hello".to_string()).unwrap();
        assert_eq!(Ok(Some("Hello".to_string())), node.get_value());
        assert!(node.set_value("Another string".to_string()).is_ok());
        assert_eq!(Ok(Some("Another string".to_string())), node.get_value());
    }

    // #[test]
    // fn get_key_and_value_single_element() {
    //     let node = Node::new(1, "Hello".to_string()).unwrap();
    //     assert_eq!(Ok(Some((1, "Hello".to_string()))), node.get_key_and_value());
    // }
}
