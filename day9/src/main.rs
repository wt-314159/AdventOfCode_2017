#[allow(unused_imports)]
use std::{fs, collections::HashMap, cmp::min, cmp::max};
// use fancy_regex::Regex;
// use regex::Regex;
// use md5::{Digest, Md5};
// use priority_queue::PriorityQueue;

fn main() {
    let input = include_str!("../inputs/puzzle_input.txt");
    // let input = include_str!("../inputs/test_puzzle_input.txt");
    //println!("{:?}", input);
    println!("Input lenght: {}", input.len());

    part_two(&input); 
}

#[allow(dead_code)]
fn part_one(input: &str) {
    println!("Score: {}", calc_score_count_garbage(input).0);
}

#[allow(dead_code)]
fn part_two(input: &str) {
    println!("Amount of garbage: {}", calc_score_count_garbage(input).1);
}

fn calc_score_count_garbage(input: &str) -> (i32, i32) {
    let mut nesting = 0;
    let mut score = 0;
    let mut in_garbage = false;
    let mut skip = false;
    let mut garbage = 0;
    
    let chars = input.chars();
    for c in chars {
        // Probably a better way of doing this, but this should work for now
        if skip {
            skip = false;
            continue;
        }
        if in_garbage {
            // any character after ! should be ignored
            if c == '!' {
                skip = true;
            }
            // end of garbage
            else if c == '>' {
                in_garbage = false;
            }
            else {
                garbage += 1;
            }
        }
        else {
            if c == '<' {
                in_garbage = true;
            }
            else if c == '{' {
                nesting += 1;
            }
            else if c == '}' {
                score += nesting;
                nesting -= 1;
            }
        }
    }
    (score, garbage)
}