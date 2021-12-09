pub fn main() {
    let test_file = include_str!("../input.txt");
    let height_map = HeightMap::generate(test_file.to_string());

    println!(
        "Total of low risks (part 1): {}",
        height_map.total_low_risks()
    )
}

struct HeightMap {
    heights: Vec<Vec<usize>>,
    x_bound: usize,
    y_bound: usize,
}

impl HeightMap {
    pub fn generate(file: String) -> Self {
        let heights: Vec<Vec<usize>> = file
            .split("\n")
            .filter_map(|line| match !line.is_empty() {
                true => Some(
                    line.split("")
                        .filter_map(|number| match !number.is_empty() {
                            true => Some(number.parse().unwrap()),
                            false => None,
                        })
                        .collect(),
                ),
                false => None,
            })
            .collect();

        let x_bound = heights[0].len();
        let y_bound = heights.len();

        Self {
            heights,
            x_bound,
            y_bound,
        }
    }

    pub fn total_low_risks(&self) -> usize {
        let risks = self.find_low_risk();
        Self::count_risks(risks)
    }

    pub fn find_low_risk(&self) -> Vec<usize> {
        let mut low_risk: Vec<usize> = Vec::new();
        for y in 0..self.y_bound {
            for x in 0..self.x_bound {
                let height = self.heights[y][x];
                let adjacents = self.find_adjacents((x, y));
                match adjacents.iter().all(|(x, y)| self.heights[*y][*x] > height) {
                    true => low_risk.push(height),
                    false => (),
                }
            }
        }
        low_risk
    }

    fn find_adjacents(&self, coords: (usize, usize)) -> Vec<(usize, usize)> {
        let right = match (coords.0 + 1, coords.1) {
            (x, y) if x < self.x_bound => Some((x, y)),
            (_, _) => None,
        };
        let left = match (coords.0.checked_sub(1), coords.1) {
            (Some(x), y) => Some((x, y)),
            (None, _) => None,
        };
        let up = match (coords.0, coords.1 + 1) {
            (x, y) if y < self.y_bound => Some((x, y)),
            (_, _) => None,
        };
        let down = match (coords.0, coords.1.checked_sub(1)) {
            (x, Some(y)) => (Some((x, y))),
            (_, None) => None,
        };

        let adjacents = vec![right, left, up, down]
            .iter()
            .filter_map(|&coord| coord)
            .collect();

        adjacents
    }

    fn count_risks(heights: Vec<usize>) -> usize {
        heights.iter().fold(0, |acc, height| acc + (height + 1))
    }
}
