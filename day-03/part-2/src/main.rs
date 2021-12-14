fn main() {
    let filename = include_str!("../input.txt");
    let readings: Vec<&str> = filename.split("\n").collect();
    let mut maybe_oxygen = readings.clone();
    let mut maybe_co2 = readings;

    for column_number in 0..12 {
        let split: (Vec<&str>, Vec<&str>) = split_binaries(maybe_oxygen, column_number);

        maybe_oxygen = match (split.0.len(), split.1.len()) {
            (zeros, ones) if zeros > ones => split.0,
            (zeros, ones) if zeros < ones => split.1,
            (zeros, ones) if zeros == ones => split.1,
            _ => panic!("This shouldn't happen."),
        };

        if maybe_oxygen.len() == 1 {
            break;
        };
    }

    for column_number in 0..12 {
        let split: (Vec<&str>, Vec<&str>) = split_binaries(maybe_co2, column_number);

        maybe_co2 = match (split.0.len(), split.1.len()) {
            (zeros, ones) if zeros > ones => split.1,
            (zeros, ones) if zeros < ones => split.0,
            (zeros, ones) if zeros == ones => split.0,
            _ => panic!("This shouldn't happen."),
        };

        if maybe_co2.len() == 1 {
            break;
        };
    }

    let oxygen_string = maybe_oxygen.first().unwrap();
    let oxygen = isize::from_str_radix(oxygen_string, 2).unwrap();

    let co2_string = maybe_co2.first().unwrap();
    let co2 = isize::from_str_radix(co2_string, 2).unwrap();

    println!("Part Two: {}", oxygen * co2);
}

fn split_binaries(binaries: Vec<&str>, position: usize) -> (Vec<&str>, Vec<&str>) {
    binaries
        .iter()
        .partition(|binary| binary.chars().nth(position).unwrap() == '1')
}
