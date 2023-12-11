use std::{
    collections::VecDeque,
    io::{Error, ErrorKind},
};

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

advent_of_code::solution!(10);

#[derive(Clone, Copy, PartialEq, Eq, EnumIter)]
enum Pipe {
    Start,
    Vertical,
    Horizontal,
    NorthToEast,
    EastToSouth,
    SouthToWest,
    WestToNorth,
}

impl TryFrom<char> for Pipe {
    type Error = Error;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'S' => Ok(Pipe::Start),
            '|' => Ok(Pipe::Vertical),
            '-' => Ok(Pipe::Horizontal),
            'L' => Ok(Pipe::NorthToEast),
            'F' => Ok(Pipe::EastToSouth),
            '7' => Ok(Pipe::SouthToWest),
            'J' => Ok(Pipe::WestToNorth),
            _ => Err(Error::new(ErrorKind::InvalidData, "Invalid char.")),
        }
    }
}

// impl Pipe {
//     fn to_str(&self) -> &'static str {
//         match &self {
//             Pipe::Start => "S",
//             Pipe::Vertical => "|",
//             Pipe::Horizontal => "-",
//             Pipe::NorthToEast => "⎣",
//             Pipe::EastToSouth => "⎡",
//             Pipe::SouthToWest => "⎤",
//             Pipe::WestToNorth => "⎦",
//         }
//     }
// }

struct PipeMaze {
    start_pos: (usize, usize),
    width: usize,
    height: usize,
    map: Vec<Vec<Option<Pipe>>>,
}

impl Pipe {
    fn get_deltas(&self) -> &'static [(isize, isize)] {
        match &self {
            Pipe::Start => &[(-1, 0), (1, 0), (0, -1), (0, 1)],
            Pipe::Vertical => &[(0, 1), (0, -1)],
            Pipe::Horizontal => &[(1, 0), (-1, 0)],
            Pipe::NorthToEast => &[(0, -1), (1, 0)],
            Pipe::EastToSouth => &[(1, 0), (0, 1)],
            Pipe::SouthToWest => &[(0, 1), (-1, 0)],
            Pipe::WestToNorth => &[(-1, 0), (0, -1)],
        }
    }
}

trait ApplyDelta<T>: Sized {
    fn try_delta(self, delta: T) -> Option<Self>;
}

impl ApplyDelta<(isize, isize)> for (usize, usize) {
    fn try_delta(self, delta: (isize, isize)) -> Option<Self> {
        let x = (self.0 as isize) + delta.0;
        let y = (self.1 as isize) + delta.1;
        if x < 0 || y < 0 {
            None
        } else {
            Some((x as usize, y as usize))
        }
    }
}

