pub fn main() {
    let file = include_str!("../input.txt");
    let height_map = HeightMap::generate(file.to_string());

    println!(
        "Total of low risks (part 1): {}",
        height_map.total_low_risks()
    );

    println!(
        "Product of three biggest basins (part 2): {}",
        height_map.total_biggest_basins()
    );
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

    pub fn total_biggest_basins(&self) -> usize {
        let low_risks = self.find_low_risk();

        let mut basins: Vec<Vec<(usize, usize)>> = low_risks
            .iter()
            .map(|node| self.map_basin(node.1))
            .collect();

        basins.sort_by(|a, b| b.len().cmp(&a.len()));

        basins
            .iter()
            .take(3)
            .fold(1, |acc, basin| acc * basin.len())
    }

    fn map_basin(&self, origin: (usize, usize)) -> Vec<(usize, usize)> {
        let mut basin_nodes = vec![origin];
        let mut next_to_look_at: Vec<(usize, usize)> = vec![origin];
        // println!("Origin: {:?}", origin);
        loop {
            match next_to_look_at.get(0) {
                Some(basin_node) => {
                    let mut children: Vec<(usize, usize)> = self
                        .find_adjacents(basin_node.clone())
                        .into_iter()
                        .filter(|&coords| {
                            self.heights[coords.1][coords.0] < 9 && !basin_nodes.contains(&coords)
                        })
                        .collect();

                    // println!("Next to look at: {:?}", next_to_look_at);
                    // println!("Children: {:?}", children);
                    basin_nodes.append(&mut children.clone());
                    next_to_look_at.append(&mut children);
                    next_to_look_at.remove(0);
                    // println!(
                    // "Next to look at after appending children & removing index 0: {:?}",
                    // next_to_look_at
                    // );
                }
                None => break,
            }
        }
        // println!("-------------------------------------");

        basin_nodes
    }

    pub fn find_low_risk(&self) -> Vec<(usize, (usize, usize))> {
        let mut low_risk: Vec<(usize, (usize, usize))> = Vec::new();
        for y in 0..self.y_bound {
            for x in 0..self.x_bound {
                let height = self.heights[y][x];
                let adjacents = self.find_adjacents((x, y));
                match adjacents.iter().all(|(x, y)| self.heights[*y][*x] > height) {
                    true => low_risk.push((height, (x, y))),
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

    fn count_risks(nodes: Vec<(usize, (usize, usize))>) -> usize {
        nodes.iter().fold(0, |acc, node| acc + (node.0 + 1))
    }
}
