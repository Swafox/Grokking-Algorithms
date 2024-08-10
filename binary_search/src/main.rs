fn binary_search(list: Vec<i32>, item: i32) -> Option<i32> {
    let mut low: usize = 0;
    let mut high: usize = list.len();

    while low < high {
        let mid: usize = (low + high) / 2;

        let guess: &i32 = &list[mid];
        if *guess == item {
            return Some(mid as i32);
        } else if *guess > item {
            high = mid - 1;
        } else {
            low = mid + 1;
        }
    }

    None
}

fn main() {
    let mut input: Vec<i32> = Vec::new();

    input.push(1);
    input.push(2);
    input.push(3);
    input.push(4);
    input.push(5);
    input.push(6);
    input.push(7);
    input.push(8);
    input.push(9);
    input.push(10);

    let searchable: i32 = 4;

    let answer: Option<i32> = binary_search(input, searchable);

    println!("{} is at position: {:?}", searchable, answer)
}
