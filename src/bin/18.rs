use core::panic;

use nom::{
    bytes::complete::tag,
    character::complete::anychar,
    character::complete::{alphanumeric1, digit1},
    combinator::map,
    sequence::{delimited, terminated, tuple},
};

advent_of_code::solution!(18);

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

struct PosIterator {
    next_pos: (usize, usize),
    dir: Direction,
    delta: usize,
}

impl Iterator for PosIterator {
    type Item = (usize, usize);
    fn next(&mut self) -> Option<Self::Item> {
        if self.delta == 0 {
            return None;
        }
        self.next_pos = match self.dir {
            Direction::Up if self.next_pos.1 > 0 => (self.next_pos.0, self.next_pos.1 - 1),
            Direction::Right => (self.next_pos.0 + 1, self.next_pos.1),
            Direction::Down => (self.next_pos.0, self.next_pos.1 + 1),
            Direction::Left if self.next_pos.0 > 0 => (self.next_pos.0 - 1, self.next_pos.1),
            _ => return None,
        };
        self.delta -= 1;
        Some(self.next_pos)
    }
}
impl PosIterator {
    fn new(begin_pos: (usize, usize), dir: Direction, delta: usize) -> Self {
        Self {
            next_pos: begin_pos,
            dir,
            delta,
        }
    }
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            'U' => Self::Up,
            'R' => Self::Right,
            'D' => Self::Down,
            'L' => Self::Left,
            _ => panic!("invalid character for Direction"),
        }
    }
}

struct Instruction {
    dir: Direction,
    length: usize,
    _color: [u8; 3],
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        map(
            tuple((
                terminated(anychar::<&str, ()>, tag(" ")),
                terminated(digit1, tag(" ")),
                delimited(tag("(#"), alphanumeric1, tag(")")),
            )),
            |(dir_char, length_str, color_str)| Instruction {
                dir: Direction::from(dir_char),
                length: usize::from_str_radix(length_str, 10).unwrap(),
                _color: {
                    let mut iter = (0..color_str.len())
                        .into_iter()
                        .step_by(2)
                        .map(|i| u8::from_str_radix(&color_str[i..i + 2], 16).unwrap());
                    [
                        iter.next().unwrap(),
                        iter.next().unwrap(),
                        iter.next().unwrap(),
                    ]
                },
            },
        )(value)
        .unwrap()
        .1
    }
}

struct DigInstructionList {
    instrs: Vec<Instruction>,
}

impl From<&str> for DigInstructionList {
    fn from(value: &str) -> Self {
        Self {
            instrs: value.lines().map(Instruction::from).collect(),
        }
    }
}

impl DigInstructionList {
    fn get_hole_size(&self) -> usize {
        fn _print_hole(map: &Vec<Vec<bool>>) {
            let mut s = String::new();
            for l in map {
                for b in l {
                    s.push(if *b { '#' } else { '.' })
                }
                s.push('\n')
            }
            println!("{s}");
        }

        let mut max_x = 0;
        let mut min_x = 0;
        let mut max_y = 0;
        let mut min_y = 0;
        let mut x = 0;
        let mut y = 0;

        for instr in &self.instrs {
            match instr.dir {
                Direction::Right => {
                    x += instr.length as isize;
                    max_x = max_x.max(x);
                }
                Direction::Left => {
                    x -= instr.length as isize;
                    min_x = min_x.min(x);
                }
                Direction::Down => {
                    y += instr.length as isize;
                    max_y = max_y.max(y);
                }
                Direction::Up => {
                    y -= instr.length as isize;
                    min_y = min_y.min(y);
                }
            }
        }

        let mut init_pos = ((-min_x) as usize, (-min_y) as usize);
        let max_x = (max_x - min_x) as usize;
        let max_y = (max_y - min_y) as usize;

        let mut allocated_map: Vec<Vec<bool>> = vec![vec![false; max_x + 1]; max_y + 1];
        for instr in &self.instrs {
            let new_pos_iter = PosIterator::new(init_pos, instr.dir, instr.length);

            for new_pos in new_pos_iter {
                // dbg!(&(new_pos));
                debug_assert!(!allocated_map[new_pos.1][new_pos.0]);
                allocated_map[new_pos.1][new_pos.0] = true;
                init_pos = new_pos;
            }
        }
        // print_hole(&allocated_map);
        // find point inside for sure
        let first_line = &allocated_map[0];
        let mut first_inside_point = None;
        for x in 0..=max_x {
            if first_line[x] && !allocated_map[1][x] {
                first_inside_point = Some((x, 1));
            }
        }
        let mut insides = vec![first_inside_point.unwrap()];
        while let Some(inside) = insides.pop() {
            allocated_map[inside.1][inside.0] = true;
            for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let new_inside = (
                    (inside.0 as isize + dx) as usize,
                    (inside.1 as isize + dy) as usize,
                );
                if !allocated_map[new_inside.1][new_inside.0] {
                    insides.push(new_inside);
                }
            }
        }
        allocated_map
            .into_iter()
            .map(|l| l.into_iter().filter(|&b| b).count())
            .sum()
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(DigInstructionList::from(input).get_hole_size())
}

pub fn part_two(input: &str) -> Option<usize> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(62));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
        // assert_eq!(result, Some(952408144115));
    }
}
