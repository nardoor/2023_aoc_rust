use std::{
    collections::{
        hash_map::Entry::{Occupied, Vacant},
        HashMap, VecDeque,
    },
    io::{Error, ErrorKind},
};

advent_of_code::solution!(10);

#[derive(Clone, Copy)]
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

    fn furthest(&self) -> Option<usize> {
        let mut treated: HashMap<(usize, usize), usize> = HashMap::new();
        let mut positions = VecDeque::new();

        treated.insert(self.start_pos, 0);
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
            match treated.entry(pos) {
                Vacant(e) => {
                    e.insert(dist);
                }
                Occupied(e) => {
                    let previous_dist = e.into_mut();
                    if *previous_dist < dist {
                        continue;
                    } else {
                        *previous_dist = dist;
                    }
                }
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

        // let mut dbg_str = String::new();
        // for y in 0..self.width {
        //     for x in 0..self.height {
        //         if (x, y) == self.start_pos {
        //             dbg_str += "S";
        //         } else if let Some(d) = treated.get(&(x, y)) {
        //             let sd = d.to_string();
        //             dbg_str += &sd.as_str()[sd.len() - 1..];
        //         } else {
        //             dbg_str += ".";
        //         }
        //     }
        //     dbg_str += "\n";
        // }
        // println!("{dbg_str}");

        treated.values().reduce(|vacc, vel| vacc.max(vel)).cloned()
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let pipe_maze = PipeMaze::new(input);
    pipe_maze.furthest()
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
