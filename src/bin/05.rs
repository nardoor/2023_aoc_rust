use std::cmp::Ordering;

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, digit1, line_ending},
    combinator::{map, opt},
    multi::{many1, separated_list1},
    sequence::{delimited, pair, terminated, tuple},
    IResult,
};

advent_of_code::solution!(5);

#[derive(Debug)]
struct Almanac<'a> {
    seeds: Vec<u64>,
    maps: Vec<AlmanacMap<'a>>,
}

impl<'a> From<&'a str> for Almanac<'a> {
    fn from(input: &'a str) -> Self {
        fn parse_seeds(input: &str) -> IResult<&str, Vec<u64>> {
            delimited(
                tag("seeds: "),
                separated_list1(
                    tag(" "),
                    map(digit1, |d| u64::from_str_radix(d, 10).unwrap()),
                ),
                line_ending,
            )(input)
        }

        fn parse_map_info(input: &str) -> IResult<&str, (&str, &str)> {
            map(
                tuple((alpha1, tag("-to-"), alpha1, tag(" map:"), line_ending)),
                |(source, _, destination, _, _)| (source, destination),
            )(input)
        }

        fn parse_range(input: &str) -> IResult<&str, MapRange> {
            map(
                tuple((digit1, tag(" "), digit1, tag(" "), digit1, line_ending)),
                |(destination, _, source, _, lenght, _)| MapRange {
                    source_start: u64::from_str_radix(source, 10).unwrap(),
                    dest_start: u64::from_str_radix(destination, 10).unwrap(),
                    length: u64::from_str_radix(lenght, 10).unwrap(),
                },
            )(input)
        }
        fn parse_map(input: &str) -> IResult<&str, AlmanacMap> {
            map(
                terminated(pair(parse_map_info, many1(parse_range)), opt(line_ending)),
                |((source, destination), ranges)| AlmanacMap {
                    source: source,
                    destination: destination,
                    map_ranges: ranges,
                },
            )(input)
        }

        fn parse_almanach(input: &str) -> IResult<&str, Almanac> {
            map(
                tuple((parse_seeds, line_ending, many1(parse_map))),
                |(seeds, _, maps)| Almanac { seeds, maps },
            )(input)
        }
        let (_left, alm) = parse_almanach(input).unwrap();
        alm
    }
}

struct AlmanacPart2<'a> {
    seed_ranges: Vec<(u64, u64)>,
    maps: Vec<AlmanacMap<'a>>,
}

impl<'a> From<Almanac<'a>> for AlmanacPart2<'a> {
    fn from(value: Almanac<'a>) -> Self {
        let mut seed_ranges = Vec::new();
        for i in 0..value.seeds.len() / 2 {
            seed_ranges.push((
                value.seeds[2 * i],
                value.seeds[2 * i] + value.seeds[2 * i + 1],
            ));
        }
        AlmanacPart2 {
            seed_ranges,
            maps: value.maps,
        }
    }
}
#[derive(Debug)]
struct MapRange {
    source_start: u64,
    dest_start: u64,
    length: u64,
}

impl MapRange {
    fn try_map(&self, input: u64) -> Option<u64> {
        if input >= self.source_start && input < self.source_start + self.length {
            let s = self.source_start as i64;
            let d = self.dest_start as i64;
            let delta: i64 = d - s;
            Some((input as i64 + delta) as u64)
        } else {
            None
        }
    }
    fn apply_delta_range(&self, range: (u64, u64)) -> (u64, u64) {
        let delta: i64 = self.dest_start as i64 - self.source_start as i64;
        (
            (range.0 as i64 + delta) as u64,
            (range.1 as i64 + delta) as u64,
        )
    }
    fn try_map_range(&self, range: (u64, u64)) -> Option<((u64, u64), [Option<(u64, u64)>; 2])> {
        match (
            range.0.cmp(&self.source_start),
            range.1.cmp(&(self.source_start + self.length)),
        ) {
            (Ordering::Equal | Ordering::Greater, Ordering::Less | Ordering::Equal) => {
                // range included in this MapRange
                return Some((self.apply_delta_range((range.0, range.1)), [None, None]));
            }
            (Ordering::Equal | Ordering::Greater, Ordering::Greater) => {
                // range is on right part of MapRange (can have superposition)
                if range.0 >= self.source_start + self.length {
                    // totally out
                    return None;
                } else {
                    // range.0 < self.source_start + self.length => superposition
                    return Some((
                        self.apply_delta_range((range.0, self.source_start + self.length - 1)),
                        [None, Some((self.source_start + self.length, range.1))],
                    ));
                }
            }
            (Ordering::Less, Ordering::Equal | Ordering::Less) => {
                // range is on left part of MapRange (can have superposition)
                if range.1 < self.source_start {
                    // totally out
                    return None;
                } else {
                    // range.1 >= self.source_start => superposition
                    return Some((
                        self.apply_delta_range((self.source_start, range.1)),
                        [Some((range.0, self.source_start - 1)), None],
                    ));
                }
            }
            (Ordering::Less, Ordering::Greater) => {
                // range is overset of MapRange
                return Some((
                    self.apply_delta_range((
                        self.source_start,
                        self.source_start + self.length - 1,
                    )),
                    [
                        Some((range.0, self.source_start - 1)),
                        Some((self.source_start + self.length, range.1)),
                    ],
                ));
            }
        }
    }
}

