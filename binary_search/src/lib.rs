pub fn binary_search(list: Vec<i32>, item: i32) -> Option<i32> {
    let mut low: usize = 0;
    let mut high: usize = list.len();

    while low < high {
        let mid: usize = low + (high - low) / 2;

        let guess: &i32 = &list[mid];
        if *guess == item {
            return Some(mid as i32);
        } else if *guess > item {
            high = mid;
        } else {
            low = mid + 1;
        }
    }

    None
}