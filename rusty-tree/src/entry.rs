use rusty_junctions::{
    channels::{RecvChannel, SendChannel},
    Junction,
};
use std::{fmt::Debug, marker::Send};

pub struct Entry<K, V> {
    junction: Junction,
    key: SendChannel<K>,
    key_put: SendChannel<K>,
    key_get: RecvChannel<K>,
    value: SendChannel<V>,
    value_put: SendChannel<V>,
    value_get: RecvChannel<V>,
    key_and_value_get: RecvChannel<(K, V)>,
}

impl<K, V> std::fmt::Debug for Entry<K, V>
where
    K: Send + Ord + Clone + 'static + Debug,
    V: Send + Clone + 'static + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        println!("GETTING ENTRY PRINT");
        let entry = self.get_key_and_value().map_or_else(
            |e| format!("ERROR: {:?}", e),
            |(k, v)| format!("{:?}, {:?}", k, v),
        );
        println!("WRITING ENTRY TOSTRING");
        write!(f, "Entry( {:?} )", entry)
    }
}

impl<K, V> Entry<K, V>
where
    // TODO: In the future it maybe possible to remove the Debug
    K: Send + Ord + Clone + 'static + Debug,
    V: Send + Clone + 'static + Debug,
{
    pub fn new(initial_key: K, initial_value: V) -> Result<Self, String> {
        let junction = Junction::new();

        let key = junction.send_channel::<K>();
        let key_put = junction.send_channel::<K>();
        let key_get = junction.recv_channel::<K>();

        let value = junction.send_channel::<V>();
        let value_put = junction.send_channel::<V>();
        let value_get = junction.recv_channel::<V>();

        let key_and_value_get = junction.recv_channel::<(K, V)>();

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
                (key, value)
            });

        // Set the value of the key and the value
        key.send(initial_key).map_err(|_| "Error setting Key")?;
        value
            .send(initial_value)
            .map_err(|_| "Error setting Value")?;

        // Construct the Entry, and return it
        Ok(Entry {
            junction,
            key,
            key_put,
            key_get,
            value,
            value_put,
            value_get,
            key_and_value_get,
        })
    }

    pub fn get_key_and_value(&self) -> Result<(K, V), String> {
        self.key_and_value_get
            .recv()
            .map_err(|_| "Failed to get value".to_string())
    }

    pub fn get_key(&self) -> Result<K, String> {
        self.key_get
            .recv()
            .map_err(|_| "Failed to get value".to_string())
    }

    pub fn get_value(&self) -> Result<V, String> {
        self.value_get
            .recv()
            .map_err(|_| "Failed to get value".to_string())
    }

    pub fn set_key(&self, key: K) -> Result<(), String> {
        self.key_put
            .send(key)
            .map_err(|_| "Failed to get value".to_string())
    }

    pub fn set_value(&self, value: V) -> Result<(), String> {
        self.value_put
            .send(value)
            .map_err(|_| "Failed to get value".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::Entry;

    #[test]
    fn create_entry() {
        let entry = Entry::new(1, "Hello".to_string());
        assert!(entry.is_ok());
    }

    #[test]
    fn get_key() {
        let entry = Entry::new(1, "Hello".to_string()).unwrap();
        assert_eq!(Ok(1), entry.get_key());
    }

    #[test]
    fn get_value() {
        let entry = Entry::new(1, "Hello".to_string()).unwrap();
        assert_eq!(Ok("Hello".to_string()), entry.get_value());
    }

    #[test]
    fn set_key_value() {
        let entry = Entry::new(1, "Hello".to_string()).unwrap();
        assert_eq!(Ok("Hello".to_string()), entry.get_value());
        assert!(entry.set_key(10).is_ok());
        assert_eq!(Ok(10), entry.get_key());
    }

    #[test]
    fn set_value() {
        let entry = Entry::new(1, "Hello".to_string()).unwrap();
        assert_eq!(Ok("Hello".to_string()), entry.get_value());
        assert!(entry.set_value("Another string".to_string()).is_ok());
        assert_eq!(Ok("Another string".to_string()), entry.get_value());
    }

    #[test]
    fn get_key_and_value_single_element() {
        let entry = Entry::new(1, "Hello".to_string()).unwrap();
        assert_eq!(Ok((1, "Hello".to_string())), entry.get_key_and_value());
    }
}
