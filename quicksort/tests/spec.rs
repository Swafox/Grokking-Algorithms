use std::time::{Duration, Instant};
use rand::prelude::*;

use quicksort::{quicksort_mid, quicksort_random};

#[test]
fn test_quicksort_mid() {
    let input: Vec<i32> = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
    let sorted: Vec<i32> = quicksort_mid(input);
    assert_eq!(sorted, vec![1, 1, 2, 3, 3, 4, 5, 5, 5, 6, 9]);
}

#[test]
fn test_quicksort_random() {
    let input: Vec<i32> = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
    let sorted: Vec<i32> = quicksort_random(input);
    assert_eq!(sorted, vec![1, 1, 2, 3, 3, 4, 5, 5, 5, 6, 9]);
}

#[test]
fn test_quicksort_empty_list() {
    let input: Vec<i32> = vec![];
    assert_eq!(quicksort_mid(input.clone()), vec![]);
    assert_eq!(quicksort_random(input), vec![]);
}

#[test]
fn test_quicksort_single_element() {
    let input: Vec<i32> = vec![1];
    assert_eq!(quicksort_mid(input.clone()), vec![1]);
    assert_eq!(quicksort_random(input), vec![1]);
}

fn benchmark_sort(arr: &[i32], iterations: u32, sort_fn: fn(Vec<i32>) -> Vec<i32>) -> Duration {
    let mut total_duration: Duration = Duration::new(0, 0);

    for _ in 0..iterations {
        let input: Vec<i32> = arr.to_vec();
        let start: Instant = Instant::now();
        let _ = sort_fn(input);
        total_duration += start.elapsed();
    }

    total_duration / iterations
}

#[test]
fn test_quicksort_performance() {
    let mut rng: ThreadRng = rand::thread_rng();
    let mut input: Vec<i32> = (0..10_000).collect();
    input.shuffle(&mut rng);

    let iterations: u32 = 100;

    let mid_time: Duration = benchmark_sort(&input, iterations, quicksort_mid);
    let random_time: Duration = benchmark_sort(&input, iterations, quicksort_random);

    println!("Quicksort Benchmark:");
    println!("Average time for quicksort_mid over {} iterations: {:?}", iterations, mid_time);
    println!("Average time for quicksort_random over {} iterations: {:?}", iterations, random_time);
}
