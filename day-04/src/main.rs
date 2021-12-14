use core::fmt;

use colour::*;

fn main() {
    let file = include_str!("../input.txt");
    let board_size: usize = 5;
    let callings_and_boards = file.split_once("\n").unwrap();
    let called_numbers: Vec<usize> = callings_and_boards
        .0
        .split(",")
        .map(|number| number.parse::<usize>().unwrap())
        .collect();

    let board_strs: Vec<&str> = callings_and_boards
        .1
        .split("\n")
        .map(|row| {
            row.split(" ")
                .filter_map(|number| match !number.is_empty() {
                    true => Some(number),
                    false => None,
                })
                .collect::<Vec<&str>>()
        })
        .flatten()
        .collect::<Vec<&str>>();

    let mut board_set: BoardSet = BoardSet::from_strs(board_strs, &board_size);

    board_set.first_to_win(&called_numbers);
    board_set.last_to_win(&called_numbers);
}

struct BoardSet {
    boards: Vec<Board>,
    board_size: usize,
}

impl BoardSet {
    pub fn from_strs(strs: Vec<&str>, board_size: &usize) -> Self {
        let board_size = board_size.clone();
        Self {
            board_size: board_size,
            boards: strs
                .chunks((board_size).pow(2))
                .map(|board_str| Board::new(board_str, board_size))
                .collect(),
        }
    }

    pub fn last_to_win(&mut self, called_numbers: &Vec<usize>) {
        let mut winning_boards: Vec<Board> = Vec::new();
        let mut last_winning_number: Option<usize> = None;

        for called_number in called_numbers {
            for board in self.boards.iter_mut() {
                if winning_boards.contains(board) {
                    continue;
                }
                board.check_rows(&called_number);
                board.check_win(self.board_size);
                if board.won {
                    winning_boards.push(board.clone());
                    last_winning_number = Some(called_number.clone());
                }
            }
        }

        let last_winner = winning_boards.last().unwrap();
        println!(
            "Board\n{}won last with multiplied score of {}",
            last_winner.clone(),
            last_winner.sum_unmarked_numbers() * last_winning_number.unwrap(),
        );
    }

    pub fn first_to_win(&mut self, called_numbers: &Vec<usize>) {
        let mut winning_board: Option<Board> = None;
        for called_number in called_numbers {
            self.boards.iter_mut().for_each(|board| {
                board.check_rows(&called_number);
                board.check_win(self.board_size);
                if board.won {
                    winning_board = Some(board.clone())
                }
            });
            if winning_board.is_some() {
                println!(
                    "Board\n{}won first with multiplied score of {}",
                    winning_board.clone().unwrap(),
                    winning_board.unwrap().sum_unmarked_numbers() * called_number,
                );
                break;
            }
        }
    }
}

#[derive(Clone, PartialEq)]
struct Board {
    rows: Vec<Vec<BoardNumber>>,
    won: bool,
}

impl fmt::Display for Board {
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
        Ok(self.rows.iter().for_each(|row| {
            row.iter().for_each(|number| print!("{} ", number));
            println!("");
        }))
    }
}

impl Board {
    pub fn new(board_str: &[&str], board_size: usize) -> Self {
        Self {
            rows: board_str
                .chunks(board_size)
                .map(|board_row| {
                    board_row
                        .iter()
                        .map(|number_str| BoardNumber::from_str(number_str))
                        .collect()
                })
                .collect(),
            won: false,
        }
    }

    pub fn check_rows(&mut self, called_number: &usize) {
        self.rows.iter_mut().for_each(|row| {
            row.iter_mut()
                .for_each(|board_number| board_number.verify(*called_number))
        });
    }

    pub fn check_win(&mut self, board_size: usize) {
        let row_win = self
            .rows
            .iter()
            .any(|row| row.iter().all(|board_number| board_number.called));
        if row_win {
            self.won = true;
        }

        for column_number in 0..board_size - 1 {
            let column_win = self
                .rows
                .iter()
                .map(|row| row.get(column_number).unwrap())
                .all(|board_number| board_number.called);

            if column_win {
                self.won = true;
            }
        }
    }

    pub fn sum_unmarked_numbers(&self) -> usize {
        self.rows
            .iter()
            .flatten()
            .filter_map(|number| match !number.called {
                true => Some(number.value),
                false => None,
            })
            .sum()
    }
}

#[derive(Clone, PartialEq)]
struct BoardNumber {
    value: usize,
    called: bool,
}

impl fmt::Display for BoardNumber {
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.called {
            true => Ok(red!("{}", self.value)),
            false => Ok(green!("{}", self.value)),
        }
    }
}

impl BoardNumber {
    pub fn from_str(str: &str) -> Self {
        Self {
            value: str.parse().unwrap(),
            called: false,
        }
    }

    pub fn verify(&mut self, called_number: usize) {
        if self.value == called_number {
            self.called = true;
        }
    }
}
