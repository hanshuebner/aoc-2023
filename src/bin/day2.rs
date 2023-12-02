use std::env;
use std::fs::read_to_string;
use regex::Regex;
use std::collections::HashMap;
use std::hash::Hash;
use std::str::FromStr;

#[macro_use]
extern crate partial_application;
extern crate strum;
#[macro_use] extern crate strum_macros;

#[derive(Debug, Hash, Eq, PartialEq, Clone, EnumString)]
#[strum(ascii_case_insensitive)]
enum Color {
    Red,
    Green,
    Blue
}

#[derive(Debug)]
struct Game {
    id: usize,
    draws: Vec<HashMap<Color, usize>>,
}

fn parse_draw(input_string: &str) -> HashMap<Color, usize> {
    let split_draw = Regex::new(", *").unwrap();
    split_draw.split(&input_string)
        .map(|input_string| input_string.split_whitespace().collect::<Vec<&str>>())
        .map(|parts|
            (Color::from_str(parts[1]).unwrap(),
             parts[0].parse::<usize>().unwrap()))
        .collect()
}

fn parse_game(input_string: &str) -> Game {
    let game_regex = Regex::new(r"^Game (\d+): (.*)$").unwrap();
    let captures = game_regex.captures(&input_string).unwrap();
    let id: usize = captures[1].parse().unwrap();
    let split_draws = Regex::new("; *").unwrap();
    let draws: Vec<HashMap<Color, usize>> = split_draws.split(&captures[2])
        .map(parse_draw)
        .collect();
    Game {
        id,
        draws,
    }
}

fn draw_impossible(bag: &HashMap<Color, usize>, draw: &HashMap<Color, usize>) -> bool {
    draw.iter().any(|(color, count)| bag[color] < *count)
}

fn game_possible(bag: &HashMap<Color, usize>, game: &Game) -> bool {
    game.draws.iter()
        .filter(partial!(draw_impossible => bag, _))
        .next().is_none()
}

fn part_one(filename: &str) -> usize {
    let bag: HashMap<Color, usize> = HashMap::from([(Color::Red, 12), (Color::Green, 13), (Color::Blue, 14)]);
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(parse_game)
        .filter(partial!(game_possible => &bag, _))
        .map(| game | game.id)
        .sum()
}

fn power(game: Game) -> usize {
    let mut min_counts: HashMap<Color, usize> = HashMap::new();
    min_counts.insert(Color::Red, 1);
    min_counts.insert(Color::Green, 1);
    min_counts.insert(Color::Blue, 1);
    for draw in game.draws {
        for color in draw.keys().cloned() {
            let draw_count = draw[&color];
            if min_counts[&color] < draw_count {
                min_counts.insert(color, draw_count);
            }
        }
    }
    min_counts[&Color::Red] * min_counts[&Color::Green] * min_counts[&Color::Blue]
}

fn part_two(filename: &str) -> usize {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(parse_game)
        .map(power)
        .sum()
}

fn main() {
    let filename = env::args().nth(1).unwrap();
    let sum = part_one(&filename);
    println!("Part One - Sum: {sum}");
    let pow_sum = part_two(&filename);
    println!("Part Two - Sum: {pow_sum}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_game() {
        let game = parse_game("Game 18: 3 red; 4 red, 1 blue; 3 green, 3 red; 10 green, 1 blue; 4 red, 6 green, 1 blue; 3 green");
        assert_eq!(game.id, 18);
        assert_eq!(game.draws.len(), 6);
        assert_eq!(*game.draws[1].get(&Color::Red).unwrap(), 4);
    }

    #[test]
    #[should_panic]
    fn test_invalid_input() {
        parse_game("foo");
    }

    #[test]
    fn test_game_possible() {
        let game = parse_game("Game 18: 3 red; 4 red, 1 blue; 3 green, 3 red; 10 green, 1 blue; 4 red, 6 green, 1 blue; 3 green");
        let bag = HashMap::from([(Color::Red, 12), (Color::Green, 13), (Color::Blue, 14)]);
        assert_eq!(game_possible(&bag, &game), true)
    }

    #[test]
    fn test_game_impossible() {
        let game = parse_game("Game 18: 3 red; 4 red, 1 blue; 3 green, 30 red; 10 green, 1 blue; 4 red, 6 green, 1 blue; 3 green");
        let bag = HashMap::from([(Color::Red, 12), (Color::Green, 13), (Color::Blue, 14)]);
        assert_eq!(game_possible(&bag, &game), false);
    }

    #[test]
    fn test_colors() {
        assert_eq!(Color::from_str("red").unwrap(), Color::Red);
        assert_eq!(Color::from_str("green").unwrap(), Color::Green);
        assert_eq!(Color::from_str("blue").unwrap(), Color::Blue);
    }

    #[test]
    fn test_part_powers() {
        let sample_data = [
            ("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", 48),
            ("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue", 12),
            ("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red", 1560),
            ("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red", 630),
            ("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", 36),];
        for (game_string, pow) in sample_data {
            let game = parse_game(game_string);
            assert_eq!(power(game), pow);
        }
    }
}
