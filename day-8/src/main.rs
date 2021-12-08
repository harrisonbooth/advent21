use std::collections::HashMap;

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
            Reading::new(input, output)
        })
        .collect();

    let reading_set = ReadingSet(readings);

    println!(
        "Total of 1s, 4s, 7s, and 8s in outputs: {:?}",
        reading_set.total_unique_outputs()
    );

    println!(
        "Total of all reading outputs: {:?}",
        reading_set.total_outputs()
    )
}

struct ReadingSet(Vec<Reading>);

impl ReadingSet {
    pub fn total_unique_outputs(&self) -> usize {
        self.0
            .iter()
            .map(|reading| reading.count_unique_outputs())
            .sum()
    }

    pub fn total_outputs(&self) -> usize {
        self.0
            .clone()
            .into_iter()
            .map(|mut reading| {
                let output = reading.calculate_output();
                println!("{:?}", output);
                output
            })
            .sum()
    }
}

#[derive(Clone)]
struct Reading {
    input: Vec<String>,
    output: Vec<String>,
    segment_map: HashMap<usize, String>,
}

impl Reading {
    pub fn new(input: Vec<String>, output: Vec<String>) -> Self {
        Self {
            input,
            output,
            segment_map: HashMap::new(),
        }
    }

    pub fn calculate_output(&mut self) -> usize {
        self.map_uniques();
        self.output
            .iter()
            .map(|code| match code.len() {
                2 | 4 | 3 | 7 => Self::determine_unique_digit(code).unwrap().to_string(),
                _ => self.determine_normal_digit(code).unwrap().to_string(),
            })
            .collect::<String>()
            .parse()
            .unwrap()
    }

    pub fn determine_normal_digit(&self, code: &str) -> Option<usize> {
        match (
            self.count_matches(code, 1),
            self.count_matches(code, 4),
            self.count_matches(code, 7),
        ) {
            (1, 2, 2) => Some(2),
            (2, 3, 3) if code.len() == 5 => Some(3),
            (1, 3, 2) if code.len() == 5 => Some(5),
            (1, 3, 2) => Some(6),
            (2, 4, 3) => Some(9),
            (2, 3, 3) => Some(0),
            (x, y, z) => {
                println!("{:?}", self.segment_map);
                panic!("Cannot determine: {:?}. {:?} matches with 1, {:?} matches with 4, {:?} matches with 7", code, x, y, z)
            }
        }
    }

    pub fn map_uniques(&mut self) {
        self.input
            .iter()
            .for_each(|code| match Self::determine_unique_digit(code) {
                Some(digit) => {
                    self.segment_map.insert(digit, code.to_string());
                }
                None => (),
            })
    }

    pub fn count_unique_outputs(&self) -> usize {
        self.output
            .iter()
            .filter(|code| Self::determine_unique_digit(code).is_some())
            .count()
    }

    fn determine_unique_digit(code: &str) -> Option<usize> {
        match code {
            code if code.len() == 2 => Some(1),
            code if code.len() == 4 => Some(4),
            code if code.len() == 3 => Some(7),
            code if code.len() == 7 => Some(8),
            _ => None,
        }
    }

    fn count_matches(&self, code: &str, number: usize) -> usize {
        code.chars()
            .filter(|char| {
                self.segment_map
                    .get(&number)
                    .unwrap()
                    .matches(|x| char == &x)
                    .count()
                    > 0
            })
            .count()
    }
}
