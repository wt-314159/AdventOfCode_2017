#[allow(unused_imports)]
use std::{fs, collections::HashMap, cmp::min, cmp::max};
// use fancy_regex::Regex;
// use regex::Regex;
// use md5::{Digest, Md5};
// use priority_queue::PriorityQueue;

fn main() {
    let input = fs::read_to_string("./inputs/puzzle_input.txt").expect("Failed to read input");
    // let input = fs::read_to_string("./inputs/test_puzzle_input.txt").expect("Failed to read test input");
    println!("{:?}", input);
    println!("Input lenght: {}", input.len());
    
    part_two(&input);
}

#[allow(dead_code)]
fn part_one(input: &str) {
    let mut instructions: Vec<i32> = input.split("\n")
        .map(|s| {
            if let Ok(num) = s.trim().parse::<i32>() {
                return num;
            }
            else { 
                println!("Failed to parse instruction: {}", s); 
                42
            }
        })
        .collect();

    let mut idx: i32 = 0;
    let mut steps = 0;
    let len = instructions.len() as i32;
    while idx >= 0 && idx < len {
        let index = idx as usize;
        let jump = instructions[index];
        instructions[index] += 1;
        idx += jump;
        steps += 1;
    }
    println!("Steps: {}", steps);
}

#[allow(dead_code)]
fn part_two(input: &str) {
    let mut instructions: Vec<i32> = input.split("\n")
        .map(|s| {
            if let Ok(num) = s.trim().parse::<i32>() {
                return num;
            }
            else { 
                println!("Failed to parse instruction: {}", s); 
                42
            }
        })
        .collect();

    let mut idx: i32 = 0;
    let mut steps = 0;
    let len = instructions.len() as i32;

    while idx >= 0 && idx < len {
        let index = idx as usize;
        let jump = instructions[index];
        if jump >= 3 {
            instructions[index] -= 1;
        }
        else {
            instructions[index] += 1;
        }
        idx += jump;
        steps += 1;
    }
    println!("Steps: {}", steps);
 
}