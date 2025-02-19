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
    let mut checksum = 0;
    for line in input.split("\n") {
        let parts = line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let max = parts.iter().max().unwrap();
        let min = parts.iter().min().unwrap();
        checksum += max - min;
    }
    println!("Checksum: {}", checksum);
}

#[allow(dead_code)]
fn part_two(input: &str) {
    let mut checksum = 0;
    for line in input.split("\n") {
        let parts = line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        for i in 0..parts.len() {
            for j in 0..parts.len() {
                if i != j && parts[i] % parts[j] == 0 {
                    checksum += parts[i] / parts[j];
                }
            }
        }
    }   
    println!("Checksum: {}", checksum);
}