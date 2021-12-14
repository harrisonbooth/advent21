// Day 6 Part 1
// Aim is to maintain a list of all fish ages and maintain it as days pass:
//
// Day 0: [1, 1, 1, 1, 2, 2, 3, 3, 3, 4, 5]
// No fish are born.
//
// Day 1: [0, 0, 0, 0, 1, 1, 2, 2, 2, 3, 4]
// No fish are born.
//
// Day 2: [6, 6, 6, 6, 0, 0, 1, 1, 1, 2, 3, 8, 8, 8, 8]
// 4 fish are born, the parents' timers go to 6.
//
// Day 3: [5, 5, 5, 5, 6, 6, 0, 0, 0, 1, 2, 7, 7, 7, 7, 8, 8]
// 2 fish are born, the parents' timers go to 6.
//
// etc

// This brute force approach worked when the day limit was 80
//
// Due to its time complexity and poor performance it became unfeasible for part
// two, as the day limit increased to 256.

fn main() {
    let s = include_str!("input.txt");

    let mut fish_counts: Vec<usize> = s
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(|line| {
            line.split(',')
                .map(|value| value.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .flatten()
        .collect();

    for day in 0..80 {
        let new_fish_counts = progress_fish(fish_counts);
        fish_counts = new_fish_counts;
    }

    println!("answer={}", fish_counts.len())
}

fn progress_fish(counts: Vec<usize>) -> Vec<usize> {
    let mut newborn_fish_count = 0;

    // Reduce the timer on existing fish and count newborns
    let older_fish_counts = counts
        .iter()
        .map(|&current| match current {
            0 => {
                newborn_fish_count += 1;
                6
            }
            val => val - 1,
        })
        .collect::<Vec<usize>>();

    // Chain an iterator containing the existing fish with an iterator of newly
    // born fish for the next iteration.
    older_fish_counts
        .into_iter()
        .chain(vec![8; newborn_fish_count].into_iter())
        .collect()
}
