use std::env;
use std::fs::read_to_string;
use regex::Regex;
use std::collections::HashMap;
use memoize::memoize;

const DIGIT_NAMES: [&str; 10] = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

#[memoize]
fn digit_encoding() -> (Regex, HashMap<String, usize>) {
    let mut map: HashMap<String, usize> = HashMap::new();
    let mut regex_parts = Vec::new();

    for (index, name) in DIGIT_NAMES.iter().enumerate() {
        map.insert(name.to_string(), index);
        map.insert(index.to_string(), index);

        regex_parts.push(name.to_string());
        regex_parts.push(index.to_string());
    }

    let regex = format!("^({})", regex_parts.join("|"));
    println!("regex: {}", regex);

    (Regex::new(&regex).unwrap(), map)
}

fn extract_calibration_value(string: String) -> usize {
    let (regex, map) = digit_encoding();

    let mut found_digit = false;
    let mut digit1: usize = 0;
    let mut digit2: usize = 0;

    let mut s: &str = &string;
    while !s.is_empty() {
        if let Some(mat) = regex.find(s) {
            let value = *map.get(mat.as_str()).unwrap();
            if !found_digit {
                digit1 = value;
                digit2 = value;
                found_digit = true;
            } else {
                digit2 = value;
            }
        }
        s = &s[1..];
    }
    if !found_digit {
        panic!("could not find digit in input line \"{string}\"")
    }

    let value_string = format!("{digit1}{digit2}");
    println!("value: {value_string}");
    value_string.parse::<usize>().unwrap()
}

fn read_lines(filename: &str) -> usize {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .map(extract_calibration_value)
        .sum()
}

fn main() {
    let filename = env::args().nth(1).unwrap();
    let sum = read_lines(&filename);
    println!("sum: {sum}");
}
