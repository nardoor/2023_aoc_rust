use std::collections::VecDeque;

advent_of_code::solution!(17);

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn apply_delta(&self, pos: (usize, usize), n: (usize, usize)) -> Vec<(usize, usize)> {
        (n.0..=n.1)
            .filter_map(|c| match self {
                Direction::Up if pos.1 >= c => Some((pos.0, pos.1 - c)),
                Direction::Right => Some((pos.0 + c, pos.1)),
                Direction::Down => Some((pos.0, pos.1 + c)),
                Direction::Left if pos.0 >= c => Some((pos.0 - c, pos.1)),
                _ => None,
            })
            .collect()
    }
}

#[derive(Debug)]
struct CityBlockMap {
    map: Vec<Vec<u8>>,
}

impl From<&str> for CityBlockMap {
    fn from(value: &str) -> Self {
        CityBlockMap {
            map: value
                .lines()
                .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
                .collect(),
        }
    }
}

impl CityBlockMap {
    fn max_x(&self) -> usize {
        self.map[0].len() - 1
    }
    fn max_y(&self) -> usize {
        self.map.len() - 1
    }
    fn in_bounds(&self, pos: &(usize, usize)) -> bool {
        pos.0 <= self.max_x() && pos.1 <= self.max_y()
    }
    fn shortest_heat_p1(&self) -> usize {
        struct State {
            pos: (usize, usize),
            heat_loss: usize,
            last_dir: Option<Direction>,
        }

        let mut states: VecDeque<State> = VecDeque::new();
        states.push_back(State {
            pos: (0, 0),
            heat_loss: 0,
            last_dir: None,
        });
        let mut best_history_hori: Vec<Vec<usize>> =
            vec![vec![usize::MAX; self.map[0].len()]; self.map.len()];
        let mut best_history_verti: Vec<Vec<usize>> =
            vec![vec![usize::MAX; self.map[0].len()]; self.map.len()];
        while let Some(state) = states.pop_front() {
            let new_dirs = match state.last_dir {
                None => [Direction::Down, Direction::Right],
                Some(Direction::Down | Direction::Up) => [Direction::Left, Direction::Right],
                Some(Direction::Left | Direction::Right) => [Direction::Up, Direction::Down],
            };

            for new_dir in new_dirs {
                let mut extra_heat_loss = 0;
                for new_pos in new_dir.apply_delta(state.pos, (1, 3)) {
                    if !self.in_bounds(&new_pos) {
                        continue;
                    }
                    extra_heat_loss += self.map[new_pos.1][new_pos.0] as usize;
                    let new_heat_loss = extra_heat_loss + state.heat_loss;
                    let best_history = match new_dir {
                        Direction::Up | Direction::Down => &mut best_history_verti,
                        Direction::Right | Direction::Left => &mut best_history_hori,
                    };
                    let old_heat_loss = best_history
                        .get_mut(new_pos.1)
                        .unwrap()
                        .get_mut(new_pos.0)
                        .unwrap();

                    if new_heat_loss < *old_heat_loss {
                        *old_heat_loss = new_heat_loss;
                        states.push_back(State {
                            pos: new_pos,
                            heat_loss: new_heat_loss,
                            last_dir: Some(new_dir),
                        })
                    }
                }
            }
        }
        best_history_hori[self.max_y()][self.max_x()]
            .min(best_history_verti[self.max_y()][self.max_x()])
    }
    fn shortest_heat_p2(&self) -> usize {
        struct State {
            pos: (usize, usize),
            heat_loss: usize,
            last_dir: Option<Direction>,
        }

        let mut states: VecDeque<State> = VecDeque::new();
        states.push_back(State {
            pos: (0, 0),
            heat_loss: 0,
            last_dir: None,
        });
        let mut best_history_hori: Vec<Vec<usize>> =
            vec![vec![usize::MAX; self.map[0].len()]; self.map.len()];
        let mut best_history_verti: Vec<Vec<usize>> =
            vec![vec![usize::MAX; self.map[0].len()]; self.map.len()];
        while let Some(state) = states.pop_front() {
            let new_dirs = match state.last_dir {
                None => [Direction::Down, Direction::Right],
                Some(Direction::Down | Direction::Up) => [Direction::Left, Direction::Right],
                Some(Direction::Left | Direction::Right) => [Direction::Up, Direction::Down],
            };

            for new_dir in new_dirs {
                let new_pos_list = new_dir.apply_delta(state.pos, (1, 10));
                if new_pos_list.len() < 4 {
                    continue;
                }
                let mut extra_heat_loss: usize = new_pos_list
                    .iter()
                    .take(3)
                    .filter_map(|(x, y)| {
                        if self.in_bounds(&(*x, *y)) {
                            Some(self.map[*y][*x] as usize)
                        } else {
                            None
                        }
                    })
                    .sum();
                for new_pos in new_pos_list[3..].into_iter() {
                    if !self.in_bounds(&new_pos) {
                        continue;
                    }
                    extra_heat_loss += self.map[new_pos.1][new_pos.0] as usize;
                    let new_heat_loss = extra_heat_loss + state.heat_loss;
                    let best_history = match new_dir {
                        Direction::Up | Direction::Down => &mut best_history_verti,
                        Direction::Right | Direction::Left => &mut best_history_hori,
                    };
                    let old_heat_loss = best_history
                        .get_mut(new_pos.1)
                        .unwrap()
                        .get_mut(new_pos.0)
                        .unwrap();

                    if new_heat_loss < *old_heat_loss {
                        *old_heat_loss = new_heat_loss;
                        states.push_back(State {
                            pos: *new_pos,
                            heat_loss: new_heat_loss,
                            last_dir: Some(new_dir),
                        })
                    }
                }
            }
        }
        best_history_hori[self.max_y()][self.max_x()]
            .min(best_history_verti[self.max_y()][self.max_x()])
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let city = CityBlockMap::from(input);
    Some(city.shortest_heat_p1())
}

pub fn part_two(input: &str) -> Option<usize> {
    let city = CityBlockMap::from(input);
    Some(city.shortest_heat_p2())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(102));
    }

    #[test]
    fn test_part_two_1() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(94));
    }

    #[test]
    fn test_part_two_2() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(71));
    }
}
