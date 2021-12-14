use std::{collections::HashMap, ops::AddAssign};

pub fn main() {
    let file = include_str!("../input.txt");
    let lines: Vec<Vec<&str>> = file
        .split("\n")
        .filter_map(|reading_set| match !reading_set.is_empty() {
            true => Some(
                reading_set
                    .split("")
                    .filter(|char| !char.is_empty())
                    .collect(),
            ),
            false => None,
        })
        .collect();

    let mut part_one_charset = HashMap::new();
    part_one_charset.insert("(", ")");
    part_one_charset.insert("[", "]");
    part_one_charset.insert("{", "}");
    part_one_charset.insert("<", ">");

    let line_checker = LineChecker {
        charset: part_one_charset,
    };

    let illegal_chars: Vec<String> = lines
        .iter()
        .filter_map(|line| line_checker.check_line(line))
        .collect();

    println!("Part one: {}", total_illegal_chars(&illegal_chars));

    let incomplete_lines: Vec<Vec<&str>> = lines
        .into_iter()
        .filter(|line| line_checker.check_line(line).is_none())
        .collect();

    let added_closer_sets: Vec<Vec<String>> = incomplete_lines
        .iter()
        .map(|line| line_checker.complete_line(line))
        .collect();

    let mut added_closer_scores: Vec<usize> = added_closer_sets
        .iter()
        .map(|closer_set| total_added_closers(closer_set))
        .collect();

    added_closer_scores.sort();

    println!(
        "Part Two: {:?}",
        added_closer_scores[added_closer_scores.len() / 2]
    )
}

fn total_added_closers(closer_set: &Vec<String>) -> usize {
    closer_set.iter().fold(0, |acc, closer| {
        let addition = match closer.as_str() {
            ")" => 1,
            "]" => 2,
            "}" => 3,
            ">" => 4,
            _ => panic!("Illegal closer: {}", closer),
        };

        (acc * 5) + addition
    })
}

pub fn total_illegal_chars(illegal_chars: &Vec<String>) -> usize {
    let mut character_counts: HashMap<&String, usize> = HashMap::new();
    illegal_chars
        .iter()
        .for_each(|character| character_counts.entry(character).or_insert(0).add_assign(1));

    character_counts.iter().fold(0, |acc, (key, value)| {
        let multiplier = match key.as_str() {
            ")" => 3,
            "]" => 57,
            "}" => 1197,
            ">" => 25137,
            _ => panic!("Illegal illegal character: {}", value),
        };

        acc + (multiplier * value)
    })
}

struct LineChecker<'a> {
    charset: HashMap<&'a str, &'a str>,
}

impl<'a> LineChecker<'a> {
    pub fn check_line(&'a self, line: &Vec<&str>) -> Option<String> {
        let mut char_stack: Vec<&str> = vec![];
        for character in line.iter() {
            match self
                .charset
                .keys()
                .collect::<Vec<&&str>>()
                .contains(&character)
            {
                true => char_stack.push(character),
                false => {
                    let most_recent_character = char_stack.last().unwrap();
                    match self.charset.get(most_recent_character).unwrap() == character {
                        true => {
                            char_stack.pop();
                        }
                        false => return Some(character.to_string()),
                    }
                }
            }
        }
        None
    }

    pub fn complete_line(&'a self, line: &Vec<&str>) -> Vec<String> {
        let mut char_stack: Vec<&str> = vec![];

        for character in line.iter() {
            match self
                .charset
                .keys()
                .collect::<Vec<&&str>>()
                .contains(&character)
            {
                true => char_stack.push(character),
                false => {
                    let most_recent_character = char_stack.last().unwrap();
                    if self.charset.get(most_recent_character).unwrap() == character {
                        char_stack.pop();
                    }
                }
            }
        }

        char_stack.reverse();
        char_stack
            .iter()
            .map(|character| self.charset.get(character).unwrap().to_string())
            .collect()
    }
}
