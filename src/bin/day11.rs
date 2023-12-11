use itertools::Itertools;
use std::collections::HashSet;
use std::env;
use std::fs::read_to_string;
// use trace::trace;

// trace::init_depth_var!();

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Galaxy {
    number: usize,
    x: usize,
    y: usize,
}

type Universe = HashSet<Galaxy>;

fn make_universe(string: &str, expand_factor: usize) -> Universe {
    let mut galaxy_number = 0;
    let raw: Vec<Vec<Option<usize>>> = string
        .split('\n')
        .filter(|s| s.len() > 1)
        .map(|line| {
            line.chars()
                .map(|c| {
                    if c == '#' {
                        galaxy_number += 1;
                        Some(galaxy_number)
                    } else {
                        None
                    }
                })
                .collect()
        })
        .collect();
    let empty_rows: HashSet<usize> = raw
        .iter()
        .enumerate()
        .filter(|(_, row)| row.iter().all(|cell| cell.is_none()))
        .map(|(y, _)| y)
        .collect();
    let empty_cols: HashSet<usize> = raw[0]
        .iter()
        .enumerate()
        .filter(|(x, _)| raw.iter().all(|row| row[*x].is_none()))
        .map(|(x, _)| x)
        .collect();
    let mut expanded_y = 0;
    let mut universe = Universe::new();
    for y in 0..raw.len() {
        if empty_rows.contains(&y) {
            expanded_y += expand_factor - 1
        } else {
            let mut expanded_x = 0;
            for x in 0..raw[0].len() {
                if empty_cols.contains(&x) {
                    expanded_x += expand_factor - 1
                } else if raw[y][x].is_some() {
                    universe.insert(Galaxy {
                        number: raw[y][x].unwrap(),
                        x: expanded_x,
                        y: expanded_y,
                    });
                }
                expanded_x += 1;
            }
        }
        expanded_y += 1;
    }
    universe
}

fn shortest_path(from: &Galaxy, to: &Galaxy) -> usize {
    from.x.max(to.x) - from.x.min(to.x) + from.y.max(to.y) - from.y.min(to.y)
}

fn shortest_paths_sum(universe: &Universe) -> usize {
    universe
        .iter()
        .combinations(2)
        .map(|pair| shortest_path(&pair[0], &pair[1]))
        .sum()
}

fn main() {
    let filename = env::args().nth(1).unwrap();
    let input = &read_to_string(filename).unwrap();
    println!("part 1: {:?}", shortest_paths_sum(&make_universe(input, 1)));
    println!("part 2: {:?}", shortest_paths_sum(&make_universe(input, 1000000)));
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT_1_1: &str = "\
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn test_part_1_1() {
        assert_eq!(shortest_paths_sum(&make_universe(TEST_INPUT_1_1, 2)), 374);
        assert_eq!(shortest_paths_sum(&make_universe(TEST_INPUT_1_1, 10)), 1030);
        assert_eq!(shortest_paths_sum(&make_universe(TEST_INPUT_1_1, 100)), 8410);
    }
}
