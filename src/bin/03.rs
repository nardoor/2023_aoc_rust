advent_of_code::solution!(3);

// 521104 too low
// 521601
// 522978 too high

struct NumberParse {
    start_x: usize,
    length: usize,
    y: usize,
}

struct Engine {
    map: Vec<Vec<char>>,
}

impl From<&str> for Engine {
    fn from(value: &str) -> Self {
        Engine {
            map: value.lines().map(|line| line.chars().collect()).collect(),
        }
    }
}

impl Engine {
    fn get(&self, pos: (usize, usize)) -> Option<char> {
        if pos.1 < self.map.len() && pos.0 < self.map[0].len() {
            Some(self.get_unchecked(pos))
        } else {
            None
        }
    }
    fn get_unchecked(&self, pos: (usize, usize)) -> char {
        self.map[pos.1][pos.0]
    }
    fn get_parts_numbers(&self) -> Vec<u32> {
        let char_is_number = |c: char| c as u32 <= '9' as u32 && c as u32 >= '0' as u32;
        let is_symbol = |c: char| !char_is_number(c) && c != '.';
        let mut numbers = Vec::new();
        let mut current_parse_pos = (0, 0);
        let max_y = self.map.len();
        assert!(max_y > 0);
        let max_x = self.map[0].len();
        let increment_parse_pos = |current_parse_pos: &mut (usize, usize)| {
            current_parse_pos.0 += 1;
            if current_parse_pos.0 >= max_x {
                current_parse_pos.0 = 0;
                current_parse_pos.1 += 1;
            }
        };

        'parsing: while current_parse_pos.0 < max_x && current_parse_pos.1 < max_y {
            match self.get_unchecked(current_parse_pos) {
                num_char if char_is_number(num_char) => {
                    // found start of number, parse it
                    let mut number_str = num_char.to_string();
                    let start_x = current_parse_pos.0;
                    let line = current_parse_pos.1;
                    while (current_parse_pos.0 + 1) < max_x
                        && char_is_number(
                            self.get_unchecked((current_parse_pos.0 + 1, current_parse_pos.1)),
                        )
                    {
                        current_parse_pos.0 += 1;
                        number_str += &self.get_unchecked(current_parse_pos).to_string();
                    }
                    // done parsing number, increment pos
                    increment_parse_pos(&mut current_parse_pos);

                    let number_parse = NumberParse {
                        length: number_str.len(),
                        start_x,
                        y: line,
                    };
                    // look around number for symbols
                    // look behind
                    if let Some(x) = number_parse.start_x.checked_sub(1) {
                        if is_symbol(self.get_unchecked((x, number_parse.y))) {
                            numbers.push(u32::from_str_radix(&number_str, 10).unwrap());
                            continue 'parsing;
                        }
                    }
                    // look after
                    if let Some(c) =
                        self.get((number_parse.start_x + number_parse.length, number_parse.y))
                    {
                        if is_symbol(c) {
                            numbers.push(u32::from_str_radix(&number_str, 10).unwrap());
                            continue 'parsing;
                        }
                    }
                    // look on top
                    if let Some(y) = number_parse.y.checked_sub(1) {
                        for x in number_parse.start_x.saturating_sub(1)
                            ..=(number_parse.start_x + number_parse.length)
                        {
                            if let Some(c) = self.get((x, y)) {
                                if is_symbol(c) {
                                    numbers.push(u32::from_str_radix(&number_str, 10).unwrap());
                                    continue 'parsing;
                                }
                            }
                        }
                    }
                    // look on bottom
                    let bottom_y = number_parse.y + 1;
                    if bottom_y >= max_y {
                        continue 'parsing;
                    } else {
                        for x in number_parse.start_x.saturating_sub(1)
                            ..=(number_parse.start_x + number_parse.length)
                        {
                            if let Some(c) = self.get((x, bottom_y)) {
                                if is_symbol(c) {
                                    numbers.push(u32::from_str_radix(&number_str, 10).unwrap());
                                    continue 'parsing;
                                }
                            }
                        }
                    }
                }
                _ => increment_parse_pos(&mut current_parse_pos),
            }
        }
        numbers
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Engine::from(input)
        .get_parts_numbers()
        .into_iter()
        .reduce(|acc, el| acc + el)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_parts_number() {
        let input: &str = ".....\n\
                           .123.\n\
                           .....";
        assert_eq!(Engine::from(input).get_parts_numbers(), vec![]);

        let input: &str = "*....\n\
                           .123.\n\
                           .....";
        assert_eq!(Engine::from(input).get_parts_numbers(), vec![123]);
        let input: &str = "....*\n\
                           .123.\n\
                           .....";
        assert_eq!(Engine::from(input).get_parts_numbers(), vec![123]);
        let input: &str = ".....\n\
                           .123.\n\
                           ....*";
        assert_eq!(Engine::from(input).get_parts_numbers(), vec![123]);
        let input: &str = ".....\n\
                           .123.\n\
                           *....";
        assert_eq!(Engine::from(input).get_parts_numbers(), vec![123]);
        let input: &str = ".....\n\
                           *123.\n\
                           .....";
        assert_eq!(Engine::from(input).get_parts_numbers(), vec![123]);
        let input: &str = ".....\n\
                           .123*\n\
                           .....";
        assert_eq!(Engine::from(input).get_parts_numbers(), vec![123]);
        let input: &str = "*******\n\
                           *.....*\n\
                           *.123.*\n\
                           *.....*\n\
                           *******";
        assert_eq!(Engine::from(input).get_parts_numbers(), vec![]);
        let input: &str = ".................\n\
                           .79*888.250*461.*\n\
                           .................";
        assert_eq!(
            Engine::from(input).get_parts_numbers(),
            vec![79, 888, 250, 461]
        );
    }
    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
