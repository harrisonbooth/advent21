use std::{collections::BTreeMap, ops::AddAssign};

fn main() {
    let filename = include_str!("../input.txt");
    let mut counts: BTreeMap<usize, usize> = BTreeMap::new();
    let readings: Vec<&str> = filename.split("\n").collect();
    let reading_count = readings.len();

    readings
        .iter()
        .map(|row| {
            row.split("")
                .filter_map(|bit| match bit.is_empty() {
                    true => None,
                    false => Some(bit.parse::<usize>().unwrap()),
                })
                .enumerate()
        })
        .flatten()
        .for_each(|(index, reading)| {
            let count = counts.entry(index).or_insert(reading);
            count.add_assign(reading);
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
        .collect::<Vec<&str>>()
        .join("");
    let gamma = isize::from_str_radix(gamma_string.as_str(), 2).unwrap();

    let epsilon_string: String = counts
        .iter()
        .map(|(_, &count)| {
            if count > (reading_count / 2) {
                "0"
            } else {
                "1"
            }
        })
        .collect::<Vec<&str>>()
        .join("");
    let epsilon = isize::from_str_radix(epsilon_string.as_str(), 2).unwrap();

    println!("Part One: {}", gamma * epsilon);

    println!("Part Two: {}", 0);
}
