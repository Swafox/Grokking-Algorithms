use std::time::{Duration, Instant};
use rand::prelude::*;

use binary_search::binary_search;

#[test]
fn test_binary_search_found() {
    let input: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    assert_eq!(binary_search(input.clone(), 4), Some(3));
    assert_eq!(binary_search(input.clone(), 1), Some(0));
    assert_eq!(binary_search(input, 10), Some(9));
}

#[test]
fn test_binary_search_not_found() {
    let input: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    assert_eq!(binary_search(input.clone(), 0), None);
    assert_eq!(binary_search(input.clone(), 11), None);
}

#[test]
fn test_binary_search_empty_list() {
    let input: Vec<i32> = vec![];
    assert_eq!(binary_search(input, 1), None);
}

#[test]
fn test_binary_search_single_element() {
    let input: Vec<i32> = vec![1];
    assert_eq!(binary_search(input.clone(), 1), Some(0));
    assert_eq!(binary_search(input, 2), None);
}

fn benchmark_search(arr: &[i32], iterations: u32) -> Duration {
    let mut total_duration: Duration = Duration::new(0, 0);
    let mut rng: ThreadRng = rand::thread_rng();

    for _ in 0..iterations {
        let target: i32 = arr[rng.gen_range(0..arr.len())];
        let start: Instant = Instant::now();
        let _ = binary_search(arr.to_vec(), target);
        total_duration += start.elapsed();
    }

    total_duration / iterations
}

#[test]
fn test_binary_search_performance() {
    let mut rng: ThreadRng = rand::thread_rng();
    let mut input: Vec<i32> = (0..1_000_000).collect();
    input.shuffle(&mut rng);
    input.sort();

    let iterations: u32 = 10_000;

    let search_time: Duration = benchmark_search(&input, iterations);

    println!("Binary Search Benchmark:");
    println!("Average time for binary_search over {} iterations: {:?}", iterations, search_time);
}