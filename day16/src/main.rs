#[allow(unused_imports)]
use std::{fs, collections::HashMap, cmp::min, cmp::max};
use day16::*;
// use fancy_regex::Regex;
// use regex::Regex;
// use md5::{Digest, Md5};
// use priority_queue::PriorityQueue;

fn main() {
    let input = include_str!("../inputs/puzzle_input.txt");
    // let input = include_str!("../inputs/test_puzzle_input.txt");
    // println!("{:?}", input);
    println!("Input length: {}", input.len());

    // part_one(&input); 
    println!("Hello world!");
    part_two(&input);
    println!("Completed!")
}

#[allow(dead_code)]
fn part_one(input: &str) {
    let mut programs = Programs::new();
    println!("{}", programs);
    for mov in Moves::parse_moves(input) {
        mov.execute_move(&mut programs);
    }
    println!("{}", programs);
}

#[allow(dead_code)]
fn part_two(input: &str) {
    let mut programs = Programs::new();
    let moves: Vec<Moves> = Moves::parse_moves(input).collect();

    // find cycle length (how many dances to get back to starting position)
    let mut cycle_length = 0;
    for i in 0.. {
        for mov in &moves {
            mov.execute_move(&mut programs);
        }
        if programs.is_start() {
            println!("Cycle length is {0}", i + 1);
            println!("{programs}");
            cycle_length = i + 1;
            break;
        }
    }

    let actual_num_cycles = 1_000_000_000 % 60;

    let mut programs = Programs::new();
    for _ in 0..actual_num_cycles {
        for mov in &moves {
            mov.execute_move(&mut programs);
        }
    }

    println!("{programs}    - 1 billion!");
}