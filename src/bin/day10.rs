use crate::Direction::{East, North, South, West};
use crate::TileKind::{Animal, Empty, Pipe};
use std::env;
use std::fs::read_to_string;
// use trace::trace;

// trace::init_depth_var!();

#[derive(Debug, PartialEq, Copy, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, PartialEq, Clone)]
enum TileKind {
    Animal,
    Empty,
    Pipe(Direction, Direction),
}

#[derive(Debug, PartialEq, Clone)]
struct Tile {
    x: usize,
    y: usize,
    kind: TileKind,
    is_loop_border: bool,
}

type Map = Vec<Vec<Tile>>;

fn char_to_tile_kind(c: char) -> TileKind {
    match c {
        'S' => Animal,
        '.' => Empty,
        '|' => Pipe(North, South),
        '-' => Pipe(East, West),
        'F' => Pipe(South, East),
        '7' => Pipe(South, West),
        'L' => Pipe(North, East),
        'J' => Pipe(North, West),
        _ => panic!("unknown character {:?}", c),
    }
}

fn tile_kind_to_char(kind: &TileKind) -> char {
    match kind {
        Animal => '▣',
        Empty => '◦',
        Pipe(North, South) => '│',
        Pipe(East, West) => '─',
        Pipe(South, East) => '┌',
        Pipe(South, West) => '┐',
        Pipe(North, East) => '└',
        Pipe(North, West) => '┘',
        _ => panic!("unexpected tile kind {:?}", kind),
    }
}

fn next_direction(entered_from: &Direction, tile_kind: &TileKind) -> Direction {
    match (entered_from, tile_kind) {
        (North, Pipe(South, output)) | (North, Pipe(output, South)) => *output,
        (East, Pipe(West, output)) | (East, Pipe(output, West)) => *output,
        (South, Pipe(North, output)) | (South, Pipe(output, North)) => *output,
        (West, Pipe(East, output)) | (West, Pipe(output, East)) => *output,
        _ => panic!("pipe flow interrupted"),
    }
}

fn new_coords(x: usize, y: usize, direction: &Direction) -> (usize, usize) {
    let (new_x, new_y) = match direction {
        North => (x, y - 1),
        East => (x + 1, y),
        South => (x, y + 1),
        West => (x - 1, y),
    };

    (new_x, new_y)
}

fn find_animal(map: &Map) -> (usize, usize) {
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x].kind == Animal {
                return (x, y);
            }
        }
    }
    panic!("Animal not found")
}

fn find_first_step(x: usize, y: usize, map: &Map) -> Direction {
    if x > 0 && matches!(map[y][x - 1].kind, Pipe(East, _) | Pipe(_, East)) {
        West
    } else if x < map[0].len() - 1 && matches!(map[y][x + 1].kind, Pipe(West, _) | Pipe(_, West)) {
        East
    } else if y > 0 && matches!(map[y - 1][x].kind, Pipe(North, _) | Pipe(_, North)) {
        South
    } else if y < map.len() - 1 && matches!(map[y + 1][x].kind, Pipe(South, _) | Pipe(_, South)) {
        North
    } else {
        panic!("No first tile found");
    }
}

fn animal_tile_kind(x: usize, y: usize, map: &Map) -> TileKind {
    let connects = (
        if x > 0 && matches!(map[y][x - 1].kind, Pipe(East, _) | Pipe(_, East)) {
            Some(West)
        } else {
            None
        },
        if x < map[0].len() - 1 && matches!(map[y][x + 1].kind, Pipe(West, _) | Pipe(_, West)) {
            Some(East)
        } else {
            None
        },
        if y < map.len() - 1 && matches!(map[y + 1][x].kind, Pipe(North, _) | Pipe(_, North)) {
            Some(South)
        } else {
            None
        },
        if y > 0 && matches!(map[y - 1][x].kind, Pipe(South, _) | Pipe(_, South)) {
            Some(North)
        } else {
            None
        },
    );
    match connects {
        (Some(West), _, Some(South), _) => Pipe(South, West),
        (Some(West), Some(East), _, _) => Pipe(East, West),
        (Some(West), _, _, Some(North)) => Pipe(North, West),
        (_, Some(East), Some(South), _) => Pipe(South, East),
        (_, Some(East), _, Some(North)) => Pipe(North, East),
        (_, _, Some(South), Some(North)) => Pipe(North, South),
        _ => panic!("Cannot determine type of animal tile")
    }
}

