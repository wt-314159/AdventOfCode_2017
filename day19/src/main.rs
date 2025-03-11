#[allow(unused_imports)]
use std::{cmp::max, cmp::min, collections::HashMap, fs};
// use fancy_regex::Regex;
// use regex::Regex;
// use md5::{Digest, Md5};
// use priority_queue::PriorityQueue;

fn main() {
    let input = include_str!("../inputs/puzzle_input.txt");
    // let input = include_str!("../inputs/test_puzzle_input.txt");
    // println!("{:?}", input);
    println!("Input length: {}", input.len());
    part_two(&input);
}

#[allow(dead_code)]
fn part_one(input: &str) {
    let diagram = Diagram::new(input);
    print!("letters: ");
    let letters = diagram
        .walk_path()
        .filter(|c| c.is_alphabetic())
        .map(|l| print!("{l}"))
        .count();
    println!("Encountered {letters} letters");
}

#[allow(dead_code)]
fn part_two(input: &str) {
    let diagram = Diagram::new(input);
    let steps = diagram.walk_path().count();
    println!("Took {steps} steps");
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Diagram {
    map: Vec<Vec<char>>,
}

impl Diagram {
    fn new(input: &str) -> Diagram {
        Diagram {
            map: input.lines().map(|l| l.chars().collect()).collect(),
        }
    }

    fn walk_path(self) -> impl Iterator<Item = char> {
        Walker::new(self)
    }
}

struct Walker {
    diagram: Diagram,
    position: (usize, usize),
    direction: Direction,
    at_end: bool,
}

impl Walker {
    fn new(diagram: Diagram) -> Self {
        let start = diagram.map[0].iter().position(|&c| c == '|').unwrap();
        Self {
            diagram,
            position: (0, start),
            direction: Direction::Down,
            at_end: false,
        }
    }

    fn find_char(&self, (row, col): (usize, usize)) -> Option<char> {
        // Check if next position is out of bounds
        if row >= self.diagram.map.len() || col >= self.diagram.map[0].len() {
            return None;
        }
        Some(self.diagram.map[row][col])
    }

    fn find_neighbour_position(&self, direction: &Direction) -> (usize, usize) {
        let (row, col) = self.position;
        match direction {
            Direction::Up => (row - 1, col),
            Direction::Down => (row + 1, col),
            Direction::Left => (row, col - 1),
            Direction::Right => (row, col + 1),
        }
    }

    fn find_next_direction(&self) -> Option<Direction> {
        let mut next = None;
        for pos_dir in self.find_other_directions() {
            let (row, col) = self.find_neighbour_position(pos_dir);
            if let Some(c) = self.find_char((row, col)) {
                if matches!(c, '|' | '-') || c.is_alphabetic() {
                    if let Some(_) = next {
                        panic!("Uh oh, we have 2 options! at line: {row}, col: {col}");
                    }
                    next = Some(*pos_dir);
                }
            }
        }
        // eprintln!("Turning {next:?} at {:?}", self.position);
        next
    }

    fn find_other_directions(&self) -> impl Iterator<Item = &Direction> {
        match self.direction {
            Direction::Up | Direction::Down => [Direction::Left, Direction::Right].iter(),
            Direction::Left | Direction::Right => [Direction::Up, Direction::Down].iter(),
        }
    }

    fn reached_end(&mut self, (row, col): (usize, usize)) {
        eprintln!("Came to end of maze at position: ({row}, {col}), heading: {:?}", self.direction);
        // self.print_context((row, col), 5);
        self.at_end = true;
    }

    #[allow(dead_code)]
    fn print_context(&self, (row, col): (usize, usize), window_size: i32) {
        // print the context of the maze around us
        let width = self.diagram.map[0].len();
        let height = self.diagram.map.len();

        for i in -window_size..=window_size {
            let r = min(max(0, row as i32 + i) as usize, height - 1);
            for j in -window_size..=window_size {
                let c = min(max(0, col as i32 + j) as usize, width - 1);
                print!("{}", self.diagram.map[r][c])
            }
            println!("");
        }
    }
}

impl Iterator for Walker {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if self.at_end {
            return None;
        }

        let (next_row, next_col) = self.find_neighbour_position(&self.direction);
        self.position = (next_row, next_col);

        if let Some(next_char) = self.find_char((next_row, next_col)) {
            if next_char == '+' {
                if let Some(direction) = self.find_next_direction() {
                    self.direction = direction;
                } else {
                    self.reached_end((next_row, next_col));
                }
            } else if next_char == ' ' {
                self.reached_end((next_row, next_col));
            }

            Some(next_char)
        } else {
            self.at_end = true;
            None
        }
    }
}
