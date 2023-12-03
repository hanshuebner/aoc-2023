use std::env;
use std::fs::read_to_string;

type MotorSchematic = Vec<Vec<char>>;

fn read_motor_schematic(lines: &Vec<String>) -> MotorSchematic
{
    let width = lines.iter().next().unwrap().len() + 2;
    let height = lines.len() + 2;

    let mut motor_schematic = vec![vec!['.'; width]; height];

    for (index, line) in lines.iter().enumerate() {
        let row = &mut motor_schematic[index + 1];
        let chars = line.chars().collect::<Vec<char>>();
        row.splice(1..chars.len() + 1, chars);
    }
    motor_schematic
}

fn is_symbol(c: char) -> bool {
    !c.is_digit(10) && c != '.'
}

fn has_adjacent_symbol(schematic: &MotorSchematic, row: usize, col: usize) -> bool {
    assert!(schematic[row][col].is_digit(10));
    row < schematic.len() - 1
        && col < schematic[0].len() - 1
        && (is_symbol(schematic[row - 1][col])
            || is_symbol(schematic[row - 1][col - 1])
            || is_symbol(schematic[row][col - 1])
            || is_symbol(schematic[row + 1][col - 1])
            || is_symbol(schematic[row + 1][col])
            || is_symbol(schematic[row + 1][col + 1])
            || is_symbol(schematic[row][col + 1])
            || is_symbol(schematic[row - 1][col + 1]))
}

fn find_and_sum_part_numbers(lines: &Vec<String>) -> usize
{
    let schematic = read_motor_schematic(lines);
    let mut sum: usize = 0;

    for (row, row_data) in schematic.iter().enumerate() {
        let mut current_number: usize = 0;
        let mut in_number = false;
        let mut is_part_number = false;
        for (col, c) in row_data.iter().enumerate() {
            if c.is_digit(10) {
                in_number = true;
                current_number = current_number * 10 + (*c as usize - '0' as usize);
                is_part_number = is_part_number || has_adjacent_symbol(&schematic, row, col)
            } else {
                if in_number {
                    if is_part_number {
                        sum = sum + current_number
                    }
                    current_number = 0;
                    in_number = false;
                    is_part_number = false;
                }
            }
        }
    }
    sum
}

fn main() {
    let filename = env::args().nth(1).unwrap();
    let lines = read_to_string(filename).unwrap().lines().map(String::from).collect();
    let sum = find_and_sum_part_numbers(&lines);
    println!("sum: {sum}");
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
        assert_eq!(schematic[0].len(), TEST_INPUT_WIDTH + 2);
        assert_eq!(schematic.len(), TEST_INPUT_HEIGHT + 2);
        assert_eq!(find_and_sum_part_numbers(&lines), 4361);
    }
}
