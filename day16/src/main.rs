#[allow(unused_imports)]
use std::{fs, collections::HashMap, cmp::min, cmp::max};
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
    let mut programs = create_programs();
    for mov in parse_moves(input) {
        mov.execute_move(&mut programs);
    }
    println!("{:?}", programs);
}

#[allow(dead_code)]
fn part_two(input: &str) {
    
}

fn create_programs() -> [char; 16] {
    ['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p']
}

fn parse_moves(input: &str) -> impl Iterator<Item = Moves> + use<'_>{
    input.split(',').map(|move_str| {
        match move_str.chars().nth(0) {
            Some('s') => Moves::Spin(move_str[1..].parse::<usize>().expect("Failed to parse int")),
            Some('x') => {
                let rest = &move_str[1..];
                let mut split = rest.split('/');
                let a = split.next().unwrap().parse::<usize>().expect("a: failed to parse to int");
                let b =  split.next().unwrap().parse::<usize>().expect("b: failed to parse to int");
                Moves::Exchange(a, b)
            }
            Some('p') => {
                let rest = &move_str[1..];
                let a = rest.chars().nth(0).unwrap();
                let b = rest.chars().nth(2).unwrap();
                Moves::Partner(a, b)
            }
            _ => panic!("Can't parse {move_str}")
        }
    })
}

enum Moves {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}

impl Moves {
    fn execute_move(&self, programs: &mut [char; 16]) {
        match self {
            Self::Spin(num) => {
                programs.rotate_right(*num);
            },
            Self::Exchange(a, b) => {
                programs.swap(*a, *b);
            },
            Self::Partner(a, b) => {
                let a = programs.iter().position(|c| c==a).unwrap();
                let b = programs.iter().position(|c| c==b).unwrap();
                programs.swap(a, b);
            }
        }
    }
}