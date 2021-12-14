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

    let mut charset = HashMap::new();
    charset.insert("(", ")");
    charset.insert("[", "]");
    charset.insert("{", "}");
    charset.insert("<", ">");

    let file_checker = FileChecker::new(charset);

    println!(
        "Part one: {:?}",
        file_checker.total_illegal_characters(&lines)
    );

    println!(
        "Part Two: {:?}",
        file_checker.total_autocompleted_characters(&lines)
    );
}

struct FileChecker<'a> {
    line_checker: LineChecker<'a>,
}

impl<'a> FileChecker<'a> {
    pub fn new(charset: HashMap<&'a str, &'a str>) -> Self {
        Self {
            line_checker: LineChecker { charset },
        }
    }

    pub fn total_illegal_characters(&self, lines: &Vec<Vec<&'a str>>) -> usize {
        let illegal_chars: Vec<String> = lines
            .iter()
            .filter_map(|line| self.line_checker.check_line(line))
            .collect();

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

    pub fn total_autocompleted_characters(&self, lines: &Vec<Vec<&'a str>>) -> usize {
        let mut added_closer_scores: Vec<usize> = lines
            .into_iter()
            .filter(|line| self.line_checker.check_line(line).is_none())
            .map(|line| self.line_checker.complete_line(&line))
            .map(|closer_set| {
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
            })
            .collect();

        added_closer_scores.sort();
        added_closer_scores[added_closer_scores.len() / 2]
    }
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
