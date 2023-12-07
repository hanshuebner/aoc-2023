#![feature(slice_group_by)]

use ::phf::{phf_map, Map};
use std::cmp::Ordering;
use std::env;
use std::fs::read_to_string;
use trace::trace;

trace::init_depth_var!();

static CARD_VALUES_1: Map<char, usize> = phf_map! {
    'A' => 12,
    'K' => 11,
    'Q' => 10,
    'J' => 9,
    'T' => 8,
    '9' => 7,
    '8' => 6,
    '7' => 5,
    '6' => 4,
    '5' => 3,
    '4' => 2,
    '3' => 1,
    '2' => 0,
};

static CARD_VALUES_2: Map<char, usize> = phf_map! {
    'A' => 12,
    'K' => 11,
    'Q' => 10,
    'T' => 9,
    '9' => 8,
    '8' => 7,
    '7' => 6,
    '6' => 5,
    '5' => 4,
    '4' => 3,
    '3' => 2,
    '2' => 1,
    'J' => 0,
};

#[derive(Debug)]
struct Hand {
    cards: String,
    card_values: Vec<usize>,
    value: usize,
    bid: usize,
}

fn parse_hand(
    input: &str,
    card_value_map: &Map<char, usize>,
    make_strands: fn(&Vec<usize>) -> Vec<usize>,
) -> Hand {
    let splits: Vec<String> = input.split(" ").map(String::from).collect();
    let card_values: Vec<usize> = splits[0].chars().map(|c| card_value_map[&c]).collect();
    let value = hand_value(make_strands(&card_values));
    Hand {
        cards: splits[0].to_string(),
        card_values,
        value,
        bid: splits.get(1).unwrap_or(&"0".to_string()).parse().unwrap(),
    }
}

fn parse_input(
    input: &str,
    card_value_map: &Map<char, usize>,
    make_strands: fn(&Vec<usize>) -> Vec<usize>,
) -> Vec<Hand> {
    input
        .split("\n")
        .filter(|s| s.len() > 0)
        .map(|s| parse_hand(s, card_value_map, make_strands))
        .collect()
}

fn make_strands_1(card_values: &Vec<usize>) -> Vec<usize> {
    let mut sorted_by_card_value = card_values.clone();
    sorted_by_card_value.sort();

    let mut strands: Vec<usize> = sorted_by_card_value
        .to_vec()
        .group_by(|&a, &b| a == b)
        .map(|strand| strand.len())
        .collect();
    strands.sort_by(|a, b| b.cmp(a));
    strands
}

fn make_strands_2(card_values: &Vec<usize>) -> Vec<usize> {
    let mut sorted_by_card_value = card_values.clone();
    sorted_by_card_value.sort();

    let mut strands_and_lengths: Vec<(usize, usize)> = sorted_by_card_value
        .to_vec()
        .group_by(|&a, &b| a == b)
        .map(|strand| (strand[0], strand.len()))
        .collect();
    strands_and_lengths.sort_by(|a, b| {
        // sort joker strand first, then by increasing length
        if a.0 == 0 {
            Ordering::Less
        } else if b.0 == 0 {
            Ordering::Greater
        } else {
            b.1.cmp(&a.1)
        }
    });
    if strands_and_lengths[0].0 == 0 && strands_and_lengths[0].1 < 5 {
        // merge joker strand with next strand
        let joker_count = strands_and_lengths[0].1;
        strands_and_lengths.remove(0);
        strands_and_lengths[0] = (
            strands_and_lengths[0].0,
            strands_and_lengths[0].1 + joker_count,
        )
    }
    let mut strands: Vec<usize> = strands_and_lengths.iter().map(|strand| strand.1).collect();
    strands.sort_by(|a, b| b.cmp(a));
    strands
}

