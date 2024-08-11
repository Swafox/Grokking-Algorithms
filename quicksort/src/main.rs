use rand::Rng;

fn quicksort_mid(mut list: Vec<i32>) -> Vec<i32> {
    if list.len() < 2 {
        return list;
    }

    let mid: usize = list.len() / 2;

    let pivot: i32 = list.remove(mid);

    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    for item in list {
        if item <= pivot {
            left.push(item);
        } else {
            right.push(item);
        }
    }

    let mut result: Vec<i32> = quicksort_mid(left);
    result.push(pivot);
    result.extend(quicksort_mid(right));

    result
}

fn quicksort_random(mut list: Vec<i32>) -> Vec<i32> {
    if list.len() < 2 {
        return list;
    }

    let pivot: i32 = list.remove(rand::thread_rng().gen_range(0..list.len() as i32) as usize);

    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    for item in list {
        if item <= pivot {
            left.push(item);
        } else {
            right.push(item);
        }
    }

    let mut result: Vec<i32> = quicksort_random(left);
    result.push(pivot);
    result.extend(quicksort_random(right));

    result
}

fn main() {
    let input: Vec<i32> = vec![1, 3, 2, 5, 6, 4, 9, 8, 7];
    let input2: Vec<i32> = input.clone();

    let sorted: Vec<i32> = quicksort_mid(input);
    let sorted2: Vec<i32> = quicksort_random(input2);

    println!("{:?}", sorted);
    println!("{:?}", sorted2);
}
