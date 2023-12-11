use std::fmt::Debug;

use itertools::Itertools;
advent_of_code::solution!(11);

#[derive(PartialEq, Eq, Clone)]
enum Observable {
    Galaxy,
    Void(usize),
}

impl Observable {
    fn is_void(&self) -> bool {
        match &self {
            Observable::Void(_) => true,
            _ => false,
        }
    }

    fn set(&mut self, n: usize) {
        match self {
            Observable::Galaxy => panic!(),
            Observable::Void(old_n) => *old_n = n,
        }
    }
}

impl Debug for Observable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Observable::Galaxy => "#",
            Observable::Void(1) => ".",
            Observable::Void(_) => "O",
        })
    }
}

impl From<char> for Observable {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Void(1),
            '#' => Self::Galaxy,
            _ => panic!(),
        }
    }
}

impl Debug for Universe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in &self.univ {
            for o in line {
                o.fmt(f)?
            }
            f.write_str("\n")?
        }
        Ok(())
    }
}

struct Universe {
    univ: Vec<Vec<Observable>>,
}

impl From<&str> for Universe {
    fn from(value: &str) -> Self {
        let universe = Universe {
            univ: value
                .lines()
                .map(|l| l.chars().map(Observable::from).collect())
                .collect(),
        };
        universe
    }
}

impl Universe {
    fn expand(&mut self, expand_size: usize) {
        let lines_to_expand: Vec<usize> = self
            .univ
            .iter()
            .enumerate()
            .filter_map(|(line_index, line)| {
                if line.iter().all(|o| o.is_void()) {
                    Some(line_index)
                } else {
                    None
                }
            })
            .collect();

        let width = self.univ[0].len();
        let columns_to_expand: Vec<usize> = (0..width)
            .filter(|column_index| self.univ.iter().all(|line| line[*column_index].is_void()))
            .collect();

        for col in columns_to_expand {
            self.univ.iter_mut().for_each(|v| v[col].set(expand_size));
        }

        for line in lines_to_expand {
            self.univ[line].iter_mut().for_each(|o| o.set(expand_size));
        }
    }

    fn iter<'a>(&'a self) -> GalaxyIterator<'a> {
        GalaxyIterator {
            universe: self,
            current_pos: (0, 0),
            universe_dims: (self.univ[0].len(), self.univ.len()),
        }
    }

    fn dist(&self, g1: (usize, usize), g2: (usize, usize)) -> usize {
        let mut d = 0;
        for x in g1.0.min(g2.0)..g1.0.max(g2.0) {
            d += match self.univ[g1.1][x] {
                Observable::Galaxy => 1,
                Observable::Void(n) => n,
            }
        }
        for y in g1.1.min(g2.1)..g1.1.max(g2.1) {
            d += match self.univ[y][g1.0] {
                Observable::Galaxy => 1,
                Observable::Void(n) => n,
            }
        }

        d
    }
}

struct GalaxyIterator<'a> {
    universe: &'a Universe,
    current_pos: (usize, usize),
    universe_dims: (usize, usize),
}

impl<'a> Iterator for GalaxyIterator<'a> {
    type Item = (usize, usize);
    fn next(&mut self) -> Option<Self::Item> {
        if self.current_pos.0 + 1 >= self.universe_dims.0 {
            self.current_pos.1 += 1;
            self.current_pos.0 = 0;
        } else {
            self.current_pos.0 += 1;
        }

        // current line
        if self.current_pos.0 != 0 {
            for x in self.current_pos.0..self.universe_dims.0 {
                if self.universe.univ[self.current_pos.1][x] == Observable::Galaxy {
                    self.current_pos.0 = x;
                    return Some(self.current_pos);
                }
            }
            self.current_pos.1 += 1;
        }
        for y in self.current_pos.1..self.universe_dims.1 {
            for x in 0..self.universe_dims.0 {
                if self.universe.univ[y][x] == Observable::Galaxy {
                    self.current_pos = (x, y);
                    return Some(self.current_pos);
                }
            }
        }
        None
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut univ = Universe::from(input);
    univ.expand(2);
    univ.iter()
        .combinations(2)
        .map(|combin| {
            let g1 = combin[0];
            let g2 = combin[1];
            univ.dist(g1, g2)
        })
        .reduce(|acc, el| acc + el)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut univ = Universe::from(input);
    univ.expand(1_000_000);
    univ.iter()
        .combinations(2)
        .map(|combin| {
            let g1 = combin[0];
            let g2 = combin[1];
            univ.dist(g1, g2)
        })
        .reduce(|acc, el| acc + el)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        // let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        let mut univ =
            Universe::from(advent_of_code::template::read_file("examples", DAY).as_str());
        univ.expand(100);
        let result = univ
            .iter()
            .combinations(2)
            .map(|combin| {
                let g1 = combin[0];
                let g2 = combin[1];
                univ.dist(g1, g2)
            })
            .reduce(|acc, el| acc + el);
        assert_eq!(result, Some(8410));
    }
}
