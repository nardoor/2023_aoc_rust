advent_of_code::solution!(16);

#[derive(Clone, Copy)]
enum MirrorMazeElement {
    MirrorSlash,
    MirrorAntislash,
    SplitterHorizontal,
    SplitterVertical,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
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

type EnergizedMap = Vec<Vec<(bool, Option<Vec<Direction>>)>>;
impl Mirror {
    fn max_x(&self) -> usize {
        self.map[0].len() - 1
    }
    fn max_y(&self) -> usize {
        self.map.len() - 1
    }
    fn in_bounds(&self, x: usize, y: usize) -> bool {
        y <= self.max_y() && x <= self.max_x()
    }
    fn simulate_laser(&self, start_pos: (usize, usize), start_dir: Direction) -> EnergizedMap {
        let mut energized_map: EnergizedMap =
            vec![vec![(false, None); self.map[0].len()]; self.map.len()];
        let mut running_lasers = vec![(start_pos, start_dir)];
        'laser_loop: while let Some(((lx, ly), dir)) = running_lasers.pop() {
            let (currently_energized, energized_dirs_opt) = &mut energized_map[ly][lx];
            if let Some(energized_dirs) = energized_dirs_opt {
                for energized_dir in energized_dirs.iter() {
                    if *energized_dir == dir {
                        // way already computed, no need to keep going
                        continue 'laser_loop;
                    }
                }
            } else {
                *energized_dirs_opt = Some(Vec::new());
            }
            *currently_energized = true;
            energized_dirs_opt.as_mut().unwrap().push(dir);

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
    }
    fn simulater_laser_count(&self, start_pos: (usize, usize), start_dir: Direction) -> usize {
        self.simulate_laser(start_pos, start_dir)
            .into_iter()
            .map(|line| line.into_iter().filter(|e| e.0).count())
            .sum()
    }

    fn simulater_laser_maximise(&self) -> usize {
        fn merge_history(history: &mut EnergizedMap, to_merge: EnergizedMap) {
            to_merge.into_iter().enumerate().for_each(|(y, line)| {
                line.into_iter()
                    .enumerate()
                    .for_each(|(x, (energized, dir_vec_opt))| {
                        if energized {
                            let (henergized, hdir_vec_opt) = &mut history[y][x];
                            if *henergized {
                                for dir in dir_vec_opt.unwrap() {
                                    if !hdir_vec_opt.as_mut().unwrap().contains(&dir) {
                                        hdir_vec_opt.as_mut().unwrap().push(dir);
                                    }
                                }
                                // merge vec
                            } else {
                                *henergized = true;
                                *hdir_vec_opt = dir_vec_opt;
                            }
                        }
                    })
            });
        }
        let mut history: Option<EnergizedMap> = None;
        let mut max_count = 0;
        let mut around_data: [(Direction, (usize, usize), (usize, usize)); 4] = [
            (Direction::Down, (0, self.max_x()), (0, 0)),
            (
                Direction::Left,
                (self.max_x(), self.max_x()),
                (0, self.max_y()),
            ),
            (
                Direction::Up,
                (0, self.max_x()),
                (self.max_y(), self.max_y()),
            ),
            (Direction::Right, (0, 0), (0, self.max_y())),
        ];
        for (sdir, rx, ry) in &mut around_data {
            for sy in ry.0..=ry.1 {
                'start_loop: for sx in rx.0..=rx.1 {
                    if let Some(old_history) = history.as_ref() {
                        let (energized, dirs) = &old_history[sy][sx];
                        if *energized {
                            for hdir in dirs.as_ref().unwrap() {
                                // optimistic hypothesis that splitters
                                // don't incapacitate this optimisation
                                //
                                // however it could, and if it happens
                                // we can go back to some check like this :
                                /*
                                if *sdir == *hdir
                                {
                                    continue 'start_loop,
                                }
                                */
                                match (*sdir, *hdir) {
                                    (
                                        Direction::Down | Direction::Up,
                                        Direction::Down | Direction::Up,
                                    ) => continue 'start_loop,
                                    (
                                        Direction::Right | Direction::Left,
                                        Direction::Right | Direction::Left,
                                    ) => continue 'start_loop,
                                    _ => (),
                                }
                            }
                        }
                    }

                    let new_history = self.simulate_laser((sx, sy), *sdir);
                    let count = new_history
                        .iter()
                        .map(|line| line.iter().filter(|e| e.0).count())
                        .sum();
                    max_count = max_count.max(count);
                    if history.is_none() {
                        history = Some(new_history);
                    } else {
                        merge_history(&mut history.as_mut().unwrap(), new_history);
                    }
                }
            }
        }
        max_count
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mirror = Mirror::from(input);
    Some(mirror.simulater_laser_count((0, 0), Direction::Right))
}

// 8315
// 8156 too low
pub fn part_two(input: &str) -> Option<usize> {
    let mirror = Mirror::from(input);
    Some(mirror.simulater_laser_maximise())
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
        assert_eq!(result, Some(51));
    }
}
