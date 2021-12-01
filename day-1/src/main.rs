use std::fs;

fn count_depth(depths: Vec<i32>) -> i32 {
    let mut depths = depths.iter().peekable();
    let mut count = 0;

    while let (Some(next), Some(peeked)) = (depths.next(), depths.peek()) {
        if peeked > &next {
            count += 1
        }
    }

    count
}

fn main() {
    let filename = "input.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let items: Vec<i32> = contents
        .split("\n")
        .map(|row| row.parse())
        .filter(|item: &Result<String, _>| item.is_ok())
        .map(|item| item.unwrap().parse().unwrap())
        .collect();

    let count = count_depth(items);
    println!("{}", count);
}
