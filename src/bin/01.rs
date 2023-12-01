/// 🕙 23:27:23 ❯ cargo solve 01 --time --release
/// Compiling advent_of_code v0.9.3 (/home/nardor/Documents/projects/2023_aoc_rust)
/// Finished release [optimized] target(s) in 0.89s
///  Running `target/release/01 --time`
/// Part 1: 55971 (60.3µs @ 3669 samples)
/// Part 2: 54719 (494.4µs @ 470 samples)
use pest::Parser;
use pest_derive::Parser;

use rayon::iter::ParallelBridge;
use rayon::iter::ParallelIterator;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    input
        .lines()
        .map(|l| l.chars().filter_map(|c| c.to_digit(10)))
        .map(|mut digits| {
            let first = digits.next().unwrap();
            10 * first + digits.last().unwrap_or(first)
        })
        .reduce(|acc, el| acc + el)
}

#[derive(Parser)]
#[grammar = "src/bin/01.pest"]
pub struct NumberParser;

fn parse_string(input: &str) -> Vec<u32> {
    let Ok(mut parse) = NumberParser::parse(Rule::numbers, input) else {
        panic!();
    };
    parse
        .next()
        .unwrap()
        .into_inner()
        .filter_map(
            |record| match record.into_inner().next().unwrap().as_rule() {
                Rule::one => Some(1),
                Rule::two => Some(2),
                Rule::three => Some(3),
                Rule::four => Some(4),
                Rule::five => Some(5),
                Rule::six => Some(6),
                Rule::seven => Some(7),
                Rule::eight => Some(8),
                Rule::nine => Some(9),
                _ => unreachable!(),
            },
        )
        .collect()
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .par_bridge()
            .map(parse_string)
            .map(|digits| {
                let first = digits.first().unwrap();
                10 * first + digits.last().unwrap_or(first)
            })
            .reduce(|| 0, |acc, el| acc + el),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(281));
    }
}
