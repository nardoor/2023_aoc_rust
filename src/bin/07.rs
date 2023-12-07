use std::cmp::Ordering;

advent_of_code::solution!(7);

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug)]
enum HandType {
    HighCard,
    One,
    Two,
    Three,
    FullHouse,
    Four,
    Five,
}

impl HandType {
    fn from2(value: &[Card; 5]) -> HandType {
        let mut uniq = [None; 5];
        let mut uniq_ptr = 0;
        let mut j_count = 0;
        for c in value {
            if c == &Card::J {
                j_count += 1;
            } else if !uniq.contains(&Some(*c)) {
                uniq[uniq_ptr] = Some(*c);
                uniq_ptr += 1;
            }
        }
        let uniq_cards_count = uniq.iter().filter(|&e| e.is_some()).count();

        match uniq_cards_count {
            0 => {
                assert!(j_count == 5);
                Self::Five
            }
            1 => Self::Five,
            2 => {
                assert!(j_count <= 3);
                let [Some(c1), Some(c2), ..] = uniq else {
                    unreachable!();
                };
                let c1_count = value.iter().filter(|&c| *c == c1).count();
                let c2_count = value.iter().filter(|&c| *c == c2).count();
                match (c1_count, c2_count) {
                    (4, 1) | (1, 4) => Self::Four,
                    (3, 2) | (2, 3) => Self::FullHouse,
                    _ if j_count == 3 || j_count == 2 => Self::Four,
                    (1, 3) | (3, 1) if j_count == 1 => Self::Four,
                    (2, 2) if j_count == 1 => Self::FullHouse,
                    _ => unreachable!(),
                }
            }
            3 => {
                assert!(j_count <= 2);
                let [Some(c1), Some(c2), Some(c3), ..] = uniq else {
                    unreachable!();
                };

                let c1_count = value.iter().filter(|&c| *c == c1).count();
                let c2_count = value.iter().filter(|&c| *c == c2).count();
                let c3_count = value.iter().filter(|&c| *c == c3).count();

                match j_count {
                    2 => Self::Three,
                    1 => Self::Three,
                    0 => {
                        if [c1_count, c2_count, c3_count].contains(&3) {
                            Self::Three
                        } else {
                            Self::Two
                        }
                    }
                    _ => unreachable!(),
                }
            }
            4 => Self::One,
            5 => Self::HighCard,
            _ => unreachable!(),
        }
    }
}

impl From<&[Card; 5]> for HandType {
    fn from(value: &[Card; 5]) -> Self {
        let mut uniq = [None; 5];
        let mut uniq_ptr = 0;
        for c in value {
            if !uniq.contains(&Some(*c)) {
                uniq[uniq_ptr] = Some(*c);
                uniq_ptr += 1;
            }
        }

        let uniq_cards_count = uniq.iter().filter(|&e| e.is_some()).count();
        match uniq_cards_count {
            1 => Self::Five,
            2 => {
                let [Some(c1), Some(c2), ..] = uniq else {
                    unreachable!();
                };
                let c1_count = value.iter().filter(|&c| *c == c1).count();
                let c2_count = value.iter().filter(|&c| *c == c2).count();
                match (c1_count, c2_count) {
                    (4, 1) | (1, 4) => Self::Four,
                    (3, 2) | (2, 3) => Self::FullHouse,
                    _ => unreachable!(),
                }
            }
            3 => {
                let [Some(c1), Some(c2), Some(c3), ..] = uniq else {
                    unreachable!();
                };

                let c1_count = value.iter().filter(|&c| *c == c1).count();
                let c2_count = value.iter().filter(|&c| *c == c2).count();
                let c3_count = value.iter().filter(|&c| *c == c3).count();

                if [c1_count, c2_count, c3_count].contains(&3) {
                    Self::Three
                } else {
                    Self::Two
                }
            }
            4 => Self::One,
            5 => Self::HighCard,
            _ => unreachable!(),
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Hand {
    cards: [Card; 5],
    bid: u32,
}

impl From<&str> for Hand {
    fn from(value: &str) -> Self {
        let (cards, bid) = value.split_once(' ').unwrap();
        let cards: [Card; 5] = [
            Card::from(cards.chars().nth(0).unwrap()),
            Card::from(cards.chars().nth(1).unwrap()),
            Card::from(cards.chars().nth(2).unwrap()),
            Card::from(cards.chars().nth(3).unwrap()),
            Card::from(cards.chars().nth(4).unwrap()),
        ];
        Hand {
            cards,
            bid: u32::from_str_radix(bid, 10).unwrap(),
        }
    }
}
#[derive(PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Debug)]
enum Card {
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
    _8,
    _9,
    J,
    T,
    Q,
    K,
    A,
}
impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            'A' => Self::A,
            'K' => Self::K,
            'Q' => Self::Q,
            'J' => Self::J,
            'T' => Self::T,
            '9' => Self::_9,
            '8' => Self::_8,
            '7' => Self::_7,
            '6' => Self::_6,
            '5' => Self::_5,
            '4' => Self::_4,
            '3' => Self::_3,
            '2' => Self::_2,
            _ => unreachable!(),
        }
    }
}

impl Card {
    fn cmp2(&self, other: &Self) -> Ordering {
        if self == other {
            return Ordering::Equal;
        }
        let cards_order_2 = [
            Self::A,
            Self::K,
            Self::Q,
            Self::T,
            Self::_9,
            Self::_8,
            Self::_7,
            Self::_6,
            Self::_5,
            Self::_4,
            Self::_3,
            Self::_2,
            Self::J,
        ];

        let first_found = cards_order_2
            .iter()
            .find(|&c| &c == &self || &c == &other)
            .unwrap();
        if first_found == self {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    }
}

impl Hand {
    fn hand_type(&self) -> HandType {
        HandType::from(&self.cards)
    }
    fn cmp2(&self, other: &Self) -> Ordering {
        let ht1 = HandType::from2(&self.cards);
        let ht2 = HandType::from2(&other.cards);
        match ht1.cmp(&ht2) {
            Ordering::Equal => self
                .cards
                .iter()
                .zip(other.cards.iter())
                .find_map(|(&c1, &c2)| {
                    let comp = c1.cmp2(&c2);
                    if comp == Ordering::Equal {
                        None
                    } else {
                        Some(comp)
                    }
                })
                .unwrap(),
            ht_cmp => ht_cmp,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let ht1 = self.hand_type();
        let ht2 = other.hand_type();
        match ht1.cmp(&ht2) {
            Ordering::Equal => self
                .cards
                .iter()
                .zip(other.cards.iter())
                .find_map(|(&c1, &c2)| {
                    let comp = c1.cmp(&c2);
                    if comp == Ordering::Equal {
                        None
                    } else {
                        Some(comp)
                    }
                }),
            ht_cmp => Some(ht_cmp),
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut hand_list: Vec<Hand> = input.lines().map(Hand::from).collect();
    hand_list.sort();
    hand_list
        .into_iter()
        .enumerate()
        .map(|(index, hand)| (index as u32 + 1) * hand.bid)
        .reduce(|acc, el| acc + el)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut hand_list: Vec<Hand> = input.lines().map(Hand::from).collect();
    hand_list.sort_by(Hand::cmp2);
    hand_list
        .into_iter()
        .enumerate()
        .map(|(index, hand)| (index as u32 + 1) * hand.bid)
        .reduce(|acc, el| acc + el)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order_hand_type() {
        assert!(HandType::Five > HandType::Four)
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
