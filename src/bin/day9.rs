use std::env;
use std::fs::read_to_string;
// use trace::trace;

// trace::init_depth_var!();

type Sequence = Vec<isize>;

fn parse_numbers(input: &str) -> Sequence {
    input
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}

fn make_diffs_sequence(s: &Sequence) -> Sequence {
    s.iter().zip(s.iter().skip(1)).map(|(a, b)| b - a).collect()
}

fn solve_sequence_1(acc: Vec<Sequence>, this: &Sequence) -> isize {
    if this.iter().all(|x| *x == 0) {
        acc.iter().rev().fold(0, |acc, s| acc + s.last().unwrap())
    } else {
        solve_sequence_1(
            acc.iter().chain([this]).cloned().collect(),
            &make_diffs_sequence(this),
        )
    }
}

fn part_1(input: &str) -> isize {
    input
        .split("\n")
        .filter(|s| s.len() > 0)
        .map(parse_numbers)
        .map(|s| solve_sequence_1(vec![], &s))
        .sum()
}

fn part_2(input: &str) -> isize {
    input
        .split("\n")
        .filter(|s| s.len() > 0)
        .map(parse_numbers)
        .map(|s| s.into_iter().rev().collect())
        .map(|s| solve_sequence_1(vec![], &s))
        .sum()
}

fn main() {
    let filename = env::args().nth(1).unwrap();
    let input = &read_to_string(filename).unwrap();
    println!("part 1: {:?}", part_1(input));
    println!("part 2: {:?}", part_2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "\
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(TEST_INPUT), 114);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(TEST_INPUT), 2)
    }
}
