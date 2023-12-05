use std::env;
use std::fs::read_to_string;
use regex::Regex;


#[macro_use]
extern crate partial_application;

#[derive(Debug)]
struct Mapper {
    #[allow(dead_code)]
    from: String,
    #[allow(dead_code)]
    to: String,
    ranges: Vec<(usize, usize, usize)>,
}

fn map_value(mapper: &Mapper, value: &usize) -> usize {
    for range in mapper.ranges.iter() {
        let (dest, src, len) = range;
        if *value >= *src && *value < *src + *len {
            return *dest + *value - *src
        }
    }
    *value
}

fn parse_mapper(string: &String) -> Mapper {
    let lines: Vec<String> = string.split("\n").map(String::from).collect();
    let regex = Regex::new(r"^(.*)-to-(.*) map:").unwrap();
    let (_, [from, to]) = regex.captures(lines[0].as_str()).unwrap().extract();
    Mapper {
        from: from.to_string(),
        to: to.to_string(),
        ranges: lines
            .iter()
            .skip(1)
            .filter(|s| !s.is_empty())
            .map( | line| {
                let mut numbers = line.split_whitespace().map( | s | s.parse::<usize > ().unwrap());
                (numbers.next().unwrap(),
                 numbers.next().unwrap(),
                 numbers.next().unwrap())
            })
            .collect(),
    }
}

#[derive(Debug)]
struct Input {
    seeds: Vec<usize>,
    mappers: Vec<Mapper>,
}

fn parse_numbers(input: &str) -> Vec<usize> {
    input
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}

fn parse_input(string: &str) -> Input {
    let chunks: Vec<String> = string.split("\n\n").map(String::from).collect();
    let regex = Regex::new(r"^seeds: ([\d ]+)$").unwrap();

    Input {
        seeds: parse_numbers(&regex.captures(&chunks[0]).unwrap()[1]),
        mappers: chunks.iter().skip(1).map(parse_mapper).collect(),
    }
}

fn seed_to_location(input: &Input, seed: &usize) -> usize {
    input.mappers.iter()
        .fold(*seed, |seed, mapper| map_value(&mapper, &seed))
}


fn part_1(input: &Input) -> usize {
    input.seeds.iter().map(partial!(seed_to_location => input, _))
        .min().unwrap()
}

fn part_2(input: &Input) -> usize {
    input.seeds
        .chunks(2)
        .flat_map(|pair| {
            let start = pair[0];
            let length = pair[1];
            (start..start + length).collect::<Vec<usize>>()
        })
        .map(|seed| seed_to_location(input, &seed))
        .min()
        .unwrap()
}

fn main() {
    let filename = env::args().nth(1).unwrap();
    let input = parse_input(&read_to_string(filename).unwrap());
    println!("part 1: {:?}", part_1(&input));
    println!("part 2: {:?}", part_2(&input));
}

#[cfg(test)]
mod tests {
    use std::time::{Duration, Instant};
    use super::*;

    static TEST_INPUT: &str = "\
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";

    #[test]
    fn test_part_1() {
        let input = parse_input(TEST_INPUT);
        assert_eq!(part_1(&input), 35);
        assert_eq!(part_2(&input), 46);
    }

    #[test]
    fn test_map_speed() {
        let mapper = Mapper { from: "foo".to_string(), to: "bar".to_string(), ranges: vec![(0, 1, 1000000)]};
        let mut result: usize = 0;
        let start = Instant::now();
        for i in 0..100000000 {
            result += map_value(&mapper, &i);
        }
        let end = Instant::now();
        println!("done: {:?} -> {:?}", result, end - start);
        assert!(end - start < Duration::from_millis(5000));
    }
}