fn parse_input(input: &str) -> Map {
    input
        .split("\n")
        .filter(|s| s.len() > 0)
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| Tile {
                    x,
                    y,
                    kind: char_to_tile_kind(c),
                    is_loop_border: false,
                })
                .collect()
        })
        .collect()
}

fn make_map(input: &str) -> Map {
    let mut map = parse_input(input);
    let (animal_x, animal_y) = find_animal(&map);
    map[animal_y][animal_x].kind = animal_tile_kind(animal_x, animal_y, &map);
    let mut direction = find_first_step(animal_x, animal_y, &map);
    let mut current_tile: &mut Tile = &mut map[animal_y][animal_x];
    loop {
        current_tile.is_loop_border = true;
        let (new_x, new_y) = new_coords(current_tile.x, current_tile.y, &direction);
        current_tile = &mut map[new_y][new_x];
        if current_tile.x == animal_x && current_tile.y == animal_y {
            break;
        }
        direction = next_direction(&direction, &current_tile.kind);
    }
    map
}

fn part_1(map: &Map) -> usize {
    let count = map
        .iter()
        .flat_map(|tile| tile)
        .filter(|tile| tile.is_loop_border)
        .count();
    (count + 1) / 2
}

fn part_2(map: &Map) -> usize {
    let mut count = 0;
    for row in map.iter().skip(1) {
        let mut inside = false;
        for tile in row {
            if tile.is_loop_border {
                match tile.kind {
                    Pipe(South, _) | Pipe(_, South) => {
                        inside = !inside;
                    }
                    _ => (),
                }
            } else if inside {
                count += 1
            }
        }
    }
    count
}

fn print_map(map: &Map) {
    print!("  ");
    for i in 0..map[0].len() {
        print!("{}", i % 10)
    }
    println!("");
    for row in map {
        print!("{:3} ", row[0].y);
        for tile in row {
            print!(
                "{}",
                if tile.is_loop_border {
                    tile_kind_to_char(&tile.kind)
                } else {
                    '.'
                }
            );
        }
        println!("");
    }
}

fn main() {
    let filename = env::args().nth(1).unwrap();
    let input = &read_to_string(filename).unwrap();
    let map = make_map(input);
    print_map(&map);
    println!("part 1: {:?}", part_1(&map));
    println!("part 2: {:?}", part_2(&map));
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT_1_1: &str = "\
7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";

    #[test]
    fn test_part_1_1() {
        let map = make_map(TEST_INPUT_1_1);
        assert_eq!(part_1(&map), 8);
    }

    static TEST_INPUT_1_2: &str = "\
-L|F7
7S-7|
L|7||
-L-J|
L|-JF";

    #[test]
    fn test_part_1_2() {
        let map = make_map(TEST_INPUT_1_2);
        assert_eq!(part_1(&map), 4);
    }

    static TEST_INPUT_2_1: &str = "\
...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";

    #[test]
    fn test_part_2_1() {
        let map = make_map(TEST_INPUT_2_1);
        print_map(&map);
        assert_eq!(part_2(&map), 4);
    }

    static TEST_INPUT_2_2: &str = "\
.F7FSF7F7F7F7F7F---7
.|LJ||||||||||||F--J
.L-7LJLJ||||||LJL-7.
F--JF--7||LJLJ.F7FJ.
L---JF-JLJ....FJLJ..
...F-JF---7...L7....
..FJF7L7F-JF7..L---7
..L-JL7||F7|L7F-7F7|
.....FJ|||||FJL7||LJ
.....L-JLJLJL--JLJ..";

    #[test]
    fn test_part_2_2() {
        let map = make_map(TEST_INPUT_2_2);
        print_map(&map);
        assert_eq!(part_2(&map), 10);
    }
}
