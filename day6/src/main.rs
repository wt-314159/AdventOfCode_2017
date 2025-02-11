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
    let mut past_iterations: Vec<[i32;16]> = Vec::new();
    let mut banks = create_banks_array(input);
    let mut count = 0;

    while !past_iterations.contains(&banks) {
        past_iterations.push(banks);
        let max = find_max(&banks);
        banks = *redistribute(&mut banks, max);
        count += 1;
    }
    println!("No. cycles: {}", count);
}

#[allow(dead_code)]
fn part_two(input: &str) {
    let mut past_iterations: HashMap<[i32;16], usize> = HashMap::new();
    let mut banks = create_banks_array(input);
    let mut count = 0;

    while !past_iterations.contains_key(&banks) {
        past_iterations.insert(banks, count);
        let max = find_max(&banks);
        banks = *redistribute(&mut banks, max);
        count += 1;
    }
    println!("No. cycles: {}", count);
    println!("Loop size: {}", count - past_iterations.get(&banks).unwrap());
}

fn create_banks_array(input: &str) -> [i32; 16] {
    let mut banks = input.split_whitespace()
        .map(|s| s.trim().parse::<i32>().unwrap());
    let mut bank_array = [0; 16];
    for i in 0..16 {
        bank_array[i] = banks.next().unwrap();
    }
    bank_array
}

fn find_max(banks: &[i32; 16]) -> usize {
    let mut max = 0;
    for i in 0..16 {
        if banks[i] > banks[max] {
            max = i;
        }
    }
    max
}

fn redistribute(banks: &mut [i32; 16], mut max: usize) -> &mut [i32; 16] {
        let count = banks[max];
        banks[max] = 0;

        for _ in 1..count+1 {
            max += 1;
            if max >= 16 {
                max = 0;
            }
            banks[max] += 1;
        }
        banks
}