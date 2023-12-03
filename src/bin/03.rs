use std::collections::HashMap;

advent_of_code::solution!(3);

fn char_is_number(c: char) -> bool {
    c as u32 <= '9' as u32 && c as u32 >= '0' as u32
}
fn is_symbol(c: char) -> bool {
    !char_is_number(c) && c != '.'
}
struct NumberParser<'a> {
    parser_pos: (usize, usize),
    engine: &'a Engine,
}

impl<'a> NumberParser<'a> {
    fn new(engine: &'a Engine) -> Self {
        NumberParser {
            parser_pos: (0, 0),
            engine: engine,
        }
    }
    fn cur_x_y(&self) -> (usize, usize) {
        self.parser_pos
    }
    fn cur_x(&self) -> usize {
        self.parser_pos.0
    }
    fn cur_x_mut(&mut self) -> &mut usize {
        &mut self.parser_pos.0
    }
    fn cur_y(&self) -> usize {
        self.parser_pos.1
    }
    fn cur_y_mut(&mut self) -> &mut usize {
        &mut self.parser_pos.1
    }
    fn done_parsing(&self) -> bool {
        self.cur_y() >= self.engine.max_y
    }
    fn increment_pos(&mut self) {
        *self.cur_x_mut() += 1;
        if self.cur_x() >= self.engine.max_x {
            *self.cur_x_mut() = 0;
            *self.cur_y_mut() += 1;
        }
    }

    /// Return `None` if done parsing
    fn parse_next_number(&mut self) -> Option<ParsedNumber> {
        while !self.done_parsing() {
            match self.engine.get_unchecked(self.cur_x_y()) {
                num_char if char_is_number(num_char) => {
                    let mut num = num_char.to_string();
                    let num_x = self.cur_x();
                    let num_y = self.cur_y();
                    while let Some(c) = self.engine.get((self.cur_x() + 1, self.cur_y())) {
                        if char_is_number(c) {
                            // consume char
                            self.increment_pos();
                            num += &c.to_string();
                        } else {
                            break;
                        }
                    }
                    // precedent pos has been consummed for sure
                    self.increment_pos();

                    return Some(ParsedNumber {
                        start_x: num_x,
                        y: num_y,
                        length: num.len(),
                        number_str: num,
                    });
                }
                _ => self.increment_pos(),
            }
        }
        None
    }
}

impl<'a> Iterator for NumberParser<'a> {
    type Item = ParsedNumber;
    fn next(&mut self) -> Option<Self::Item> {
        self.parse_next_number()
    }
}

#[derive(Clone)]
struct ParsedNumber {
    start_x: usize,
    length: usize,
    y: usize,
    number_str: String,
}

impl ParsedNumber {
    fn number(&self) -> u32 {
        u32::from_str_radix(&self.number_str, 10).unwrap()
    }
    fn get_surround(&self) -> Vec<(usize, usize)> {
        let mut surround = Vec::new();
        // on left
        if let Some(x) = self.start_x.checked_sub(1) {
            surround.push((x, self.y));
        }
        // on right
        surround.push((self.start_x + self.length, self.y));

        // on top
        if let Some(y) = self.y.checked_sub(1) {
            for x in (self.start_x.saturating_sub(1))..=(self.start_x + self.length) {
                surround.push((x, y));
            }
        }

        // on bottom
        for x in (self.start_x.saturating_sub(1))..=(self.start_x + self.length) {
            surround.push((x, self.y + 1));
        }
        surround
    }
}

struct Engine {
    map: Vec<Vec<char>>,
    max_x: usize,
    max_y: usize,
}

impl From<&str> for Engine {
    fn from(value: &str) -> Self {
        let map: Vec<Vec<char>> = value.lines().map(|line| line.chars().collect()).collect();
        let max_x = map[0].len();
        let max_y = map.len();
        assert!(map.len() > 0);
        Engine { map, max_x, max_y }
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
    fn is_part(&self, parsed_number: &ParsedNumber) -> bool {
        for pos in parsed_number.get_surround() {
            if let Some(c) = self.get(pos) {
                if is_symbol(c) {
                    return true;
                }
            }
        }
        false
    }
    fn get_stars(&self, parsed_number: &ParsedNumber) -> Option<Vec<(usize, usize)>> {
        let res: Vec<(usize, usize)> = parsed_number
            .get_surround()
            .into_iter()
            .filter(|&pos| Some('*') == self.get(pos))
            .collect();
        if res.len() == 0 {
            None
        } else {
            Some(res)
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let engine = Engine::from(input);
    let parser = NumberParser::new(&engine);
    let mut sum = 0;
    for parsed_number in parser {
        if engine.is_part(&parsed_number) {
            sum += parsed_number.number();
        }
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let engine = Engine::from(input);
    let parser = NumberParser::new(&engine);
    let mut hash_map: HashMap<(usize, usize), Vec<ParsedNumber>> = HashMap::new();

    let mut sum = 0;

    for parsed_number in parser {
        if let Some(stars) = engine.get_stars(&parsed_number) {
            for star_pos in stars {
                hash_map
                    .entry(star_pos)
                    .and_modify(|v| v.push(parsed_number.clone()))
                    .or_insert(vec![parsed_number.clone()]);
            }
        }
    }
    for (_star_pos, v) in hash_map {
        if v.len() == 2 {
            sum += v[0].number() * v[1].number();
        }
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
