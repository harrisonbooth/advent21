use std::{collections::BTreeMap, ops::AddAssign};

fn main() {
    let filename = include_str!("../input.txt");
    let mut counts: BTreeMap<usize, usize> = BTreeMap::new();
    let readings: Vec<&str> = filename.split("\n").collect();
    let reading_count = readings.len();

    readings
        .iter()
        .map(|row| row.chars().enumerate())
        .flatten()
        .filter(|(_, reading)| *reading == '1')
        .for_each(|(index, _)| {
            counts.entry(index).or_insert(1).add_assign(1);
        });

    let gamma_string: String = counts
        .iter()
        .map(|(_, &count)| {
            if count >= (reading_count / 2) {
                "1"
            } else {
                "0"
            }
        })
        .collect();
    let gamma = isize::from_str_radix(gamma_string.as_str(), 2).unwrap();

    let epsilon_string: String = gamma_string
        .chars()
        .map(|bit| if bit == '1' { '0' } else { '1' })
        .collect();
    let epsilon = isize::from_str_radix(epsilon_string.as_str(), 2).unwrap();

    println!("Part One: {}", gamma * epsilon);

    println!("Part Two: {}", 0);
}
