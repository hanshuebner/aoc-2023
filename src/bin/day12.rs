use regex::Regex;
use std::collections::HashSet;
use std::env;
use std::fs::read_to_string;
use itertools::Itertools;

#[macro_use]
extern crate prog1;

// use trace::trace;

// trace::init_depth_var!();

fn fill_in(input: &str, mut permutation: usize) -> String {
    input
        .chars()
        .map(|c| {
            if c == '?' {
                prog1!(
                if (permutation & 1usize) == 1 {
                    '#'
                } else {
                    '.'
                };
                {
                    permutation >>= 1
                })
            } else {
                c
            }
        })
        .collect()
}

fn make_permutations(input: &str) -> HashSet<String> {
    let question_mark_count = input.chars().filter(|c| *c == '?').count();
    (0..2usize.pow(question_mark_count as u32))
        .map(|permutation| fill_in(input, permutation))
        .collect()
}

fn make_regex_string(input: &str) -> String {
    let matcher: String = input.split(',').intersperse("}\\.+#{").collect();
    format!("^\\.*#{{{}}}\\.*$", matcher)
}

fn permutation_match_count(input: &str) -> usize {
    let v: Vec<&str> = input.split(' ').collect();
    let (left, right) = (v[0], v[1]);
    let regex = Regex::new(&make_regex_string(right)).unwrap();
    make_permutations(left).iter().filter(|permutation| regex.find(permutation).is_some()).count()
}

fn part_1(input: &str) -> usize {
    input
        .split('\n')
        .filter(|s| s.len() > 0)
        .map(permutation_match_count)
        .sum()
}

fn main() {
    let filename = env::args().nth(1).unwrap();
    let input = &read_to_string(filename).unwrap();
    println!("part 1: {}", part_1(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT_1_1: &str = "\
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    #[test]
    fn test_part_1_1() {
        assert_eq!(make_permutations("???.###").len(), 8);
        assert_eq!(make_regex_string("1,1,3"), r"^\.*#{1}\.+#{1}\.+#{3}\.*$");
        assert_eq!(permutation_match_count("???.### 1,1,3"), 1);
        assert_eq!(permutation_match_count(".??..??...?##. 1,1,3"), 4);
        assert_eq!(part_1(TEST_INPUT_1_1), 21);
    }
}
