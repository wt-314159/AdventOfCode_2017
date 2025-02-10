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

    //part_one(&input);
    part_two(&input);
}

#[allow(dead_code)]
fn part_one(input: &str) {
    let mut prev_int;
    let mut cur_int = 0;
    let mut sum = 0;
    for c in input.chars() {
        prev_int = cur_int;
        cur_int = c.to_digit(10).unwrap();
        if prev_int == cur_int {
            sum += cur_int;
        }
    }

    if input.chars().next().unwrap().to_digit(10).unwrap() == cur_int {
        sum += cur_int;
    }

    println!("Sum: {}", sum);
}

#[allow(dead_code)]
fn part_two(input: &str) {
    let mut sum = 0;
    let comp_int = input.len() / 2;
    let chars = input.chars().collect::<Vec<char>>();

    for i in 0..input.len() {
        let cur_int = chars[i].to_digit(10).unwrap();
        let mut comp_index = i + comp_int;
        if comp_index >= input.len() {
            comp_index -= input.len();
        } 
        let comp_int = chars[comp_index].to_digit(10).unwrap();
        if cur_int == comp_int {
            sum += cur_int;
        }
    }

    println!("Sum: {}", sum);
}