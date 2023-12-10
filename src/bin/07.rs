use std::cmp;

advent_of_code::solution!(7);
#[derive(Debug)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Debug)]

struct Hand {
    cards: [u32; 5],
    hand_type: HandType,
    bid: u32,
}

impl Hand {
    pub fn convert_card(c: char) -> u32 {
        match c {
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            'T' => 10,
            'J' => 11,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => {
                panic!("Invalid card")
            }
        }
    }

    fn get_hand_type(card_counts: &[i32; 13]) -> HandType {
        let mut hand_type = HandType::HighCard;
        for count in card_counts.iter() {
            if count == &5 {
                hand_type = HandType::FiveOfAKind;
                break;
            } else if count == &4 {
                hand_type = HandType::FourOfAKind;
                break;
            } else if count == &3 {
                if hand_type == HandType::OnePair {
                    hand_type = HandType::FullHouse;
                } else {
                    hand_type = HandType::ThreeOfAKind;
                }
            } else if count == &2 {
                if hand_type == HandType::ThreeOfAKind {
                    hand_type = HandType::FullHouse;
                } else if hand_type == HandType::OnePair {
                    hand_type = HandType::TwoPair;
                } else if hand_type < HandType::OnePair {
                    hand_type = HandType::OnePair;
                }
            }
        }
        hand_type
    }

    fn raw_from(input: &str, use_jokers: bool) -> Self {
        let input = input.split_once(" ").unwrap();
        let hand = input.0;
        let bid = input.1.parse::<u32>().unwrap();
        let mut jokers = 0;
        let mut cards = [0; 5];
        let mut card_counts = [0; 13];
        for (i, card) in hand.chars().map(Self::convert_card).enumerate() {
            if use_jokers && card == 11 {
                cards[i] = 1;
                jokers += 1;
            } else {
                cards[i] = card;
                card_counts[card as usize % 13] += 1;
            }
        }

        // Substituted the Joker for every possible card, try all, use the best one:
        if jokers > 0 {
            Self {
                cards,
                hand_type: (0..13)
                    .map(|i| {
                        let mut possible_card_count = card_counts.clone();
                        possible_card_count[i] += jokers;
                        possible_card_count
                    })
                    .map(|c| Self::get_hand_type(&c))
                    .max()
                    .unwrap(),
                bid,
            }
        } else {
            Self {
                cards,
                hand_type: Self::get_hand_type(&card_counts),
                bid,
            }
        }
    }
    pub fn from_normal(input: &str) -> Self {
        Self::raw_from(input, false)
    }
    pub fn from_jokers(input: &str) -> Self {
        Self::raw_from(input, true)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut cards = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|s| Hand::from_normal(s))
        .collect::<Vec<_>>();
    cards.sort();

    Some(
        cards
            .iter()
            .enumerate()
            .fold(0, |acc, (i, card)| acc + (i as u32 + 1) * card.bid),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut cards = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|s| Hand::from_jokers(s))
        .collect::<Vec<_>>();
    cards.sort();

    Some(
        cards
            .iter()
            .enumerate()
            .fold(0, |acc, (i, card)| acc + (i as u32 + 1) * card.bid),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

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

// ----- Implementing ordering traits for HandType -----

impl cmp::Ord for HandType {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        let worth_self = match self {
            HandType::FiveOfAKind => 8,
            HandType::FourOfAKind => 7,
            HandType::FullHouse => 6,
            HandType::ThreeOfAKind => 5,
            HandType::TwoPair => 4,
            HandType::OnePair => 3,
            HandType::HighCard => 2,
        };
        let worth_other = match other {
            HandType::FiveOfAKind => 8,
            HandType::FourOfAKind => 7,
            HandType::FullHouse => 6,
            HandType::ThreeOfAKind => 5,
            HandType::TwoPair => 4,
            HandType::OnePair => 3,
            HandType::HighCard => 2,
        };
        worth_self.cmp(&worth_other)
    }
}
impl cmp::PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl cmp::PartialEq for HandType {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == cmp::Ordering::Equal
    }
}
impl cmp::Eq for HandType {}

// ----- Implementing ordering traits for Hand -----

impl cmp::Ord for Hand {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        if self.hand_type > other.hand_type {
            return cmp::Ordering::Greater;
        } else if self.hand_type < other.hand_type {
            return cmp::Ordering::Less;
        } else {
            for (self_card, other_card) in self.cards.iter().zip(other.cards.iter()) {
                if self_card > other_card {
                    return cmp::Ordering::Greater;
                } else if self_card < other_card {
                    return cmp::Ordering::Less;
                }
            }
        }
        cmp::Ordering::Equal
    }
}
impl cmp::PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}
impl cmp::Eq for Hand {}
impl cmp::PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}
