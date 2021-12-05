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
                .filter_map(|coord_string| match !coord_string.is_empty() {
                    true => Some(coord_string.parse().unwrap()),
                    false => None,
                })
                .collect::<Vec<isize>>()
        })
        .map(|coord_vec| {
            Coord::new(
                coord_vec.get(0).unwrap().clone(),
                coord_vec.get(1).unwrap().clone(),
            )
        })
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

    let vent_readings = VentReadings::new(lines);

    println!(
        "Number of points with more than one crossing cardinal line: {}",
        vent_readings.count_overlaps(ReadingMode::Cardinal),
    );

    println!(
        "Number of points with more than one crossing line: {}",
        vent_readings.count_overlaps(ReadingMode::CardinalAndOrdinal)
    );
}

enum ReadingMode {
    Cardinal,
    CardinalAndOrdinal,
}
struct VentReadings {
    lines: Vec<Line>,
}

impl VentReadings {
    pub fn new(lines: Vec<Line>) -> Self {
        Self { lines }
    }

    pub fn count_overlaps(&self, mode: ReadingMode) -> usize {
        let touched_count = self.count_touched_coords(mode);

        touched_count
            .values()
            .filter(|&count| count > &1)
            .collect::<Vec<&isize>>()
            .len()
    }

    fn count_touched_coords(&self, mode: ReadingMode) -> HashMap<Coord, isize> {
        let lines: Vec<Line> = match mode {
            ReadingMode::Cardinal => self
                .lines
                .iter()
                .filter_map(|&line| match line.is_cardinal() {
                    true => Some(line),
                    false => None,
                })
                .collect(),
            ReadingMode::CardinalAndOrdinal => self.lines.clone(),
        };

        let mut touched_count: HashMap<Coord, isize> = HashMap::new();
        lines
            .iter()
            .map(|line| line.touched_coords())
            .flatten()
            .for_each(|coord| touched_count.entry(coord).or_insert(0).add_assign(1));
        touched_count
    }
}

#[derive(Debug, Copy, Clone)]
struct Line {
    start: Coord,
    end: Coord,
}

impl Line {
    pub fn new(start: Coord, end: Coord) -> Self {
        Self { start, end }
    }

    pub fn touched_coords(&self) -> HashSet<Coord> {
        match self.is_cardinal() {
            true => self.cardinal_touched_coords(),
            false => self.ordinal_touched_coors(),
        }
    }

    fn ordinal_touched_coors(&self) -> HashSet<Coord> {
        let mut touched: HashSet<Coord> = HashSet::new();

        let x_direction = match self.start.x > self.end.x {
            true => -1,
            false => 1,
        };

        let y_direction = match self.start.y > self.end.y {
            true => -1,
            false => 1,
        };

        let mut current_x = self.start.x;
        let mut current_y = self.start.y;

        let initial_coord = Coord::new(current_x, current_y);
        touched.insert(initial_coord);

        loop {
            current_x += x_direction;
            current_y += y_direction;
            let touched_coord = Coord::new(current_x, current_y);
            touched.insert(touched_coord);

            if current_x == self.end.x && current_y == self.end.y {
                break;
            }
        }

        touched
    }

    fn cardinal_touched_coords(&self) -> HashSet<Coord> {
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

#[derive(Debug, Clone, Hash, Eq, PartialEq, Copy)]
struct Coord {
    x: isize,
    y: isize,
}

impl Coord {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}
