use std::env;
use std::fs::read_to_string;
use std::collections::HashSet;

#[derive(Debug)]
struct MotorSchematic {
    input_data: Vec<Vec<char>>,
    part_numbers: HashSet<usize>,
    part_number_index: Vec<Vec<Option<usize>>>,
}

fn read_motor_schematic(lines: &Vec<String>) -> MotorSchematic
{
    // Read the input data into a two-dimensional "array" that has a one character around the
    // border so that we can safely check adjacency without worrying about the edges.
    let width = lines.iter().next().unwrap().len() + 2;
    let height = lines.len() + 2;

    let mut input_data = vec![vec!['.'; width]; height];

    for (index, line) in lines.iter().enumerate() {
        let row = &mut input_data[index + 1];
        let chars = line.chars().collect::<Vec<char>>();
        row.splice(1..chars.len() + 1, chars);
    }

    // Find the part numbers, building an index from (expanded) input coordinates to part
    // numbers and a set of all part numbers.
    let mut part_number_index = vec![vec![None; width]; height];
    let mut part_numbers: HashSet<usize> = HashSet::new();

    for (row, row_data) in input_data.iter().enumerate() {
        let mut number_start_col = 0;
        let mut current_number: usize = 0;
        let mut in_number = false;
        let mut is_part_number = false;
        for (col, c) in row_data.iter().enumerate() {
            if c.is_digit(10) {
                if !in_number {
                    in_number = true;
                    number_start_col = col;
                }
                current_number = current_number * 10 + (*c as usize - '0' as usize);
                is_part_number = is_part_number || has_adjacent(&input_data, row, col, is_symbol)
            } else {
                if in_number {
                    if is_part_number {
                        part_numbers.insert(current_number);
                        for index_col in number_start_col..col {
                            part_number_index[row][index_col] = Some(current_number);
                        }
                    }
                    current_number = 0;
                    in_number = false;
                    is_part_number = false;
                }
            }
        }
    }

    MotorSchematic {
        input_data,
        part_number_index,
        part_numbers,
    }
}

fn is_symbol(c: char) -> bool {
    !c.is_digit(10) && c != '.'
}

fn has_adjacent<F>(
    input_data: &Vec<Vec<char>>,
    row: usize,
    col: usize,
    predicate: F,
) -> bool
    where
        F: Fn(char) -> bool,
{
    assert!(input_data[row][col].is_digit(10));
    row < input_data.len() - 1
        && col < input_data[0].len() - 1
        && (predicate(input_data[row - 1][col])
        || predicate(input_data[row - 1][col - 1])
        || predicate(input_data[row][col - 1])
        || predicate(input_data[row + 1][col - 1])
        || predicate(input_data[row + 1][col])
        || predicate(input_data[row + 1][col + 1])
        || predicate(input_data[row][col + 1])
        || predicate(input_data[row - 1][col + 1]))
}

fn adjacent_part_numbers(schematic: &MotorSchematic, row: usize, col: usize) -> HashSet<usize> {
    let mut result: HashSet<usize> = HashSet::new();
    let index = &schematic.part_number_index;
    for candidate in [
        index[row - 1][col],
        index[row - 1][col - 1],
        index[row][col - 1],
        index[row + 1][col - 1],
        index[row + 1][col],
        index[row + 1][col + 1],
        index[row][col + 1],
        index[row - 1][col + 1],
    ] {
        if let Some(part_number) = candidate {
            result.insert(part_number);
        }
    }
    return result;
}

fn find_and_sum_part_numbers(schematic: &MotorSchematic) -> usize
{
    schematic.part_numbers.iter().sum()
}

fn find_and_sum_gear_ratios(schematic: &MotorSchematic) -> usize
{
    let mut sum: usize = 0;
    for (row, row_data) in schematic.input_data.iter().enumerate() {
        for (col, c) in row_data.iter().enumerate() {
            if *c == '*' {
                let part_numbers = adjacent_part_numbers(&schematic, row, col);
                if part_numbers.len() == 2 {
                    let v : Vec<&usize> = part_numbers.iter().collect();
                    sum = sum + v[0] * v[1]
                }
            }
        }
    }
    sum
}

fn main() {
    let filename = env::args().nth(1).unwrap();
    let lines = read_to_string(filename).unwrap().lines().map(String::from).collect();
    let schematic = read_motor_schematic(&lines);
    let sum_1 = find_and_sum_part_numbers(&schematic);
    println!("part 1 sum: {sum_1}");
    let sum_2 = find_and_sum_gear_ratios(&schematic);
    println!("part 2 sum: {sum_2}")
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..";
    static TEST_INPUT_WIDTH: usize = 10;
    static TEST_INPUT_HEIGHT: usize = 10;

    #[test]
    fn test_part_1() {
        let lines = TEST_INPUT.lines().map(String::from).collect();
        let schematic = read_motor_schematic(&lines);
        assert_eq!(schematic.input_data[0].len(), TEST_INPUT_WIDTH + 2);
        assert_eq!(schematic.input_data.len(), TEST_INPUT_HEIGHT + 2);
        assert_eq!(find_and_sum_part_numbers(&schematic), 4361);
        assert_eq!(find_and_sum_gear_ratios(&schematic), 467835)
    }
}
