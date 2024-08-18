use hash_tables::{
    add_to_hash_map, get_value_from_hash_map, new_hash_map, new_hash_table, remove_from_hash_map,
};
use std::collections::HashMap;

fn main() {
    let mut hash_map: HashMap<i32, i32> = new_hash_map();

    add_to_hash_map(&mut hash_map, 1, 11);
    add_to_hash_map(&mut hash_map, 2, 12);
    add_to_hash_map(&mut hash_map, 3, 13);

    remove_from_hash_map(&mut hash_map, 2);

    if let Some(value) = get_value_from_hash_map(&hash_map, 1) {
        println!("Value for key 1: {}", value);
    }

    let arr = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let hash_table = new_hash_table(arr);
    println!("Hash table: {:?}", hash_table);
}
