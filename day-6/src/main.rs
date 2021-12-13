fn main() {
    let file = include_str!("../input.txt");
    let fishes: Vec<usize> = file
        .split(",")
        .filter_map(|fish_str| match !fish_str.is_empty() {
            true => Some(fish_str.parse().unwrap()),
            false => None,
        })
        .collect();

    let mut age_counts: [usize; 9] = [0, 0, 0, 0, 0, 0, 0, 0, 0];
    fishes.iter().for_each(|&fish| age_counts[fish] += 1);

    let days = 256;

    for _ in 0..days {
        let birthing = age_counts[0];
        age_counts[0] = 0;
        age_counts.rotate_left(1);
        age_counts[8] = birthing;
        age_counts[6] += birthing;
    }

    let total_fish: usize = age_counts.iter().sum();

    println!("Fish after {} days: {}", days, total_fish);
}
