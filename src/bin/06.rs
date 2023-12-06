advent_of_code::solution!(6);

struct RaceList {
    races: Vec<Race>,
}

impl From<&str> for RaceList {
    fn from(value: &str) -> Self {
        let mut lines = value.lines();
        let times = lines.next().unwrap();
        let distances = lines.next().unwrap();

        assert!(times.contains("Time:"));
        assert!(distances.contains("Distance:"));

        let races = times
            .split_once(':')
            .unwrap()
            .1
            .split(' ')
            .filter_map(|time| {
                if time.len() > 0 {
                    Some(u64::from_str_radix(time, 10).unwrap())
                } else {
                    None
                }
            })
            .zip(
                distances
                    .split_once(':')
                    .unwrap()
                    .1
                    .split(' ')
                    .filter_map(|distance| {
                        if distance.len() > 0 {
                            Some(u64::from_str_radix(distance, 10).unwrap())
                        } else {
                            None
                        }
                    }),
            )
            .map(|(time, distance)| Race { time, distance })
            .collect();

        RaceList { races }
    }
}

struct Race {
    time: u64,
    distance: u64,
}

// for part2
impl From<&str> for Race {
    fn from(value: &str) -> Self {
        let mut lines = value.lines();
        let time = lines.next().unwrap();
        let distance = lines.next().unwrap();

        assert!(time.contains("Time:"));
        assert!(distance.contains("Distance:"));

        let time = u64::from_str_radix(
            time.split_once(':').unwrap().1.replace(' ', "").as_str(),
            10,
        )
        .unwrap();
        let distance = u64::from_str_radix(
            distance
                .split_once(':')
                .unwrap()
                .1
                .replace(' ', "")
                .as_str(),
            10,
        )
        .unwrap();
        Race { time, distance }
    }
}

impl Race {
    fn count_ways_to_win(&self) -> u64 {
        let time = self.time as f64;
        let distance = self.distance as f64;
        let delta = (time.powf(2.)) - (4. * distance);
        let root1 = (-(delta.sqrt()) + time) / f64::from(2);
        let root2 = ((delta).sqrt() + time) / f64::from(2);
        let root1_int_sup: u64 = if root1.ceil() > root1 {
            root1.ceil()
        } else {
            (root1 + f64::from(1)).ceil()
        } as u64;

        let root2_int_inf: u64 = if root2.floor() < root2 {
            root2.floor()
        } else {
            (root2 - f64::from(1)).floor()
        } as u64;
        (root2_int_inf).min(self.time - 1) - (root1_int_sup).max(1) + 1
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let race_list = RaceList::from(input);
    race_list
        .races
        .into_iter()
        .map(|race| race.count_ways_to_win())
        .reduce(|acc, el| acc * el)
}

pub fn part_two(input: &str) -> Option<u64> {
    let race = Race::from(input);
    Some(race.count_ways_to_win())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