impl PipeMaze {
    fn new(input: &str) -> Self {
        let mut start_pos = (0, 0);
        let map: Vec<Vec<Option<Pipe>>> = input
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| {
                        if c == 'S' {
                            start_pos = (x, y);
                        };
                        Pipe::try_from(c).ok()
                    })
                    .collect()
            })
            .collect();

        PipeMaze {
            width: map[0].len(),
            height: map.len(),
            start_pos,
            map,
        }
    }

    fn get_pipe(&self, pos: (usize, usize)) -> Option<Pipe> {
        if pos.0 >= self.width || pos.1 >= self.height {
            None
        } else {
            self.map[pos.1][pos.0]
        }
    }

    fn get_empty_map<T>(&self) -> Vec<Vec<Option<T>>>
    where
        T: Sized + Clone,
    {
        let mut empty_map: Vec<Vec<Option<T>>> = Vec::with_capacity(self.height);
        let mut empty_line = Vec::with_capacity(self.width);
        for _ in 0..self.width {
            empty_line.push(None);
        }
        for _ in 0..self.height {
            empty_map.push(empty_line.clone());
        }

        empty_map
    }

    fn get_distance_map(&self) -> Vec<Vec<Option<usize>>> {
        let mut distance_map = self.get_empty_map::<usize>();
        let mut positions = VecDeque::new();

        distance_map[self.start_pos.1][self.start_pos.0] = Some(0);
        for &delta_around_start in self.get_pipe(self.start_pos).unwrap().get_deltas() {
            if let Some(new_pos) = self.start_pos.try_delta(delta_around_start) {
                if let Some(new_pos_pipe) = self.get_pipe(new_pos) {
                    let minus_delta_around_start = (-delta_around_start.0, -delta_around_start.1);
                    if new_pos_pipe
                        .get_deltas()
                        .contains(&minus_delta_around_start)
                    {
                        // this pipe really connects to start.
                        positions.push_back((new_pos, 1));
                    }
                }
            }
        }

        while let Some((pos, dist)) = positions.pop_front() {
            match distance_map.get_mut(pos.1).unwrap().get_mut(pos.0).unwrap() {
                v if v == &mut None => {
                    *v = Some(dist);
                }
                Some(previous_dist) => {
                    if *previous_dist < dist {
                        continue;
                    } else {
                        *previous_dist = dist
                    }
                }
                _ => unreachable!(),
            }
            let deltas = {
                if let Some(pipe) = self.get_pipe(pos) {
                    pipe.get_deltas()
                } else {
                    &[]
                }
            };
            for &delta in deltas {
                if let Some(new_pos) = pos.try_delta(delta) {
                    positions.push_back((new_pos, dist + 1))
                }
            }
        }
        distance_map
    }

    fn deduce_start(&self) -> Pipe {
        let mut valid_deltas = Vec::new();

        for &delta_around_start in self.get_pipe(self.start_pos).unwrap().get_deltas() {
            if let Some(new_pos) = self.start_pos.try_delta(delta_around_start) {
                if let Some(new_pos_pipe) = self.get_pipe(new_pos) {
                    let minus_delta_around_start = (-delta_around_start.0, -delta_around_start.1);
                    if new_pos_pipe
                        .get_deltas()
                        .contains(&minus_delta_around_start)
                    {
                        // this pipe really connects to start.
                        valid_deltas.push(delta_around_start);
                    }
                }
            }
        }
        'enum_loop: for pipe in Pipe::iter() {
            if pipe == Pipe::Start {
                continue;
            }
            for delta in pipe.get_deltas() {
                if !valid_deltas.contains(delta) {
                    continue 'enum_loop;
                }
            }
            return pipe;
        }
        panic!();
    }

    fn furthest(&self) -> Option<usize> {
        self.get_distance_map()
            .into_iter()
            .flatten()
            .filter_map(|x| x)
            .reduce(|vacc, vel| vacc.max(vel))
    }

    fn get_enclosed(&self) -> Option<usize> {
        #[derive(Clone, Copy, PartialEq, Eq)]
        enum Enclosed {
            In,
            Out,
        }
        let loop_map = self.get_distance_map();
        let mut enclosed_map = self.get_empty_map::<Enclosed>();

        for y in 0..self.height {
            for x in 0..self.width {
                // println!("starting_region at {x} {y}");
                if enclosed_map[y][x].is_some() || loop_map[y][x].is_some() {
                    continue;
                }
                let mut maybe_enclosed = true;
                let mut region = self.get_empty_map::<()>();
                let mut around = VecDeque::new();
                around.push_back((x, y));
                while let Some(pos) = around.pop_front() {
                    if pos.0 >= self.width || pos.1 >= self.height {
                        continue;
                    }
                    if loop_map[pos.1][pos.0].is_some() {
                        continue;
                    }
                    if region[pos.1][pos.0].is_some() {
                        continue;
                    }
                    if pos.0 == 0
                        || pos.0 == self.width - 1
                        || pos.1 == 0
                        || pos.1 == self.height - 1
                    {
                        maybe_enclosed = false;
                    }
                    // println!("add to region at {pos:?}");
                    // assert!(region[pos.1][pos.0].is_none());
                    region[pos.1][pos.0] = Some(());

                    for &delta in Pipe::Start.get_deltas() {
                        if let Some(new_pos) = pos.try_delta(delta) {
                            around.push_back(new_pos);
                        }
                    }
                }

                let enclosed_val = if maybe_enclosed {
                    // cast array
                    let (px, py) = region
                        .iter()
                        .enumerate()
                        .find_map(|(py, v)| {
                            if let Some((px, _)) = v.iter().enumerate().find(|(_x, &v)| v.is_some())
                            {
                                Some((px, py))
                            } else {
                                None
                            }
                        })
                        .unwrap();
                    let mut vertical_layers_count = 0;
                    let mut expected_pipe = None;
                    for dx in 0..px {
                        if loop_map[py][dx].is_none() {
                            continue;
                        }
                        if let Some(p) = self.get_pipe((dx, py)) {
                            let p = if p == Pipe::Start {
                                self.deduce_start()
                            } else {
                                p
                            };
                            match p {
                                Pipe::Start => {
                                    unreachable!();
                                }
                                Pipe::Horizontal => {
                                    // let mut dbg_str = String::new();
                                    // for ddx in 0..px {
                                    //     if loop_map[py][ddx].is_some() {
                                    //         dbg_str += self.map[py][ddx].unwrap().to_str();
                                    //     } else {
                                    //         dbg_str += ".";
                                    //     }
                                    // }
                                    // println!("bug at {dx}\n{dbg_str}");
                                    assert!(expected_pipe.is_some());
                                }
                                Pipe::Vertical => vertical_layers_count += 1,
                                Pipe::NorthToEast => expected_pipe = Some(Pipe::SouthToWest),
                                Pipe::EastToSouth => expected_pipe = Some(Pipe::WestToNorth),
                                Pipe::WestToNorth => {
                                    // assert!(expected_pipe.is_some());
                                    if expected_pipe == Some(Pipe::WestToNorth) {
                                        vertical_layers_count += 1;
                                    }
                                    expected_pipe = None;
                                }
                                Pipe::SouthToWest => {
                                    // assert!(expected_pipe.is_some());
                                    if expected_pipe == Some(Pipe::SouthToWest) {
                                        vertical_layers_count += 1;
                                    }
                                    expected_pipe = None;
                                }
                            }
                        }
                    }
                    // dbg!(&vertical_layers_count);
                    let enclosed_for_sure = vertical_layers_count % 2 == 1;
                    if enclosed_for_sure {
                        Enclosed::In
                    } else {
                        Enclosed::Out
                    }
                } else {
                    Enclosed::Out
                };
                region.into_iter().enumerate().for_each(|(cur_y, v)| {
                    v.into_iter()
                        .enumerate()
                        .filter(|(_, v)| v.is_some())
                        .for_each(|(cur_x, _something)| {
                            enclosed_map[cur_y][cur_x] = Some(enclosed_val)
                        })
                });

                // let mut dbg_str = String::new();
                // for y in 0..self.height {
                //     for x in 0..self.width {
                //         if let Some(enclosed) = enclosed_map[y][x] {
                //             dbg_str += match enclosed {
                //                 Enclosed::In => "I",
                //                 Enclosed::Out => "O",
                //             };
                //         } else {
                //             if loop_map[y][x].is_some() {
                //                 dbg_str += self.map[y][x].unwrap().to_str();
                //             } else {
                //                 dbg_str += ".";
                //             }
                //         }
                //     }
                //     dbg_str += "\n";
                // }
                // println!("{dbg_str}");
            }
        }

        Some(
            enclosed_map
                .into_iter()
                .flatten()
                .filter(|&e| e == Some(Enclosed::In))
                .count(),
        )
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let pipe_maze = PipeMaze::new(input);
    pipe_maze.furthest()
}

pub fn part_two(input: &str) -> Option<usize> {
    let pipe_maze = PipeMaze::new(input);
    pipe_maze.get_enclosed()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_other_example() {
        let input = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF
";
        let result = part_one(input);
        assert_eq!(result, Some(4))
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(10));
    }

    #[test]
    fn test_part_two_one() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 21,
        ));
        assert_eq!(result, Some(4));
    }
}
