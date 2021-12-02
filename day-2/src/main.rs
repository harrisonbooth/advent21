fn parse_commands(commands: Vec<String>) -> impl Iterator<Item = (String, String)> {
    commands.into_iter().map(|command| {
        let mut split_command = command.split_whitespace();
        match (split_command.next(), split_command.next()) {
            (Some(direction), Some(size)) => (direction.to_string(), size.to_string()),
            _ => panic!("Problem parsing commands."),
        }
    })
}

fn execute_commands(commands: Vec<String>) -> (i32, i32) {
    let mut horizontal = 0;
    let mut depth = 0;

    parse_commands(commands).for_each(|(direction, size)| {
        let int_size: i32 = size.parse().unwrap();
        match direction.as_str() {
            "forward" => horizontal += int_size,
            "up" => depth -= int_size,
            "down" => depth += int_size,
            direction => panic!("Unrecognised direction {}", direction),
        }
    });

    (horizontal, depth)
}

fn execute_updated_commands(commands: Vec<String>) -> (i32, i32) {
    let mut aim = 0;
    let mut horizontal = 0;
    let mut depth = 0;

    parse_commands(commands).for_each(|(direction, size)| {
        let int_size: i32 = size.parse().unwrap();
        match direction.as_str() {
            "forward" => {
                horizontal += int_size;
                depth += aim * int_size;
            }
            "up" => aim -= int_size,
            "down" => aim += int_size,
            direction => panic!("Unrecognised direction {}", direction),
        }
    });

    (horizontal, depth)
}

fn main() {
    let filename = include_str!("../input.txt");
    let items: Vec<String> = filename
        .split("\n")
        .map(|row| row.parse().unwrap_or_else(|err| panic!("{}", err)))
        .collect();

    let (horizontal, depth) = execute_commands(items.clone());
    let part_1_product = horizontal * depth;
    println!("Part One: {}", part_1_product);

    let (horizontal, depth) = execute_updated_commands(items.clone());
    let part_2_product = horizontal * depth;
    println!("Part Two: {}", part_2_product);
}
