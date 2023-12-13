advent_of_code::solution!(13);

enum Element {
    Ash,
    Rock,
}

impl From<char> for Element {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Ash,
            '#' => Self::Rock,
            _ => unreachable!(),
        }
    }
}

impl From<&Element> for usize {
    fn from(value: &Element) -> Self {
        match value {
            Element::Ash => 1,
            Element::Rock => 0,
        }
    }
}

struct Pattern {
    map: Vec<Vec<Element>>,
}

impl From<&str> for Pattern {
    fn from(value: &str) -> Self {
        Self {
            map: value
                .lines()
                .map(|l| l.chars().map(Element::from).collect())
                .collect(),
        }
    }
}

enum ReflectResult {
    Line(usize),
    Column(usize),
}

impl ReflectResult {
    fn line(n: usize) -> Self {
        Self::Line(n)
    }
    fn column(n: usize) -> Self {
        Self::Column(n)
    }
}

impl Pattern {
    fn get_reflection(&self, need_smudge: bool) -> ReflectResult {
        let lines: Vec<usize> = self
            .map
            .iter()
            .map(|e| {
                e.iter()
                    .map(usize::from)
                    .enumerate()
                    .map(|(i, n)| n << i)
                    .sum()
            })
            .collect();

        let col_count = self.map[0].len();
        let columns: Vec<usize> = (0..col_count)
            .map(|col| {
                self.map
                    .iter()
                    .enumerate()
                    .map(|(i, l)| usize::from(l.iter().nth(col).unwrap()) << i)
                    .sum()
            })
            .collect();
        for (nums, result_fn) in [
            (lines, ReflectResult::line as fn(usize) -> ReflectResult),
            (columns, ReflectResult::column as fn(usize) -> ReflectResult),
        ] {
            for i in 0..nums.len() {
                // mirror at i,i+1
                let mut i1 = i;
                let mut i2 = i + 1;
                let mut smudge = false;
                if i2 >= nums.len() {
                    break;
                }
                let symmetry = 'symmetry_loop: loop {
                    if nums[i1] != nums[i2] {
                        if need_smudge && !smudge {
                            let xored = nums[i1] ^ nums[i2];
                            // we know for sure xored != 0
                            debug_assert!(xored != 0);
                            if (xored & (xored - 1)) == 0 {
                                smudge = true;
                            } else {
                                break 'symmetry_loop false;
                            }
                        } else {
                            break 'symmetry_loop false;
                        }
                    }
                    if let Some(new_i1) = i1.checked_sub(1) {
                        i1 = new_i1;
                    } else {
                        break 'symmetry_loop true;
                    }

                    i2 += 1;
                    if i2 >= nums.len() {
                        break 'symmetry_loop true;
                    }
                };

                if symmetry && !(need_smudge ^ smudge) {
                    return result_fn(i + 1);
                }
            }
        }
        panic!("Failed to find symmetry for Pattern.");
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let (line, col) = input
        .split("\n\n")
        .map(Pattern::from)
        .map(|p| p.get_reflection(false))
        .fold((0, 0), |acc, el| match el {
            ReflectResult::Column(n) => (acc.0, acc.1 + n),
            ReflectResult::Line(n) => (acc.0 + n, acc.1),
        });
    Some(line * 100 + col)
}

// 38225 too high
pub fn part_two(input: &str) -> Option<usize> {
    let (line, col) = input
        .split("\n\n")
        .map(Pattern::from)
        .map(|p| p.get_reflection(true))
        .fold((0, 0), |acc, el| match el {
            ReflectResult::Column(n) => (acc.0, acc.1 + n),
            ReflectResult::Line(n) => (acc.0 + n, acc.1),
        });
    Some(line * 100 + col)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(400));
    }
}
