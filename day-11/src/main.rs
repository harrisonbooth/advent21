use std::collections::HashSet;

fn main() {
    let file = include_str!("../input.txt");
    let octopi: Vec<Vec<usize>> = file
        .split("\n")
        .filter_map(|line| match !line.is_empty() {
            true => Some(
                line.split("")
                    .filter_map(|light_level| match !light_level.is_empty() {
                        true => Some(light_level.parse::<usize>().unwrap()),
                        false => None,
                    })
                    .collect(),
            ),
            false => None,
        })
        .collect();

    let mut shoal = Shoal { octopi };

    println!("{:?} flashes after 100 steps.", shoal.clone().steps(100));
    println!(
        "First synchronised flash at step: {:?}.",
        shoal.find_synchronised_flash()
    );
}

#[derive(Clone)]
struct Shoal {
    octopi: Vec<Vec<usize>>,
}

impl Shoal {
    pub fn steps(&mut self, steps: usize) -> usize {
        let mut flashes = 0;

        for _ in 1..(steps + 1) {
            flashes += self.step();
        }

        flashes
    }

    pub fn find_synchronised_flash(&mut self) -> usize {
        let mut step = 0;
        let mut flashes = 0;

        while flashes != 100 {
            flashes = self.step();
            step += 1;
        }

        step
    }

    pub fn step(&mut self) -> usize {
        let mut flashing_coords: HashSet<(usize, usize)> = HashSet::new();
        let mut to_increase: Vec<(usize, usize)> = vec![];

        for y in 0..10 {
            for x in 0..10 {
                to_increase.push((x, y));
            }
        }

        while to_increase.len() > 0 {
            let mut new_flashes: Vec<(usize, usize)> = Vec::new();
            to_increase.iter().for_each(|&(x, y)| {
                self.octopi[y][x] += 1;
                if self.octopi[y][x] > 9 && flashing_coords.insert((x, y)) {
                    new_flashes.push((x, y));
                }
            });

            to_increase = new_flashes
                .iter()
                .map(|&coord| find_adjacents(coord))
                .flatten()
                .collect();
        }

        flashing_coords
            .iter()
            .for_each(|&(x, y)| self.octopi[y][x] = 0);

        // println!("Shoal after step:");
        // self.octopi.iter().for_each(|row| println!("{:?}", row));
        flashing_coords.len()
    }
}

fn find_adjacents(coord: (usize, usize)) -> Vec<(usize, usize)> {
    let right = (coord.0.checked_add(1), Some(coord.1));
    let left = (coord.0.checked_sub(1), Some(coord.1));
    let up = (Some(coord.0), coord.1.checked_sub(1));
    let down = (Some(coord.0), coord.1.checked_add(1));
    let top_right = (coord.0.checked_add(1), coord.1.checked_sub(1));
    let top_left = (coord.0.checked_sub(1), coord.1.checked_sub(1));
    let bottom_left = (coord.0.checked_sub(1), coord.1.checked_add(1));
    let bottom_right = (coord.0.checked_add(1), coord.1.checked_add(1));

    vec![
        right,
        left,
        up,
        down,
        top_right,
        top_left,
        bottom_left,
        bottom_right,
    ]
    .iter()
    .filter_map(|&coord| match coord {
        (Some(x), Some(y)) if x < 10 && y < 10 => Some((x, y)),
        (_, _) => None,
    })
    .collect()
}