fn hand_value(strands: Vec<usize>) -> usize {
    match (strands[0], strands.get(1)) {
        (5, _) => 6,
        (4, _) => 5,
        (3, Some(2)) => 4,
        (3, _) => 3,
        (2, Some(2)) => 2,
        (2, _) => 1,
        _ => 0,
    }
}

fn compare_hands(a: &Hand, b: &Hand) -> Ordering {
    assert_ne!(a.cards.cmp(&b.cards), Ordering::Equal);
    if a.value > b.value {
        Ordering::Greater
    } else if a.value < b.value {
        Ordering::Less
    } else {
        a.card_values
            .iter()
            .zip(b.card_values.iter())
            .map(|(ca, cb)| ca.cmp(cb))
            .find(|&o| o != Ordering::Equal)
            .unwrap() // can't have two equal elements in input set, so panic if we do
    }
}

fn compute_result(
    input: &str,
    card_value_map: &Map<char, usize>,
    make_strands: fn(&Vec<usize>) -> Vec<usize>,
) -> usize {
    let mut hands = parse_input(input, card_value_map, make_strands);
    hands.sort_by(compare_hands);

    hands
        .iter()
        .enumerate()
        .map(|(index, hand)| (index + 1) * hand.bid)
        .sum()
}

fn main() {
    let filename = env::args().nth(1).unwrap();
    let input = &read_to_string(filename).unwrap();
    println!(
        "part 1: {:?}",
        compute_result(input, &CARD_VALUES_1, make_strands_1)
    );
    println!(
        "part 2: {:?}",
        compute_result(input, &CARD_VALUES_2, make_strands_2)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "\
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";

    fn parse_hand_1(input: &str) -> Hand {
        parse_hand(input, &CARD_VALUES_1, make_strands_1)
    }

    #[test]
    fn test_hand_comparison_1() {
        assert_eq!(
            compare_hands(&parse_hand_1("33332"), &parse_hand_1("2AAAA")),
            Ordering::Greater
        );
        assert_eq!(
            compare_hands(&parse_hand_1("77888"), &parse_hand_1("77788")),
            Ordering::Greater
        );

        assert_eq!(
            compare_hands(&parse_hand_1("KK677"), &parse_hand_1("32T3K")),
            Ordering::Greater
        );
        assert_eq!(
            compare_hands(&parse_hand_1("KK677"), &parse_hand_1("KTJJT")),
            Ordering::Greater
        );
        assert_eq!(
            compare_hands(&parse_hand_1("T55J5"), &parse_hand_1("KTJJT")),
            Ordering::Greater
        );
        assert_eq!(
            compare_hands(&parse_hand_1("QQQJA"), &parse_hand_1("T55J5")),
            Ordering::Greater
        );

        assert_eq!(
            compare_hands(&parse_hand_1("T55J5"), &parse_hand_1("QQQJA")),
            Ordering::Less
        );
    }

    fn parse_hand_2(input: &str) -> Hand {
        parse_hand(input, &CARD_VALUES_2, make_strands_2)
    }

    #[test]
    fn test_hand_comparison_2() {
        assert_eq!(
            compare_hands(&parse_hand_2("JJJJJ"), &parse_hand_2("22222")),
            Ordering::Less
        );
        assert_eq!(
            compare_hands(&parse_hand_2("JKKK2"), &parse_hand_2("QQQQ2")),
            Ordering::Less
        );

        assert_eq!(
            compare_hands(&parse_hand_2("22222"), &parse_hand_2("JJJJJ")),
            Ordering::Greater
        );
        assert_eq!(
            compare_hands(&parse_hand_2("QQQQ2"), &parse_hand_2("JKKK2")),
            Ordering::Greater
        );
    }

    #[test]
    fn test_part_1() {
        assert_eq!(
            compute_result(TEST_INPUT, &CARD_VALUES_1, make_strands_1),
            6440
        )
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            compute_result(TEST_INPUT, &CARD_VALUES_2, make_strands_2),
            5905
        )
    }
}
