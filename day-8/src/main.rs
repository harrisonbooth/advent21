fn main() {
    let file = include_str!("../input.txt");
    let readings_strs: Vec<(&str, &str)> = file
        .split("\n")
        .filter_map(|reading_set| match !reading_set.is_empty() {
            true => Some(reading_set.split_once(" | ").unwrap()),
            false => None,
        })
        .collect();

    let readings: Vec<Reading> = readings_strs
        .iter()
        .map(|reading_str_tuple| {
            let input: Vec<String> = reading_str_tuple
                .0
                .split(" ")
                .filter_map(|input| match !input.is_empty() {
                    true => Some(input.to_string()),
                    false => None,
                })
                .collect();
            let output: Vec<String> = reading_str_tuple
                .1
                .split(" ")
                .filter_map(|output| match !output.is_empty() {
                    true => Some(output.to_string()),
                    false => None,
                })
                .collect();
            Reading { input, output }
        })
        .collect();

    let reading_set = ReadingSet(readings);

    println!(
        "Total of 1s, 4s, 7s, and 8s in outputs: {:?}",
        reading_set.total_unique_outputs()
    )
}

struct ReadingSet(Vec<Reading>);

impl ReadingSet {
    pub fn new(readings: Vec<Reading>) -> Self {
        Self(readings)
    }

    pub fn total_unique_outputs(&self) -> usize {
        self.0
            .iter()
            .map(|reading| reading.count_unique_outputs())
            .sum()
    }
}

struct Reading {
    input: Vec<String>,
    output: Vec<String>,
}

impl Reading {
    pub fn count_unique_outputs(&self) -> usize {
        self.output
            .iter()
            .filter(|code| Self::determine_digit(code).is_some())
            .count()
    }

    fn determine_digit(reading: &str) -> Option<isize> {
        match reading {
            reading if reading.len() == 2 => Some(1),
            reading if reading.len() == 4 => Some(4),
            reading if reading.len() == 3 => Some(7),
            reading if reading.len() == 7 => Some(8),
            _ => None,
        }
    }
}
