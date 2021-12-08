fn main() {
    let file = include_str!("../input.txt");
    let mut positions: Vec<isize> = file
        .split(",")
        .filter_map(|pos_str| match !pos_str.is_empty() {
            true => Some(pos_str.parse().unwrap()),
            false => None,
        })
        .collect();

    positions.sort();

    let linear_fuel = CrabCalculator::new(CalculatorMode::LinearCost).calculate_fuel(&positions);
    println!(
        "Fuel used to get to linear best position was: {}",
        linear_fuel
    );

    let gaussian_fuel =
        CrabCalculator::new(CalculatorMode::GaussianCost).calculate_fuel(&positions);
    println!(
        "Fuel used to get to gaussian best position was: {}",
        gaussian_fuel
    );
}

enum CalculatorMode {
    GaussianCost,
    LinearCost,
}

struct CrabCalculator {
    mode: CalculatorMode,
}

impl CrabCalculator {
    pub fn new(mode: CalculatorMode) -> Self {
        Self { mode }
    }

    fn find_best_position(&self, positions: &Vec<isize>) -> isize {
        match self.mode {
            CalculatorMode::GaussianCost => {
                (positions.iter().sum::<isize>() + 1) / (positions.len() as isize)
            }
            CalculatorMode::LinearCost => positions[positions.len() / 2],
        }
    }

    pub fn calculate_fuel(&self, positions: &Vec<isize>) -> isize {
        let best_position = self.find_best_position(&positions);

        match self.mode {
            CalculatorMode::GaussianCost => {
                self.calculcate_fuel_gaussian_rate(&positions, best_position)
            }
            CalculatorMode::LinearCost => {
                self.calculcate_fuel_linear_rate(&positions, best_position)
            }
        }
    }

    fn calculcate_fuel_linear_rate(&self, positions: &Vec<isize>, best_position: isize) -> isize {
        positions
            .iter()
            .map(|position| (position - best_position).abs())
            .sum()
    }

    fn calculcate_fuel_gaussian_rate(&self, positions: &Vec<isize>, best_position: isize) -> isize {
        positions
            .iter()
            .map(|position| {
                let difference = (position - best_position).abs();
                let fuel = (difference as f64 / 2.0f64) * (1.0f64 + difference as f64);
                fuel.round() as isize
            })
            .sum()
    }
}
