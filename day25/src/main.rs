#[allow(unused_imports)]
use std::{cmp::max, cmp::min, collections::HashMap, fs};
use std::{collections::BTreeMap, str::Lines};
// use fancy_regex::Regex;
// use regex::Regex;
// use md5::{Digest, Md5};
// use priority_queue::PriorityQueue;

fn main() {
    let input = include_str!("../inputs/puzzle_input.txt");
    // let input = include_str!("../inputs/test_puzzle_input.txt");
    // println!("{:?}", input);
    println!("Input length: {}", input.len());

    part_one(&input);
}

#[allow(dead_code)]
fn part_one(input: &str) {
    let input = parse_input(input);
    let mut turing_machine = TuringMachine {
        states: input.states,
        tape: Tape::new(),
        cursor_idx: 0,
        cur_state: input.start_state,
    };
    for _ in 0..input.num_steps {
        turing_machine.step();
    }
    println!("After {} steps, there were {} ones", input.num_steps, turing_machine.tape.count_ones());
}

fn parse_input(input: &str) -> Input {
    let mut lines: Lines = input.lines();
    let start_state = lines
        .next()
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .chars()
        .next()
        .unwrap();
    let num_steps = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(5)
        .next()
        .unwrap()
        .parse::<usize>()
        .unwrap();

    let mut states = BTreeMap::new();
    loop {
        if let Some(state) = parse_state(&mut lines) {
            states.insert(state.id, state);
        } else {
            break;
        }
    }

    dbg!(&states);
    Input {
        start_state,
        num_steps,
        states,
    }
}

fn parse_state(lines: &mut Lines) -> Option<State> {
    if let None = lines.next() {
        return None;
    }
    let id = lines
        .next()
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .chars()
        .next()
        .unwrap();
    let zero_action = parse_action(lines);
    let one_action = parse_action(lines);
    Some(State {
        id,
        zero_action,
        one_action,
    })
}

fn parse_action(lines: &mut Lines) -> Action {
    let write_val = lines
        .skip(1)
        .next()
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .trim_matches('.')
        .parse::<u8>()
        .unwrap();
    let move_dir = match lines.next().unwrap().split_whitespace().last().unwrap() {
        "right." => Direction::Right,
        "left." => Direction::Left,
        other => panic!("Can't parse {other} to direction"),
    };
    let next_state = lines
        .next()
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .chars()
        .next()
        .unwrap();
    Action {
        write_val,
        move_dir,
        next_state,
    }
}

#[derive(Debug, Clone)]
struct Input {
    start_state: char,
    num_steps: usize,
    states: BTreeMap<char, State>,
}

#[derive(Debug, Clone)]
struct Tape(BTreeMap<i32, u8>);

impl Tape {
    fn new() -> Tape {
        Tape(BTreeMap::new())
    }

    #[allow(dead_code)]
    fn read(&mut self, idx: i32) -> u8 {
        *self.0.entry(idx).or_insert(0)
    }

    fn write(&mut self, idx: i32, value: u8) {
        let entry = self.0.entry(idx).or_insert(0);
        *entry = value;
    }

    fn count_ones(&self) -> usize {
        self.0.iter().filter(|(_, &b)| b == 1).count()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, Clone, PartialEq)]
struct State {
    id: char,
    zero_action: Action,
    one_action: Action,
}

#[derive(Debug, Clone, PartialEq)]
struct Action {
    write_val: u8,
    move_dir: Direction,
    next_state: char,
}

struct TuringMachine {
    tape: Tape,
    cursor_idx: i32,
    states: BTreeMap<char, State>,
    cur_state: char,
}

impl TuringMachine {
    fn step(&mut self) {
        let action = match self.read() {
            0 => &self.states[&self.cur_state].zero_action,
            1 => &self.states[&self.cur_state].one_action,
            _ => panic!("Can only have values of 0 or 1 in tape"),
        };
        self.tape.write(self.cursor_idx, action.write_val);
        self.cursor_idx += match action.move_dir {
            Direction::Left => -1,
            Direction::Right => 1,
        };
        self.cur_state = action.next_state;
    }

    fn read(&mut self) -> u8 {
        *self.tape.0.entry(self.cursor_idx).or_insert(0)
    }

    #[allow(dead_code)]
    fn write(&mut self, value: u8) {
        let entry = self.tape.0.entry(self.cursor_idx).or_insert(0);
        *entry = value;
    }

    #[allow(dead_code)]
    fn move_cursor(&mut self, dir: &Direction) {
        self.cursor_idx += match dir {
            Direction::Left => -1,
            Direction::Right => 1,
        }
    }
}
