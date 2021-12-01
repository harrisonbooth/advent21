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

fn count_depth_groups(depths: Vec<i32>) -> i32 {
    let mut depth_groups = depths
        .windows(3)
        .map(|depth_window| depth_window.iter().sum::<i32>())
        .peekable();

    let mut count = 0;

    while let (Some(next), Some(peeked)) = (depth_groups.next(), depth_groups.peek()) {
        if peeked > &next {
            count += 1
        }
    }

    count
}

fn main() {
    let filename = include_str!("../input.txt");
    let items: Vec<i32> = filename
        .split("\n")
        .map(|row| row.parse::<i32>().unwrap_or_else(|err| panic!("{}", err)))
        .collect();

    let count = count_depth(items.clone());
    println!("Part One: {}", count);

    let count = count_depth_groups(items.clone());
    println!("Part Two: {}", count);
}
