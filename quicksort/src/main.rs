use quicksort::{quicksort_mid, quicksort_random};

fn main() {
    let input: Vec<i32> = vec![1, 3, 2, 5, 6, 4, 9, 8, 7];
    let input2: Vec<i32> = input.clone();

    let sorted: Vec<i32> = quicksort_mid(input);
    let sorted2: Vec<i32> = quicksort_random(input2);

    println!("Sorted with quicksort_mid: {:?}", sorted);
    println!("Sorted with quicksort_random: {:?}", sorted2);
}