#[derive(Debug)]
struct AlmanacMap<'a> {
    source: &'a str,
    destination: &'a str,
    map_ranges: Vec<MapRange>,
}

impl<'a> AlmanacMap<'a> {
    fn map(&self, input: u64) -> u64 {
        self.map_ranges
            .iter()
            .find_map(|range| range.try_map(input))
            .unwrap_or(input)
    }

    fn map_range(&self, input_range: (u64, u64)) -> Vec<(u64, u64)> {
        let mut mapped_vec: Vec<(u64, u64)> = Vec::new();
        let mut to_map = vec![input_range];

        for map_range in &self.map_ranges {
            let mut new_to_map = Vec::new();
            for range in to_map {
                if let Some((mapped, not_mapped_array)) = map_range.try_map_range(range) {
                    mapped_vec.push(mapped);
                    for not_mapped in not_mapped_array {
                        if let Some(not_mapped_range) = not_mapped {
                            new_to_map.push(not_mapped_range);
                        }
                    }
                } else {
                    new_to_map.push(range);
                }
            }
            to_map = new_to_map;
        }

        mapped_vec.append(&mut to_map);
        mapped_vec
    }
}
pub fn part_one(input: &str) -> Option<u64> {
    let almanac = Almanac::from(input);

    let mut state = ("seed", almanac.seeds.clone());

    while state.0 != "location" {
        let Some(valid_map) = almanac.maps.iter().find(|&m| m.source == state.0) else {
            panic!();
        };
        state = (
            valid_map.destination,
            state.1.into_iter().map(|inp| valid_map.map(inp)).collect(),
        );
    }

    state.1.into_iter().min()
}
fn merge_ranges(r1: (u64, u64), r2: (u64, u64)) -> Option<(u64, u64)> {
    assert!(r1.1 >= r1.0);
    assert!(r2.1 >= r2.0);
    if r2.1 < r1.0 || r1.1 < r2.0 {
        None
    } else {
        Some((r1.0.min(r2.0), r1.1.max(r2.1)))
    }
}

fn simplify_ranges(mut ranges_vec: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    let mut new_vec = Vec::new();

    new_vec.push(ranges_vec.pop().unwrap());

    for range1 in ranges_vec {
        let mut range1_added = false;
        for range2 in new_vec.iter_mut() {
            if let Some(merged) = merge_ranges(range1, *range2) {
                *range2 = merged;
                range1_added = true;
                break;
            }
        }
        if !range1_added {
            new_vec.push(range1);
        }
    }

    new_vec
}

pub fn part_two(input: &str) -> Option<u64> {
    let almanac = AlmanacPart2::from(Almanac::from(input));

    let mut state = ("seed", almanac.seed_ranges.clone());

    while state.0 != "location" {
        let Some(valid_map) = almanac.maps.iter().find(|&m| m.source == state.0) else {
            panic!();
        };

        state = (
            valid_map.destination,
            simplify_ranges(
                state
                    .1
                    .into_iter()
                    .flat_map(|input_range| valid_map.map_range(input_range))
                    .collect(),
            ),
        );
    }
    Some(
        state
            .1
            .into_iter()
            .reduce(|prec, el| if prec.0 <= el.0 { prec } else { el })
            .unwrap()
            .0,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
