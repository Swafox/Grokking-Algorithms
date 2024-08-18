use std::collections::HashMap;

fn new_hash_map() -> HashMap<i32, i32> {
    let hash_map = HashMap::new();
    hash_map
}

fn add_to_hash_map(hash_map: &mut HashMap<i32, i32>, key: i32, value: i32) {
    hash_map.insert(key, value);
}

fn get_value_from_hash_map(hash_map: &HashMap<i32, i32>, key: i32) -> i32 {
    hash_map.get(&key).unwrap().to_owned()
}

fn remove_from_hash_map(hash_map: &mut HashMap<i32, i32>, key: i32) {
    hash_map.remove(&key);
}

fn new_hash_table(arr: Vec<i32>) -> HashMap<i32, i32> {
    let mut hash_table = HashMap::new();
    for i in arr {
        hash_table.insert(i, i);
    }
    hash_table
}

fn main() {
    let mut hash_map: HashMap<i32, i32> = new_hash_map();

    add_to_hash_map(&mut hash_map, 1, 11);
    add_to_hash_map(&mut hash_map, 2, 12);
    add_to_hash_map(&mut hash_map, 3, 13);

    remove_from_hash_map(&mut hash_map, 2);

    println!("{}", get_value_from_hash_map(&hash_map, 1));

    let arr = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let hash_table = new_hash_table(arr);
    println!("{:?}", hash_table);
}
