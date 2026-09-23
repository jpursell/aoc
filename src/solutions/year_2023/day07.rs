use crate::AocSolution;
use counter::Counter;
use itertools::Itertools;

pub struct Day07;

#[derive(Hash, Debug, Eq, PartialEq, PartialOrd, Ord, Clone, Copy)]
enum CardA {
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
    C9,
    T,
    J,
    Q,
    K,
    A,
}

impl TryFrom<char> for CardA {
    type Error = ();

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'A' => Ok(CardA::A),
            'K' => Ok(CardA::K),
            'Q' => Ok(CardA::Q),
            'J' => Ok(CardA::J),
            'T' => Ok(CardA::T),
            '9' => Ok(CardA::C9),
            '8' => Ok(CardA::C8),
            '7' => Ok(CardA::C7),
            '6' => Ok(CardA::C6),
            '5' => Ok(CardA::C5),
            '4' => Ok(CardA::C4),
            '3' => Ok(CardA::C3),
            '2' => Ok(CardA::C2),
            _ => Err(()),
        }
    }
}

#[derive(Hash, Debug, Eq, PartialEq, PartialOrd, Ord, Clone, Copy)]
enum CardB {
    J,
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
    C9,
    T,
    Q,
    K,
    A,
}

impl TryFrom<char> for CardB {
    type Error = ();

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'A' => Ok(CardB::A),
            'K' => Ok(CardB::K),
            'Q' => Ok(CardB::Q),
            'J' => Ok(CardB::J),
            'T' => Ok(CardB::T),
            '9' => Ok(CardB::C9),
            '8' => Ok(CardB::C8),
            '7' => Ok(CardB::C7),
            '6' => Ok(CardB::C6),
            '5' => Ok(CardB::C5),
            '4' => Ok(CardB::C4),
            '3' => Ok(CardB::C3),
            '2' => Ok(CardB::C2),
            _ => Err(()),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOAK,
    FullHouse,
    FourOAK,
    FiveOAK,
}

fn determine_hand_type_a(cards: &[CardA]) -> HandType {
    let most_common = cards.iter().collect::<Counter<_>>().most_common_ordered();
    let signature: Vec<usize> = most_common.into_iter().map(|(_, count)| count).collect();
    match signature.len() {
        1 => HandType::FiveOAK,
        2 => match signature.iter().collect_tuple().unwrap() {
            (4, 1) => HandType::FourOAK,
            (3, 2) => HandType::FullHouse,
            _ => unreachable!(),
        },
        3 => match signature.iter().collect_tuple().unwrap() {
            (3, 1, 1) => HandType::ThreeOAK,
            (2, 2, 1) => HandType::TwoPair,
            _ => unreachable!(),
        },
        4 => HandType::OnePair,
        5 => HandType::HighCard,
        _ => unreachable!(),
    }
}

fn determine_hand_type_b(cards: &[CardB]) -> HandType {
    let non_j_cards: Vec<CardB> = cards.iter().copied().filter(|&c| c != CardB::J).collect();
    let most_common = non_j_cards
        .iter()
        .collect::<Counter<_>>()
        .most_common_ordered();
    let mut signature: Vec<usize> = most_common.into_iter().map(|(_, count)| count).collect();
    let num_j = 5 - non_j_cards.len();
    if signature.is_empty() {
        signature.push(num_j);
    } else {
        signature[0] += num_j;
    }
    match signature.len() {
        1 => HandType::FiveOAK,
        2 => match signature.iter().collect_tuple().unwrap() {
            (4, 1) => HandType::FourOAK,
            (3, 2) => HandType::FullHouse,
            _ => unreachable!(),
        },
        3 => match signature.iter().collect_tuple().unwrap() {
            (3, 1, 1) => HandType::ThreeOAK,
            (2, 2, 1) => HandType::TwoPair,
            _ => unreachable!(),
        },
        4 => HandType::OnePair,
        5 => HandType::HighCard,
        _ => unreachable!(),
    }
}

fn day_7_a(input: &str) -> u64 {
    let mut hands: Vec<(HandType, Vec<CardA>, u64)> = input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|line| {
            let (hand_str, bid_str) = line.trim().split_once(' ').unwrap();
            let cards: Vec<CardA> = hand_str.chars().map(|c| c.try_into().unwrap()).collect();
            let hand_type = determine_hand_type_a(&cards);
            let bid = bid_str.parse::<u64>().unwrap();
            (hand_type, cards, bid)
        })
        .collect();
    hands.sort();
    hands
        .iter()
        .enumerate()
        .map(|(rank, (_, _, bid))| bid * (rank as u64 + 1))
        .sum()
}

fn day_7_b(input: &str) -> u64 {
    let mut hands: Vec<(HandType, Vec<CardB>, u64)> = input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|line| {
            let (hand_str, bid_str) = line.trim().split_once(' ').unwrap();
            let cards: Vec<CardB> = hand_str.chars().map(|c| c.try_into().unwrap()).collect();
            let hand_type = determine_hand_type_b(&cards);
            let bid = bid_str.parse::<u64>().unwrap();
            (hand_type, cards, bid)
        })
        .collect();
    hands.sort();
    hands
        .iter()
        .enumerate()
        .map(|(rank, (_, _, bid))| bid * (rank as u64 + 1))
        .sum()
}

impl AocSolution for Day07 {
    fn part1(&self, input: &str) -> String {
        day_7_a(input).to_string()
    }

    fn part2(&self, input: &str) -> String {
        day_7_b(input).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;

    #[test]
    fn test_part1_example() {
        assert_eq!(Day07.part1(EXAMPLE), "6440");
    }

    #[test]
    fn test_part1_full() {
        let input = crate::get_input_for_day(2023, 7).expect("Failed to get input");
        assert_eq!(Day07.part1(&input), "250474325");
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(Day07.part2(EXAMPLE), "5905");
    }

    #[test]
    fn test_part2_full() {
        let input = crate::get_input_for_day(2023, 7).expect("Failed to get input");
        assert_eq!(Day07.part2(&input), "248909434");
    }
}
