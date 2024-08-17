use rand::Rng;

pub fn quicksort_mid(mut list: Vec<i32>) -> Vec<i32> {
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

pub fn quicksort_random(mut list: Vec<i32>) -> Vec<i32> {
    if list.len() < 2 {
        return list;
    }

    let pivot: i32 = list.remove(rand::thread_rng().gen_range(0..list.len()) as usize);

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
