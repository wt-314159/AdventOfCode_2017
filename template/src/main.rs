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

    part_one(&input); 
}

#[allow(dead_code)]
fn part_one(input: &str) {

}

#[allow(dead_code)]
fn part_two(input: &str) {
    
}