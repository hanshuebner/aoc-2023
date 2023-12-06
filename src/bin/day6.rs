use regex::Regex;
use std::env;
use std::fs::read_to_string;
use trace::trace;

trace::init_depth_var!();

#[derive(Debug)]
struct Race {
    time: usize,
    record: usize,
}

type Races = Vec<Race>;

fn parse_part_1(input: &str) -> Vec<usize> {
    Regex::new(r": *")
        .unwrap()
        .split(input)
        .skip(1)
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn parse_part_2(input: &str) -> usize {
    Regex::new(r": *")
        .unwrap()
        .split(input)
        .skip(1)
        .next()
        .unwrap()
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .parse()
        .unwrap()
}

fn parse_input_1(input: &str) -> Races {
    let lines: Vec<&str> = input.split("\n").collect();
    let times: Vec<usize> = parse_part_1(lines[1]);
    let records: Vec<usize> = parse_part_1(lines[0]);
    records
        .iter()
        .zip(times.iter())
        .map(|(time, record)| Race {
            time: *time,
            record: *record,
        })
        .collect()
}

#[trace]
fn parse_input_2(input: &str) -> Race {
    let lines: Vec<&str> = input.split("\n").collect();
    let time: usize = parse_part_2(lines[0]);
    let record: usize = parse_part_2(lines[1]);
    Race { time, record }
}

fn is_win(race: &Race, press_length: usize) -> bool {
    let travel_time = race.time - press_length;
    let distance = press_length * travel_time;
    if press_length < 10 {
        println!("travel_time {travel_time} distance {distance}")
    }
    race.record < distance
}

#[trace]
fn ways_to_win_count(race: &Race) -> usize {
    (0..race.time)
        .filter(|press_length| is_win(race, *press_length))
        .count()
}

fn part_1(races: &Races) -> usize {
    races
        .iter()
        .fold(1, |value, race| value * ways_to_win_count(race))
}

fn main() {
    let filename = env::args().nth(1).unwrap();
    let input = &read_to_string(filename).unwrap();
    let races = parse_input_1(input);
    println!("part 1: {:?}", part_1(&races));
    let race = parse_input_2(input);
    println!("part 2: {:?}", ways_to_win_count(&race))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{Duration, Instant};

    static TEST_INPUT: &str = "\
Time:      7  15   30
Distance:  9  40  200
";

    #[test]
    fn test_part_1() {
        let races = parse_input_1(TEST_INPUT);
        assert_eq!(part_1(&races), 288);
    }

    #[test]
    fn test_part_2() {
        let race = parse_input_2(TEST_INPUT);
        assert_eq!(ways_to_win_count(&race), 71503);
    }
}
