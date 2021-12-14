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
}
