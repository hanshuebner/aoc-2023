use regex::Regex;
use std::collections::{HashMap};
use std::env;
use std::fs::read_to_string;
use num_integer::lcm;
// use trace::trace;

// trace::init_depth_var!();

#[derive(Debug, Clone)]
enum Step {
    Left,
    Right,
}

#[derive(Debug, Clone)]
struct Stepper<'a> {
    next: usize,
    steps: Vec<Step>,
    nodes: &'a NodeMap,
    current_node: &'a Node,
    at_end: NodePredicate,
}

fn get_next<'a>(stepper: &'a mut Stepper) -> &'a Step {
    let step: &Step = &stepper.steps[stepper.next];
    stepper.next = (stepper.next + 1) % stepper.steps.len();
    step
}

fn next_step<'a>(stepper: &mut Stepper) {
    stepper.current_node = match get_next(stepper) {
        Step::Left => &stepper.nodes[&stepper.current_node.left],
        Step::Right => &stepper.nodes[&stepper.current_node.right],
    }
}

fn at_end(stepper: &Stepper) -> bool {
    (stepper.at_end)(stepper.current_node)
}

fn steps_to_end(stepper: &mut Stepper) -> usize {
    let mut count = 0;
    while !at_end(stepper) {
        next_step(stepper);
        count += 1;
    }
    count
}

fn char_to_step(c: char) -> Step {
    match c {
        'L' => Step::Left,
        'R' => Step::Right,
        _ => panic!("Invalid step character {:?}", c),
    }
}

fn make_stepper<'a>(
    steps_input: &str,
    nodes: &'a NodeMap,
    at_end: NodePredicate,
    current_node: &'a Node,
) -> Stepper<'a> {
    let steps = steps_input.chars().map(char_to_step).collect();
    Stepper {
        next: 0,
        steps,
        nodes,
        current_node,
        at_end,
    }
}

#[derive(Debug)]
struct Node {
    name: String,
    left: String,
    right: String,
}

type NodePredicate = fn(&Node) -> bool;

type NodeMap = HashMap<String, Node>;

fn parse_node(input: &str) -> Node {
    let regex = Regex::new(r"^(.*) = \((.*), (.*)\)$").unwrap();
    let (_, [name, left, right]) = regex.captures(input).unwrap().extract();
    Node {
        name: name.to_string(),
        left: left.to_string(),
        right: right.to_string(),
    }
}

fn parse_nodes(input: &str) -> NodeMap {
    input
        .split("\n")
        .filter(|s| s.len() > 0)
        .map(parse_node)
        .map(|node| (node.name.clone(), node))
        .collect()
}

fn parse_input(input: &str) -> (String, NodeMap) {
    let splits: Vec<&str> = input.split("\n\n").collect();
    (splits[0].to_string(), parse_nodes(splits[1]))
}

fn count_steps_1(input: &str) -> usize {
    let (steps_string, nodes): (String, NodeMap) = parse_input(input);
    let mut stepper = make_stepper(
        &steps_string,
        &nodes,
        |node| node.name == "ZZZ",
        &nodes["AAA"],
    );
    steps_to_end(&mut stepper)
}

fn count_steps_2(input: &str) -> usize {
    let (steps_string, nodes): (String, NodeMap) = parse_input(input);
    nodes
        .values()
        .filter(|node| node.name.ends_with('A'))
        .map(|node| {
            let mut stepper = make_stepper(&steps_string, &nodes, |node| node.name.ends_with('Z'), node);
            steps_to_end(&mut stepper)
        })
        .fold(1, |acc, x| lcm(acc, x))
}

fn main() {
    let filename = env::args().nth(1).unwrap();
    let input = &read_to_string(filename).unwrap();
    println!("part 1: {:?}", count_steps_1(input));
    println!("part 2: {:?}", count_steps_2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT_1_1: &str = "\
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    static TEST_INPUT_1_2: &str = "\
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    #[test]
    fn test_part_1() {
        assert_eq!(count_steps_1(TEST_INPUT_1_1), 2);
        assert_eq!(count_steps_1(TEST_INPUT_1_2), 6);
    }

    static TEST_INPUT_2: &str = "\
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
";

    #[test]
    fn test_part_2() {
        assert_eq!(count_steps_2(TEST_INPUT_2), 6);
    }

    #[test]
    fn test_closure() {
        let mut x = 0;
        let _z: Vec<bool> = [1,2,3].iter().map(|y| { x += y; true }).collect();
        assert_eq!(x, 6);
    }
}
