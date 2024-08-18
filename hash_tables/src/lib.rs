use std::collections::HashMap;

pub fn new_hash_map() -> HashMap<i32, i32> {
    HashMap::new()
}

pub fn add_to_hash_map(hash_map: &mut HashMap<i32, i32>, key: i32, value: i32) {
    hash_map.insert(key, value);
}

pub fn get_value_from_hash_map(hash_map: &HashMap<i32, i32>, key: i32) -> Option<i32> {
    hash_map.get(&key).copied()
}

pub fn remove_from_hash_map(hash_map: &mut HashMap<i32, i32>, key: i32) {
    hash_map.remove(&key);
}

pub fn new_hash_table(arr: Vec<i32>) -> HashMap<i32, i32> {
    arr.into_iter().map(|i| (i, i)).collect()
}
