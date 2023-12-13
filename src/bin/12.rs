use std::collections::HashMap;

use rayon::iter::ParallelBridge;
use rayon::iter::ParallelIterator;

advent_of_code::solution!(12);

#[derive(Clone, PartialEq, Eq, Debug)]
enum Spring {
    Operational,
    Unknown,
    Damaged,
}

// impl Spring {
//     fn to_str(&self) -> &str {
//         match self {
//             Self::Operational => ".",
//             Self::Damaged => "#",
//             Self::Unknown => "?",
//         }
//     }
// }

// fn log_debug_springs(v: &Vec<Spring>) {
//     v.iter().for_each(|s| print!("{}", s.to_str()));
//     println!("");
// }

impl From<char> for Spring {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Operational,
            '#' => Self::Damaged,
            '?' => Self::Unknown,
            _ => unreachable!(),
        }
    }
}

struct Record {
    springs: Vec<Spring>,
    groups: Vec<usize>,
}

impl From<&str> for Record {
    fn from(value: &str) -> Self {
        let (springs, groups) = value.split_once(' ').unwrap();
        Record {
            springs: springs.chars().map(Spring::from).collect(),
            groups: groups
                .split(',')
                .map(|n| usize::from_str_radix(n, 10).unwrap())
                .collect(),
        }
    }
}

impl Record {
    fn unfold(&mut self) {
        let base_springs = self.springs.clone();
        for _ in 0..4 {
            self.springs.push(Spring::Unknown);
            self.springs.append(&mut base_springs.clone());
        }

        let base_groups = self.groups.clone();
        for _ in 0..4 {
            self.groups.append(&mut base_groups.clone());
        }
    }

