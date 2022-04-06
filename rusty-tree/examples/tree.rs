fn main() {
    let tree = rusty_tree::RustyTree::new();
    (0..100_000).into_iter().for_each(|_| {
        let key = rand::random::<u64>() % 100;
        let value = rand::random::<u64>();
        let insert_or_get = rand::random::<bool>();

        if insert_or_get {
            let r = tree.insert(key, value);
            // println!("Insert: Key: {key}, Value: {value} ==> {r:?}");
        } else {
            let r = tree.get(key);
            // println!("Get: Key: {key} ==> {r:?}");
        }
    });
}
