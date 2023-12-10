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

#[derive(Debug, PartialEq)]
enum TileKind {
    Animal,
    Empty,
    Pipe(Direction, Direction),
}

#[derive(Debug, PartialEq)]
struct Tile {
    x: usize,
    y: usize,
    kind: TileKind,
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

fn next_direction<'a>(entered_from: &Direction, tile_kind: &'a TileKind) -> Direction {
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

fn find_animal(map: &Map) -> &Tile {
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x].kind == Animal {
                return &map[y][x]
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

fn parse_input(input: &str) -> Map {
    input
        .split("\n")
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| Tile {
                    x,
                    y,
                    kind: char_to_tile_kind(c),
                })
                .collect()
        })
        .collect()
}

fn part_1(map: &Map) -> usize {
    let animal_tile = find_animal(map);
    let mut direction = find_first_step(animal_tile.x, animal_tile.y, map);
    let mut current_tile = animal_tile;
    let mut count = 1;
    loop {
        let (new_x, new_y) = new_coords(current_tile.x, current_tile.y, &direction);
        current_tile = &map[new_y][new_x];
        if current_tile == animal_tile {
            break;
        }
        direction = next_direction(&direction, &current_tile.kind);
        count += 1;
    }
    (count + 1) / 2
}

fn main() {
    let filename = env::args().nth(1).unwrap();
    let input = &read_to_string(filename).unwrap();
    let map = parse_input(input);
    println!("part 1: {:?}", part_1(&map));
//    println!("part 2: {:?}", part_2(input));
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
    fn test_find_animal_1_1() {
        let map = parse_input(TEST_INPUT_1_1);
        assert_eq!(find_animal(&map), &map[2][0]);
        assert_eq!(find_first_step(0, 2, &map), East);
    }

    #[test]
    fn test_part_1_1() {
        let map = parse_input(TEST_INPUT_1_1);
        assert_eq!(part_1(&map), 8);
    }

    static TEST_INPUT_1_2: &str = "\
-L|F7
7S-7|
L|7||
-L-J|
L|-JF";

    #[test]
    fn test_find_animal_1_2() {
        let map = parse_input(TEST_INPUT_1_2);
        assert_eq!(find_animal(&map), &map[1][1]);
        assert_eq!(find_first_step(1, 1, &map), East);
    }

    #[test]
    fn test_part_1_2() {
        let map = parse_input(TEST_INPUT_1_2);
        assert_eq!(part_1(&map), 4);
    }
}