    fn _count_arrangements(
        &self,
        mut group_to_match_index: usize,
        mut currently_matching: Option<usize>,
        spring_index: usize,
        cache: &mut HashMap<(usize, Option<usize>, usize), Result<usize, ()>>,
    ) -> Result<usize, ()> {
        if let Some(&entry) = cache.get(&(group_to_match_index, currently_matching, spring_index)) {
            return entry;
        }

        // check if this is worth it to keep parsing
        let total_damaged_to_find = self.groups[group_to_match_index..]
            .to_owned()
            .into_iter()
            .reduce(|acc, el| acc + el)
            .unwrap();
        let potential_damaged_springs_to_parse = self.springs[spring_index..]
            .iter()
            .filter(|&s| *s != Spring::Operational)
            .count();
        let potential_operational_springs_to_parse = self.springs[spring_index..]
            .iter()
            .filter(|&s| *s != Spring::Damaged)
            .count();
        if total_damaged_to_find
            > potential_damaged_springs_to_parse + currently_matching.unwrap_or(0)
        {
            return Err(());
        }
        if self.groups[group_to_match_index..].len() > potential_operational_springs_to_parse + 1 {
            return Err(());
        }

        let mut group_to_match_opt = self.groups.get(group_to_match_index);
        for current_spring_index in spring_index..self.springs.len() {
            let s = &self.springs[current_spring_index];
            match s {
                Spring::Operational => {
                    match (currently_matching, group_to_match_opt) {
                        (None, _) => currently_matching = Some(0),
                        (Some(0), _) => (),
                        (Some(curr), Some(&grp)) if curr == grp => {
                            group_to_match_index += 1;
                            group_to_match_opt = self.groups.get(group_to_match_index);
                            currently_matching = Some(0);
                        }
                        (Some(curr), Some(&grp)) if curr < grp => {
                            return Err(());
                        } // _ => Err(())
                        (Some(_), _) => unreachable!(),
                    }
                }
                Spring::Damaged => {
                    if let Some(curr) = &mut currently_matching {
                        *curr += 1;
                    }
                    match (currently_matching, group_to_match_opt) {
                        (None, _) => return Err(()), // just finished a damaged group, can't start other one
                        (Some(_curr), None) => return Err(()), // nothing to match but we found damaged
                        (Some(curr), Some(&grp)) if curr == grp => {
                            group_to_match_index += 1;
                            group_to_match_opt = self.groups.get(group_to_match_index);
                            currently_matching = None;
                        }

                        (Some(curr), Some(&grp)) => {
                            if curr >= grp {
                                return Err(());
                            }
                        }
                    }
                }
                Spring::Unknown => {
                    match (currently_matching, group_to_match_opt) {
                        (None, None) => (), // only operational after that
                        // (Some(curr), Some(&grp)) if curr > grp => return Err(()),
                        (Some(curr), None) if curr != 0 => return Err(()),
                        (Some(0), None) => (),
                        (Some(0), Some(&grp)) => {
                            if current_spring_index + grp > self.springs.len() {
                                if spring_index == 0 {
                                    dbg!((
                                        current_spring_index,
                                        grp,
                                        &self.springs,
                                        self.springs.len()
                                    ));
                                }
                                return Err(());
                            }
                            let (new_currently_matching, new_spring_index) = if self.springs
                                [current_spring_index..(current_spring_index + grp)]
                                .iter()
                                .all(|s| *s != Spring::Operational)
                            {
                                (Some(grp), current_spring_index + grp)
                            } else {
                                (Some(1), current_spring_index + 1)
                            };
                            // start damaged group
                            let count1 = self._count_arrangements(
                                group_to_match_index,
                                new_currently_matching,
                                new_spring_index,
                                cache,
                            );
                            cache.insert(
                                (
                                    group_to_match_index,
                                    new_currently_matching,
                                    new_spring_index,
                                ),
                                count1,
                            );
                            let count1 = count1.unwrap_or(0);

                            // start new_group later
                            let count2 = self._count_arrangements(
                                group_to_match_index,
                                Some(0),
                                current_spring_index + 1,
                                cache,
                            );
                            cache.insert(
                                (group_to_match_index, Some(0), current_spring_index + 1),
                                count2,
                            );
                            let count2 = count2.unwrap_or(0);

                            return Ok(count1 + count2);
                        }
                        (None, Some(_)) => {
                            currently_matching = Some(0);
                        } // we need at least one operational to split grp
                        (Some(curr), Some(&grp)) if curr < grp => {
                            // must continue damaged group
                            currently_matching = Some(currently_matching.take().unwrap() + 1);
                        }
                        (Some(curr), Some(&grp)) if curr == grp => {
                            // must stop damaged with operational
                            group_to_match_index += 1;
                            group_to_match_opt = self.groups.get(group_to_match_index);
                            currently_matching = Some(0);
                        }
                        _ => unreachable!(),
                    }
                }
            }
        }
        // dealt with all springs, check the ends
        if group_to_match_index < (self.groups.len() - 1) {
            // not even at last group
            return Err(());
        }
        match (currently_matching, group_to_match_opt) {
            (None, None) => Ok(1),    // all good
            (Some(0), None) => Ok(1), // all good
            (Some(_curr), None) => Err(()),
            (Some(curr), Some(&grp)) => {
                if curr == grp {
                    {
                        Ok(1)
                    }
                } else {
                    Err(())
                }
            }
            (None, Some(_grp)) => Err(()),
        }
    }

    fn count_arrangements(&self) -> usize {
        let mut cache = HashMap::new();
        self._count_arrangements(0, Some(0), 0, &mut cache).unwrap()
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(
        input
            .lines()
            .par_bridge()
            .map(Record::from)
            .map(|r| r.count_arrangements())
            .reduce(|| 0, |acc, el| acc + el),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(
        input
            .lines()
            .par_bridge()
            .map(Record::from)
            .map(|mut r| {
                r.unfold();
                r
            })
            .map(|r| r.count_arrangements())
            .reduce(|| 0, |acc, el| acc + el),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(525152));
    }
}
