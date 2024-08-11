fn quicksort(mut list: Vec<i32>) -> Vec<i32> {
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

    let mut result: Vec<i32> = quicksort(left);
    result.push(pivot);
    result.extend(quicksort(right));

    result
}

fn main() {
    let input: Vec<i32> = vec![1, 3, 2, 5, 6, 4, 9, 8, 7];
    let sorted: Vec<i32> = quicksort(input);
    println!("{:?}", sorted);
}
