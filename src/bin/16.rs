advent_of_code::solution!(16);

#[derive(Clone, Copy)]
enum MirrorMazeElement {
    MirrorSlash,
    MirrorAntislash,
    SplitterHorizontal,
    SplitterVertical,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn apply_delta(&self, (x, y): (usize, usize)) -> Option<(usize, usize)> {
        match self {
            Direction::Up if y > 0 => Some((x, y - 1)),
            Direction::Right => Some((x + 1, y)),
            Direction::Down => Some((x, y + 1)),
            Direction::Left if x > 0 => Some((x - 1, y)),
            _ => None,
        }
    }
}

impl TryFrom<char> for MirrorMazeElement {
    type Error = ();
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '-' => Ok(Self::SplitterHorizontal),
            '|' => Ok(Self::SplitterVertical),
            '/' => Ok(Self::MirrorSlash),
            '\\' => Ok(Self::MirrorAntislash),
            _ => Err(()),
        }
    }
}

impl MirrorMazeElement {
    fn get_next_pos(
        &self,
        lx: usize,
        ly: usize,
        ldir: Direction,
        mut cb: impl FnMut(usize, usize, Direction) -> (),
    ) {
        match self {
            MirrorMazeElement::SplitterHorizontal => match ldir {
                Direction::Up | Direction::Down => {
                    for new_dir in [Direction::Right, Direction::Left] {
                        if let Some((new_lx, new_ly)) = new_dir.apply_delta((lx, ly)) {
                            cb(new_lx, new_ly, new_dir);
                        }
                    }
                }
                Direction::Right | Direction::Left => {
                    if let Some((new_lx, new_ly)) = ldir.apply_delta((lx, ly)) {
                        cb(new_lx, new_ly, ldir)
                    }
                }
            },
            MirrorMazeElement::SplitterVertical => match ldir {
                Direction::Right | Direction::Left => {
                    for new_dir in [Direction::Up, Direction::Down] {
                        if let Some((new_lx, new_ly)) = new_dir.apply_delta((lx, ly)) {
                            cb(new_lx, new_ly, new_dir);
                        }
                    }
                }
                Direction::Up | Direction::Down => {
                    if let Some((new_lx, new_ly)) = ldir.apply_delta((lx, ly)) {
                        cb(new_lx, new_ly, ldir)
                    }
                }
            },
            MirrorMazeElement::MirrorSlash => {
                let new_dir = match ldir {
                    Direction::Up => Direction::Right,
                    Direction::Right => Direction::Up,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Down,
                };
                if let Some((new_lx, new_ly)) = new_dir.apply_delta((lx, ly)) {
                    cb(new_lx, new_ly, new_dir);
                }
            }
            MirrorMazeElement::MirrorAntislash => {
                let new_dir = match ldir {
                    Direction::Up => Direction::Left,
                    Direction::Right => Direction::Down,
                    Direction::Down => Direction::Right,
                    Direction::Left => Direction::Up,
                };
                if let Some((new_lx, new_ly)) = new_dir.apply_delta((lx, ly)) {
                    cb(new_lx, new_ly, new_dir);
                }
            }
        }
    }
}

struct Mirror {
    map: Vec<Vec<Option<MirrorMazeElement>>>,
}

impl From<&str> for Mirror {
    fn from(value: &str) -> Self {
        Mirror {
            map: value
                .lines()
                .map(|l| {
                    l.chars()
                        .map(|c| MirrorMazeElement::try_from(c).ok())
                        .collect()
                })
                .collect(),
        }
    }
}

impl Mirror {
    fn in_bounds(&self, x: usize, y: usize) -> bool {
        y < self.map.len() && x < self.map[0].len()
    }

    fn simulater_laser(&self, start_pos: (usize, usize), start_dir: Direction) -> usize {
        let mut energized_map = vec![vec![(false, vec![]); self.map[0].len()]; self.map.len()];
        let mut running_lasers = vec![(start_pos, start_dir)];
        'laser_loop: while let Some(((lx, ly), dir)) = running_lasers.pop() {
            let (currently_energized, energized_dirs) = &mut energized_map[ly][lx];
            for energized_dir in energized_dirs.iter() {
                if *energized_dir == dir {
                    // way already computed, no need to keep going
                    continue 'laser_loop;
                }
            }
            *currently_energized = true;
            energized_dirs.push(dir);

            // TODO - maybe optimise for empty tiles with a loop here
            if let Some(element) = self.map[ly][lx] {
                element.get_next_pos(lx, ly, dir, |new_lx, new_ly, new_dir| {
                    if self.in_bounds(new_lx, new_ly) {
                        running_lasers.push(((new_lx, new_ly), new_dir));
                    }
                })
            } else if let Some((new_lx, new_ly)) = dir.apply_delta((lx, ly)) {
                if self.in_bounds(new_lx, new_ly) {
                    running_lasers.push(((new_lx, new_ly), dir));
                }
            }
        }
        energized_map
            .into_iter()
            .map(|line| line.into_iter().filter(|e| e.0).count())
            .sum()
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mirror = Mirror::from(input);
    Some(mirror.simulater_laser((0, 0), Direction::Right))
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
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
