use std::collections::VecDeque;

use rayon::iter::{ParallelBridge, ParallelIterator};

advent_of_code::solution!(4);

struct Card {
    winnig: Vec<u32>,
    numbers: Vec<u32>,
}

impl From<&str> for Card {
    fn from(value: &str) -> Self {
        let (_card, winning_and_numbers) = value.split_once(':').unwrap();
        let (winning, numbers) = winning_and_numbers.split_once('|').unwrap();

        Card {
            winnig: winning
                .split(' ')
                .filter(|n| !n.is_empty())
                .map(|n| u32::from_str_radix(n, 10).unwrap())
                .collect(),
            numbers: numbers
                .split(' ')
                .filter(|n| !n.is_empty())
                .map(|n| u32::from_str_radix(n, 10).unwrap())
                .collect(),
        }
    }
}

impl Card {
    fn score(&self) -> u32 {
        let mut score = 0;
        for n in &self.numbers {
            if self.winnig.contains(n) {
                if score == 0 {
                    score = 1;
                } else {
                    score *= 2;
                }
            }
        }
        score
    }

    fn score_part_2(&self) -> u32 {
        self.numbers
            .iter()
            .filter(|n| self.winnig.contains(n))
            .count() as u32
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .par_bridge()
            .map(Card::from)
            .map(|c| c.score())
            .reduce(|| 0, |acc, el| acc + el),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    // this is the memory of how many copies we have for following cards
    let mut memory = VecDeque::new();
    Some(
        input
            .lines()
            .map(Card::from)
            .map(|c| c.score_part_2())
            .fold((&mut memory, 0), |(memory, cc), score| {
                let current_card_copies = memory.pop_front().unwrap_or(1);
                for delta in 0..(score as usize) {
                    if let Some(future_copies) = memory.get(delta) {
                        memory[delta] = future_copies + current_card_copies;
                    } else {
                        // original cards + 1
                        memory.push_back(1 + current_card_copies);
                    }
                }
                (memory, cc + current_card_copies)
            })
            .1,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
