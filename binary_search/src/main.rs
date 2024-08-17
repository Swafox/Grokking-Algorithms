use binary_search::binary_search;

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