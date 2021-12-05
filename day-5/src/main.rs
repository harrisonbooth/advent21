use std::{
    collections::{HashMap, HashSet},
    ops::AddAssign,
};

fn main() {
    let file = include_str!("../input.txt");
    let pairs: Vec<&str> = file
        .split("\n")
        .filter(|pair_str| !pair_str.is_empty())
        .map(|pair_str| pair_str.split(" -> ").collect::<Vec<&str>>())
        .flatten()
        .collect();

    let coords: Vec<Coord> = pairs
        .iter()
        .map(|pair| {
            pair.split(",")
                .filter_map(|x| match !x.is_empty() {
                    true => Some(x.parse().unwrap()),
                    false => None,
                })
                .collect::<Vec<isize>>()
        })
        .map(|coord_vec| Coord::from_vec(coord_vec))
        .collect();

    let lines: Vec<Line> = coords
        .chunks(2)
        .map(|coords| {
            Line::new(
                coords.get(0).unwrap().clone(),
                coords.get(1).unwrap().clone(),
            )
        })
        .collect();

    let mut touched_count: HashMap<Coord, isize> = HashMap::new();
    lines
        .iter()
        .filter(|line| line.is_cardinal())
        .map(|line| line.touched_coords())
        .flatten()
        .for_each(|coord| touched_count.entry(coord).or_insert(0).add_assign(1));

    let multi_touched_count = touched_count
        .values()
        .filter(|&count| count > &1)
        .collect::<Vec<&isize>>()
        .len();

    println!("Total number of points touched: {}", touched_count.len());

    println!(
        "Number of points with more than one crossing line: {}",
        multi_touched_count,
    )
}

#[derive(Debug)]
struct Line {
    start: Coord,
    end: Coord,
}

impl Line {
    pub fn new(start: Coord, end: Coord) -> Self {
        Self { start, end }
    }

    pub fn touched_coords(&self) -> HashSet<Coord> {
        let mut touched: HashSet<Coord> = HashSet::new();

        let x_range = match self.start.x > self.end.x {
            true => self.end.x..=self.start.x,
            false => self.start.x..=self.end.x,
        };

        let y_range = match self.start.y > self.end.y {
            true => self.end.y..=self.start.y,
            false => self.start.y..=self.end.y,
        };

        for x in x_range {
            let coord = Coord::new(x, self.start.y);
            touched.insert(coord);
        }

        for y in y_range {
            let coord = Coord::new(self.start.x, y);
            touched.insert(coord);
        }

        touched
    }

    pub fn is_cardinal(&self) -> bool {
        (self.start.x == self.end.x) || (self.start.y == self.end.y)
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Coord {
    x: isize,
    y: isize,
}

impl Coord {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub fn from_vec(vec: Vec<isize>) -> Self {
        Self {
            x: vec.get(0).unwrap().clone(),
            y: vec.get(1).unwrap().clone(),
        }
    }
}
