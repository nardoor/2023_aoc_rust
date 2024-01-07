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

impl Instruction {
    fn get_new_pos(&self, current_pos: &(isize, isize)) -> (isize, isize) {
        let length = self.length as isize;
        match self.dir {
            Direction::Up => (current_pos.0, current_pos.1 - length),
            Direction::Right => (current_pos.0 + length, current_pos.1),
            Direction::Down => (current_pos.0, current_pos.1 + length),
            Direction::Left => (current_pos.0 - length, current_pos.1),
        }
    }
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
    fn to_part_two_instructions(self) -> Self {
        DigInstructionList {
            instrs: self
                .instrs
                .into_iter()
                .map(|ins| Instruction {
                    dir: [
                        Direction::Right,
                        Direction::Down,
                        Direction::Left,
                        Direction::Up,
                    ][(ins._color[2] & 0x0f) as usize],
                    length: ((ins._color[0] as usize) << 12)
                        + ((ins._color[1] as usize) << 4)
                        + ((ins._color[2] as usize & 0xf0) >> 4) as usize,
                    _color: [0; 3],
                })
                .collect(),
        }
    }
    fn get_hole_size(&self) -> usize {
        fn _shoelace_formula(points: &Vec<(isize, isize)>) -> usize {
            assert_eq!(points.first(), points.last());
            let mut sum = 0;
            for i in 0..(points.len() - 1) {
                let p1 = points[i];
                let p2 = points[i + 1];
                sum += (p1.1 + p2.1) * (p1.0 - p2.0)
            }
            assert!(sum >= 0);
            return (sum / 2) as usize;
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
        let mut points = Vec::new();
        let init_pos = (-min_x, -min_y);
        points.push(init_pos);
        let mut current_pos = init_pos;

        for instr in &self.instrs {
            current_pos = instr.get_new_pos(&current_pos);
            points.push(current_pos);
        }
        let mut area = _shoelace_formula(&points);
        // add missing edge:
        self.instrs.iter().for_each(|ins| match ins.dir {
            Direction::Right | Direction::Up => area += ins.length,
            _ => (),
        });
        // I miss 1, who knows
        area + 1
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(DigInstructionList::from(input).get_hole_size())
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(
        DigInstructionList::from(input)
            .to_part_two_instructions()
            .get_hole_size(),
    )
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
        assert_eq!(result, Some(952408144115));
    }
}
