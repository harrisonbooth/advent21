// Day 6 Part 2
// Aim is to maintain a list of quantities of fish at given ages and maintain it
// as days pass by rotating left and ammending as needed:
//
// Day 0: [0, 4, 2, 3, 1, 1, 0, 0, 0]
// No fish are born.
//
// Day 1: [4, 2, 3, 1, 1, 0, 0, 0, 0]
// No fish are born.
//
// Day 2: [2, 3, 1, 1, 0, 4, 0, 0, 4]
// 4 fish are born, the parents' timers go to 6.
//
// Day 3: [3, 1, 1, 0, 4, 2, 0, 4, 2]
// 2 fish are born, the parents' timers go to 6.
//
// etc

// Due to this solution not increasing the length of any loop iteration
// regardless of day limit, it maintained feasible performance throughout.

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
        // Count the number of fish ready to give birth and clear the timer 0 count
        let birthing = age_counts[0];
        age_counts[0] = 0;
        // Rotate the fish left to reflect decrease in timers
        age_counts.rotate_left(1);
        // Add the new fish to the timer 8 count, add the fish who gave birth to
        // the timer 6 count
        age_counts[8] = birthing;
        age_counts[6] += birthing;
    }

    let total_fish: usize = age_counts.iter().sum();

    println!("Fish after {} days: {}", days, total_fish);
}
