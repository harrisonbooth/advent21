fn main() {
    let file = include_str!("../input.txt");
    let mut positions: Vec<isize> = file
        .split(",")
        .filter_map(|pos_str| match !pos_str.is_empty() {
            true => Some(pos_str.parse().unwrap()),
            false => None,
        })
        .collect();

    positions.sort();
    let mid_point = positions.len() / 2;
    let median: isize = positions[mid_point];

    let fuel: isize = positions
        .iter()
        .map(|position| (position - median).abs())
        .sum();

    println!(
        "Fuel used to get to most efficient position: {} was: {}",
        median, fuel
    );
}
