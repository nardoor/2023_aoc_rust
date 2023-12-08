use std::collections::HashMap;
use std::collections::VecDeque;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until1;
use nom::character::complete::{char, line_ending};
use nom::combinator::{map, opt};
use nom::multi::many1;
use nom::sequence::{delimited, separated_pair, terminated};
use nom::IResult;
use num::Integer;
use rayon::iter::ParallelBridge;
use rayon::iter::ParallelIterator;
advent_of_code::solution!(8);

enum Instr {
    Left,
    Right,
}

impl From<char> for Instr {
    fn from(value: char) -> Self {
        match value {
            'R' => Self::Right,
            'L' => Self::Left,
            _ => unreachable!(),
        }
    }
}

impl Instr {
    fn apply<'a>(&self, doc: &'a Document) -> &'a str {
        match self {
            Self::Left => doc.left,
            Self::Right => doc.right,
        }
    }
}

struct Document<'a> {
    name: &'a str,
    left: &'a str,
    right: &'a str,
}

struct InputParser;

impl InputParser {
    fn parse<'a>(input: &'a str) -> (VecDeque<Instr>, Vec<Document<'a>>) {
        let res: IResult<&str, Vec<Instr>> = terminated(
            many1(map(alt((char('L'), char('R'))), |a| Instr::from(a))),
            many1(line_ending),
        )(input);

        let Ok((left, instr)) = res else {
            panic!();
        };

        let res: IResult<&str, Vec<Document>> = many1(map(
            terminated(
                separated_pair(
                    take_until1(" "),
                    tag(" = "),
                    delimited(
                        tag("("),
                        separated_pair(take_until1(","), tag(", "), take_until1(")")),
                        tag(")"),
                    ),
                ),
                opt(line_ending),
            ),
            |(name, (left, right))| Document { name, right, left },
        ))(left);

        let Ok((left, docs)) = res else {
            panic!();
        };

        assert!(left.len() == 0);

        (VecDeque::from(instr), docs)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (instr, docs) = InputParser::parse(input);
    let mut doc_maps = HashMap::new();
    docs.iter().for_each(|d| {
        doc_maps.insert(d.name, d);
        ()
    });

    // let instr_count = instr.len();
    let mut cur_state_doc = doc_maps.get("AAA").unwrap();
    let mut result = 0;
    instr.iter().cycle().find(|&cur_instr| {
        cur_state_doc = doc_maps.get(cur_instr.apply(cur_state_doc)).unwrap();
        result += 1;

        cur_state_doc.name == "ZZZ"
    });
    Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    let (instr, docs) = InputParser::parse(input);
    let mut doc_maps = HashMap::new();
    docs.iter().for_each(|d| {
        doc_maps.insert(d.name, d);
        ()
    });

    let instr_len = instr.len();

    // get periods
    let periods: Vec<usize> = docs
        .iter()
        .par_bridge()
        .filter(|&d| d.name.ends_with('A'))
        .map(|s| {
            let mut s = s;
            let mut move_count = 0;
            while !s.name.ends_with('Z') || move_count % instr_len != 0 {
                s = doc_maps
                    .get(instr[move_count % instr_len].apply(s))
                    .unwrap();
                move_count += 1;
            }
            move_count / instr_len
        })
        .collect();
    let lcm = periods
        .into_iter()
        .par_bridge()
        .reduce(|| 1, |acc, el| acc.lcm(&el));

    Some(lcm * instr_len)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(6));
    }
}
