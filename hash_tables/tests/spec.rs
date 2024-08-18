use rand::prelude::*;
use std::collections::HashMap;
use std::time::{Duration, Instant};

use hash_tables::{
    add_to_hash_map, get_value_from_hash_map, new_hash_map, new_hash_table, remove_from_hash_map,
};

#[test]
fn test_new_hash_map() {
    let hash_map: HashMap<i32, i32> = new_hash_map();
    assert_eq!(hash_map.len(), 0);
}

#[test]
fn test_add_and_get_from_hash_map() {
    let mut hash_map: HashMap<i32, i32> = new_hash_map();
    add_to_hash_map(&mut hash_map, 1, 11);
    add_to_hash_map(&mut hash_map, 2, 22);
    assert_eq!(get_value_from_hash_map(&hash_map, 1), Some(11));
    assert_eq!(get_value_from_hash_map(&hash_map, 2), Some(22));
}

#[test]
fn test_remove_from_hash_map() {
    let mut hash_map: HashMap<i32, i32> = new_hash_map();
    add_to_hash_map(&mut hash_map, 1, 11);
    add_to_hash_map(&mut hash_map, 2, 22);
    remove_from_hash_map(&mut hash_map, 1);
    assert_eq!(hash_map.contains_key(&1), false);
    assert_eq!(hash_map.contains_key(&2), true);
}

#[test]
fn test_new_hash_table() {
    let input: Vec<i32> = vec![1, 2, 3, 4, 5];
    let hash_table: HashMap<i32, i32> = new_hash_table(input);
    assert_eq!(hash_table.len(), 5);
    for i in 1..=5 {
        assert_eq!(hash_table.get(&i), Some(&i));
    }
}

fn benchmark_hash_table_operations(size: usize, iterations: u32) -> Duration {
    let mut rng: ThreadRng = rand::thread_rng();
    let mut total_duration: Duration = Duration::new(0, 0);

    for _ in 0..iterations {
        let mut hash_map: HashMap<i32, i32> = new_hash_map();
        let start: Instant = Instant::now();

        // Insert operations
        for _ in 0..size {
            let key: i32 = rng.gen();
            let value: i32 = rng.gen();
            add_to_hash_map(&mut hash_map, key, value);
        }

        // Get operations
        for _ in 0..size {
            let key: i32 = rng.gen_range(0..size as i32);
            let _ = hash_map.get(&key);
        }

        // Remove operations
        for _ in 0..size / 2 {
            let key: i32 = rng.gen_range(0..size as i32);
            remove_from_hash_map(&mut hash_map, key);
        }

        total_duration += start.elapsed();
    }

    total_duration / iterations
}

#[test]
fn test_hash_table_performance() {
    let sizes: Vec<usize> = vec![1_000, 10_000, 100_000];
    let iterations: u32 = 10;

    println!("Hash Table Performance Benchmark:");
    for size in sizes {
        let avg_time: Duration = benchmark_hash_table_operations(size, iterations);
        println!(
            "Average time for {} operations over {} iterations: {:?}",
            size, iterations, avg_time
        );
    }
}
