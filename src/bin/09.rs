advent_of_code::solution!(9);

struct InputParser;

impl InputParser {
    fn parse(input: &str) -> Vec<Vec<isize>> {
        input
            .lines()
            .map(|l| {
                l.split(' ')
                    .map(|n| isize::from_str_radix(n, 10).unwrap())
                    .collect()
            })
            .collect()
    }
}

trait Predict {
    fn predict(&self) -> isize;
    fn predict_backward(&self) -> isize;
}

impl Predict for Vec<isize> {
    fn predict(&self) -> isize {
        let next_vec: Vec<isize> = self.windows(2).map(|s| s[1] - s[0]).collect();
        if next_vec.iter().all(|&v| v == 0) {
            *self.last().unwrap()
        } else {
            let predicted = self.last().unwrap() + next_vec.predict();
            predicted
        }
    }
    fn predict_backward(&self) -> isize {
        let next_vec: Vec<isize> = self.windows(2).map(|s| s[1] - s[0]).collect();
        if next_vec.iter().all(|&v| v == 0) {
            *self.first().unwrap()
        } else {
            let predicted = self.first().unwrap() - next_vec.predict_backward();
            predicted
        }
    }
}

pub fn part_one(input: &str) -> Option<isize> {
    let data = InputParser::parse(input);
    data.into_iter()
        .map(|v| v.predict())
        .reduce(|acc, el| acc + el)
}

pub fn part_two(input: &str) -> Option<isize> {
    let data = InputParser::parse(input);
    data.into_iter()
        .map(|v| v.predict_backward())
        .reduce(|acc, el| acc + el)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
