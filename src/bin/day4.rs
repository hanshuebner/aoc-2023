use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::read_to_string;
use regex::Regex;

#[derive(Debug, Clone)]
struct Card {
    id: usize,
    winning: HashSet<usize>,
    present: HashSet<usize>,
}

fn parse_numbers(input: &str) -> HashSet<usize> {
    input
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}

fn read_lottery_cards(input_lines: &str) -> Vec<Card> {
    let regex = Regex::new(r"(?m)^Card *(\d+): ([\d ]+) \| ([\d ]+)$").unwrap();

    let mut cards = vec![];
    for (_, [id, winning, present]) in regex.captures_iter(input_lines).map(|c| c.extract()) {
        cards.push(Card {
            id: id.parse::<usize>().unwrap(),
            winning: parse_numbers(winning),
            present: parse_numbers(present),
        })
    }
    assert_eq!(input_lines.matches("\n").count(), cards.len());
    cards
}

fn card_match_count(card: &Card) -> usize {
    card.present.intersection(&card.winning).count()
}

fn score_part_1(cards: &Vec<Card>) -> usize {
    cards
        .iter()
        .map(card_match_count)
        .map(|count|
            if count > 0 {
                usize::pow(2, (count - 1) as u32)
            } else {
                0
            })
        .sum()
}

fn record_copy(id: usize, copies: &mut HashMap<usize, usize>, max_id: usize) {
    if id <= max_id {
        *copies.entry(id).or_insert(0) += 1;
    }
}

fn score_part_2(cards: &Vec<Card>) -> usize {
    let mut copies: HashMap<usize, usize> = HashMap::new();
    let max_id = cards.len() + 1;
    for card in cards {
        record_copy(card.id, &mut copies, max_id);
        for _copy in 0..copies[&card.id] {
            for won in 0..card_match_count(card) {
                record_copy(card.id + 1 + won, &mut copies, max_id);
            }
        }
    }
    copies.values().sum()
}

fn main() {
    let filename = env::args().nth(1).unwrap();
    let cards = read_lottery_cards(&read_to_string(filename).unwrap());
    println!("part 1: {}", score_part_1(&cards));
    println!("part 2: {}", score_part_2(&cards))
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\nCard 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\nCard 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\nCard 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\nCard 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\nCard 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11\n";

    #[test]
    fn test_part_1() {
        let cards = read_lottery_cards(TEST_INPUT);
        assert_eq!(score_part_1(&cards), 13);
        assert_eq!(score_part_2(&cards), 30)
    }
}
