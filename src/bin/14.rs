use std::{
    collections::HashMap,
    fmt::{Debug, Write},
    hash::Hash,
};

advent_of_code::solution!(14);

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Element {
    SquareRock,
    Rock,
}

impl TryFrom<char> for Element {
    type Error = ();
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '#' => Ok(Self::SquareRock),
            'O' => Ok(Self::Rock),
            _ => Err(()),
        }
    }
}

struct Board {
    board: Vec<Vec<Option<Element>>>,
}

impl From<&str> for Board {
    fn from(value: &str) -> Self {
        Board {
            board: value
                .lines()
                .map(|line| line.chars().map(|c| Element::try_from(c).ok()).collect())
                .collect(),
        }
    }
}

impl Board {
    fn north_load(&self) -> usize {
        let line_count = self.board.len();
        let mut load = 0;
        for col in 0..self.board[0].len() {
            for line in 0..line_count {
                match self.board[line][col] {
                    Some(Element::Rock) => {
                        load += line_count - line;
                    }
                    _ => (),
                }
            }
        }
        load
        // let column_count = self.board[0].len();
        // let line_count = self.board.len();

        // let mut last_free_space;
        // let mut load = 0;

        // for col in 0..column_count {
        //     last_free_space = 0;
        //     for line in 0..line_count {
        //         match self.board[line][col] {
        //             None => (),
        //             Some(Element::Rock) => {
        //                 load += line_count - last_free_space;
        //                 last_free_space += 1;
        //             }
        //             Some(Element::SquareRock) => {
        //                 last_free_space = line + 1;
        //             }
        //         }
        //     }
        // }
        // load
    }

    fn north_tilt(&mut self) {
        let column_count = self.board[0].len();
        let line_count = self.board.len();

        let mut last_free_line;

        for col in 0..column_count {
            last_free_line = 0;
            for line in 0..line_count {
                match self.board[line][col] {
                    None => (),
                    Some(Element::Rock) => {
                        if line != last_free_line {
                            self.board[line][col] = None;
                            self.board[last_free_line][col] = Some(Element::Rock);
                        }
                        last_free_line += 1;
                    }
                    Some(Element::SquareRock) => {
                        last_free_line = line + 1;
                    }
                }
            }
        }
    }
    fn west_tilt(&mut self) {
        let column_count = self.board[0].len();
        let line_count = self.board.len();

        let mut last_free_col;

        for line in 0..line_count {
            last_free_col = 0;
            for col in 0..column_count {
                match self.board[line][col] {
                    None => (),
                    Some(Element::Rock) => {
                        if col != last_free_col {
                            self.board[line][col] = None;
                            self.board[line][last_free_col] = Some(Element::Rock);
                        }
                        last_free_col += 1;
                    }
                    Some(Element::SquareRock) => {
                        last_free_col = col + 1;
                    }
                }
            }
        }
    }
    fn south_tilt(&mut self) {
        let column_count = self.board[0].len();
        let line_count = self.board.len();

        let mut last_free_line;

        for col in 0..column_count {
            last_free_line = line_count - 1;
            for line in (0..line_count).rev() {
                match self.board[line][col] {
                    None => (),
                    Some(Element::Rock) => {
                        if line != last_free_line {
                            self.board[line][col] = None;
                            self.board[last_free_line][col] = Some(Element::Rock);
                        }
                        last_free_line = last_free_line.saturating_sub(1);
                    }
                    Some(Element::SquareRock) => {
                        last_free_line = line.saturating_sub(1);
                    }
                }
            }
        }
    }
    fn east_tilt(&mut self) {
        let column_count = self.board[0].len();
        let line_count = self.board.len();

        let mut last_free_col;

        for line in 0..line_count {
            last_free_col = column_count - 1;
            for col in (0..column_count).rev() {
                match self.board[line][col] {
                    None => (),
                    Some(Element::Rock) => {
                        if col != last_free_col {
                            self.board[line][col] = None;
                            self.board[line][last_free_col] = Some(Element::Rock);
                        }
                        last_free_col = last_free_col.saturating_sub(1);
                    }
                    Some(Element::SquareRock) => {
                        last_free_col = col.saturating_sub(1);
                    }
                }
            }
        }
    }

    fn cycle(&mut self, n: usize) {
        let mut cache = HashMap::new();
        let mut cycle_realised = 0;
        cache.insert(self.board.clone(), cycle_realised);
        // println!("Start: \n{:?}", &self);
        while cycle_realised < n {
            self.north_tilt();
            // println!("After north tilt: \n{:?}", &self);
            self.west_tilt();
            // println!("After west tilt: \n{:?}", &self);
            self.south_tilt();
            // println!("After south tilt: \n{:?}", &self);
            self.east_tilt();
            // println!("After east tilt: \n{:?}", &self);
            cycle_realised += 1;
            if let Some(old_board_cycle_count) = cache.get(&self.board) {
                let period = cycle_realised - old_board_cycle_count;
                if period < n - cycle_realised {
                    let nearest = n - ((n - old_board_cycle_count) % period);
                    // dbg!(cycle_realised, old_board_cycle_count, period, nearest);
                    cycle_realised = nearest;
                }
            } else {
                cache.insert(self.board.clone(), cycle_realised);
            }
        }
        assert_eq!(cycle_realised, n);
    }
}

impl Debug for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.board.iter().for_each(|l| {
            l.iter().for_each(|&e| {
                match e {
                    None => f.write_char('.'),
                    Some(Element::Rock) => f.write_char('O'),
                    Some(Element::SquareRock) => f.write_char('#'),
                }
                .unwrap();
            });
            f.write_char('\n').unwrap();
        });
        Ok(())
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut board = Board::from(input);
    board.north_tilt();
    Some(board.north_load())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut board = Board::from(input);
    board.cycle(1_000_000_000);
    Some(board.north_load())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}
