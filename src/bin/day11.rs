use itertools::Itertools;
use std::collections::HashSet;
use std::env;
use std::fs::read_to_string;
// use trace::trace;

// trace::init_depth_var!();

#[derive(Debug, Copy, Clone)]
struct Galaxy {
    number: usize,
    x: usize,
    y: usize,
}

type Universe = Vec<Vec<Option<Galaxy>>>;

fn repeat_elements<T: Copy>(input: &Vec<Option<T>>, repeats: &HashSet<usize>) -> Vec<Option<T>> {
    input
        .into_iter()
        .enumerate()
        .flat_map(|(index, value)| {
            if repeats.contains(&index) {
                vec![None, None]
            } else {
                vec![*value]
            }
        })
        .collect()
}

fn make_universe(string: &str) -> Universe {
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
    let mut empty_cols: HashSet<usize> = HashSet::new();
    for col in 0..raw[0].len() {
        if raw.iter().all(|row| row[col].is_none()) {
            empty_cols.insert(col);
        }
    }
    let raw_expanded: Vec<Vec<Option<usize>>> = raw
        .iter()
        .map(|row| repeat_elements(&row, &empty_cols))
        .map(|row| {
            if row.iter().all(|cell| cell.is_none()) {
                vec![row.clone(), row]
            } else {
                vec![row]
            }
        })
        .flatten()
        .collect();
    raw_expanded.iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, cell)| {
                    cell.map(|galaxy_number| Galaxy {
                        number: galaxy_number,
                        x,
                        y,
                    })
                })
                .collect()
        })
        .collect()
}

fn shortest_path(
    from: &Galaxy,
    to: &Galaxy,
) -> usize {
    from.x.max(to.x) - from.x.min(to.x)
        + from.y.max(to.y) - from.y.min(to.y)
}

fn print_universe(universe: &Universe) {
    for row in universe {
        for galaxy in row {
            if galaxy.is_none() {
                print!(".")
            } else {
                print!("{}", (galaxy.unwrap().number % 10).to_string())
            }
        }
        print!("\n")
    }
}

fn part_1(universe: &Universe) -> usize {
    universe
        .iter()
        .flatten()
        .filter(|galaxy| !galaxy.is_none())
        .map(|galaxy| galaxy.unwrap())
        .combinations(2)
        .map(|pair| shortest_path(&pair[0], &pair[1]))
        .sum()
}

fn main() {
    let filename = env::args().nth(1).unwrap();
    let input = &read_to_string(filename).unwrap();
    let universe = make_universe(input);
    println!("part 1: {:?}", part_1(&universe));
    //    println!("part 2: {:?}", part_2(&map));
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
        let universe = make_universe(TEST_INPUT_1_1);
        print_universe(&universe);
        assert_eq!(part_1(&universe), 374);
    }
}
