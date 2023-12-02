advent_of_code::solution!(2);
struct Handful {
    red: u32,
    green: u32,
    blue: u32,
}

impl From<&str> for Handful {
    fn from(value: &str) -> Self {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        let cubes_groups = value.split(',');

        for cube_group in cubes_groups {
            let cube_group = cube_group.strip_prefix(' ').unwrap_or(cube_group);
            let cube_group = cube_group.strip_suffix(' ').unwrap_or(cube_group);
            let (amount, color) = cube_group.split_once(' ').unwrap();
            let amount = u32::from_str_radix(amount, 10).unwrap();
            match color {
                "red" => red = amount,
                "green" => green = amount,
                "blue" => blue = amount,
                _ => panic!(),
            }
        }

        Handful { red, green, blue }
    }
}
struct Game {
    id: u32,
    handfuls_drawn: Vec<Handful>,
}

impl From<&str> for Game {
    fn from(value: &str) -> Self {
        let (game, handfuls) = value.split_once(':').unwrap();
        let game_id = u32::from_str_radix(game.split_once(' ').unwrap().1, 10).unwrap();
        let mut handfuls_drawn = Vec::new();
        let handfuls = handfuls.split(';');
        for handful in handfuls {
            handfuls_drawn.push(Handful::from(handful))
        }
        Game {
            id: game_id,
            handfuls_drawn,
        }
    }
}

impl Game {
    fn power(&self) -> u32 {
        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;

        for handful in &self.handfuls_drawn {
            min_red = min_red.max(handful.red);
            min_green = min_green.max(handful.green);
            min_blue = min_blue.max(handful.blue);
        }

        min_red * min_green * min_blue
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let valid_game = |game: &Game| {
        for handful in &game.handfuls_drawn {
            if handful.red > 12 {
                return false;
            }
            if handful.green > 13 {
                return false;
            }
            if handful.blue > 14 {
                return false;
            }
        }
        return true;
    };
    input
        .lines()
        .map(Game::from)
        .filter_map(|game| {
            if valid_game(&game) {
                Some(game.id)
            } else {
                None
            }
        })
        .reduce(|acc, el| acc + el)
}

pub fn part_two(input: &str) -> Option<u32> {
    input
        .lines()
        .map(Game::from)
        .map(|game| game.power())
        .reduce(|acc, el| acc + el)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
