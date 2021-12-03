fn main() {
    let filename = include_str!("../input.txt");
    let readings: Vec<&str> = filename.split("\n").collect(); // ["01101001", "00101001", "01001000", ...]
    let mut maybe_oxygen = readings.clone();
    let mut maybe_co2 = readings;

    for column_number in 0..12 {
        let split: (Vec<&str>, Vec<&str>) = maybe_oxygen
            .iter()
            .partition(|binary| binary.chars().nth(column_number).unwrap() == '1');

        maybe_oxygen = match (split.0.len(), split.1.len()) {
            (zeros, ones) if zeros > ones => split.0,
            (zeros, ones) if zeros < ones => split.1,
            (zeros, ones) if zeros == ones => split.0,
            _ => panic!("This shouldn't happen."),
        };

        if maybe_oxygen.len() == 1 {
            break;
        };
    }

    let oxygen_string = maybe_oxygen.first().unwrap();
    let oxygen = isize::from_str_radix(oxygen_string, 2).unwrap();

    for column_number in 0..12 {
        let split: (Vec<&str>, Vec<&str>) = maybe_co2
            .iter()
            .partition(|binary| binary.chars().nth(column_number).unwrap() == '1');

        maybe_co2 = match (split.0.len(), split.1.len()) {
            (zeros, ones) if zeros > ones => split.1,
            (zeros, ones) if zeros < ones => split.0,
            (zeros, ones) if zeros == ones => split.1,
            _ => panic!("This shouldn't happen."),
        };

        if maybe_co2.len() == 1 {
            break;
        };
    }

    let co2_string = maybe_co2.first().unwrap();
    let co2 = isize::from_str_radix(co2_string, 2).unwrap();
    print!("{:?}", co2);

    println!("Part Two: {}", oxygen * co2);
}
