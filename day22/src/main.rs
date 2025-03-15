use std::{char::ParseCharError, collections::BTreeMap, str::FromStr};
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
    let mut grid = input.parse::<Grid>().unwrap();
    for _ in 0..10_000 {
        grid.step();
    }

    println!(
        "After 10,000 steps, {} clean cells were infected",
        grid.infected_count
    );
}

#[allow(dead_code)]
fn part_two(input: &str) {
    let mut grid = input.parse::<NewGrid>().unwrap();
    for _ in 0..10_000_000 {
        grid.step();
    }

    println!(
        "After 10,000 steps, {} clean cells were infected",
        grid.infected_count
    );
}

#[derive(Debug, Clone, Copy)]
enum State {
    Clean,
    Weakened,
    Infected,
    Flagged,
}

impl State {
    fn toggle(&self) -> State {
        match self {
            State::Clean => State::Weakened,
            State::Weakened => State::Infected,
            State::Infected => State::Flagged,
            State::Flagged => State::Clean,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn turn_left(&self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
        }
    }

    fn reverse(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
        }
    }
}

#[derive(Debug, Clone)]
struct Grid {
    map: BTreeMap<(i32, i32), bool>,
    carrier_location: (i32, i32),
    carrier_direction: Direction,
    infected_count: usize,
}

impl FromStr for Grid {
    type Err = ParseCharError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map = BTreeMap::new();

        s.lines()
            .enumerate()
            .map(|(r, l)| {
                l.chars().enumerate().map(move |(c, b)| {
                    (
                        (r as i32, c as i32),
                        match b {
                            '#' => true,
                            '.' => false,
                            _ => panic!("Can't match {b}"),
                        },
                    )
                })
            })
            .flatten()
            .for_each(|(k, v)| {
                map.insert(k, v);
            });
        let rows = s.lines().count() as i32;
        let cols = s.lines().next().unwrap().len() as i32;

        Ok(Self {
            map,
            carrier_location: (rows / 2, cols / 2),
            carrier_direction: Direction::Up,
            infected_count: 0,
        })
    }
}

impl Grid {
    fn step(&mut self) {
        let entry = self.map.entry(self.carrier_location).or_insert(false);
        self.carrier_direction = if *entry {
            self.carrier_direction.turn_right()
        } else {
            self.infected_count += 1;
            self.carrier_direction.turn_left()
        };
        *entry = !*entry;
        self.move_forward();
    }

    fn move_forward(&mut self) {
        self.carrier_location = match self.carrier_direction {
            Direction::Up => (self.carrier_location.0 - 1, self.carrier_location.1),
            Direction::Right => (self.carrier_location.0, self.carrier_location.1 + 1),
            Direction::Down => (self.carrier_location.0 + 1, self.carrier_location.1),
            Direction::Left => (self.carrier_location.0, self.carrier_location.1 - 1),
        }
    }
}

#[derive(Debug, Clone)]
struct NewGrid {
    map: BTreeMap<(i32, i32), State>,
    carrier_location: (i32, i32),
    carrier_direction: Direction,
    infected_count: usize,
}

impl FromStr for NewGrid {
    type Err = ParseCharError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map = BTreeMap::new();

        s.lines()
            .enumerate()
            .map(|(r, l)| {
                l.chars().enumerate().map(move |(c, b)| {
                    (
                        (r as i32, c as i32),
                        match b {
                            '#' => State::Infected,
                            '.' => State::Clean,
                            _ => panic!("Can't match {b}"),
                        },
                    )
                })
            })
            .flatten()
            .for_each(|(k, v)| {
                map.insert(k, v);
            });
        let rows = s.lines().count() as i32;
        let cols = s.lines().next().unwrap().len() as i32;
        
        Ok(Self {
            map,
            carrier_location: (rows / 2, cols / 2),
            carrier_direction: Direction::Up,
            infected_count: 0,
        })
    }
}

impl NewGrid {
    fn step(&mut self) {
        let entry = self
            .map
            .entry(self.carrier_location)
            .or_insert(State::Clean);
        self.carrier_direction = match *entry {
            State::Clean => self.carrier_direction.turn_left(),
            State::Weakened => {
                self.infected_count += 1;
                self.carrier_direction
            }, 
            State::Infected => self.carrier_direction.turn_right(),
            State::Flagged => self.carrier_direction.reverse(),
        };
        *entry = entry.toggle();
        self.move_forward();
    }

    fn move_forward(&mut self) {
        self.carrier_location = match self.carrier_direction {
            Direction::Up => (self.carrier_location.0 - 1, self.carrier_location.1),
            Direction::Right => (self.carrier_location.0, self.carrier_location.1 + 1),
            Direction::Down => (self.carrier_location.0 + 1, self.carrier_location.1),
            Direction::Left => (self.carrier_location.0, self.carrier_location.1 - 1),
        }
    }
}
