use std::collections::VecDeque;
#[allow(unused_imports)]
use std::{fs, collections::HashMap, cmp::min, cmp::max};
// use fancy_regex::Regex;
// use regex::Regex;
// use md5::{Digest, Md5};
// use priority_queue::PriorityQueue;

fn main() {
    let input = 324;
    // let input = 3;
    // let input = include_str!("../inputs/test_puzzle_input.txt");
    // println!("{:?}", input);
    println!("Input: {}", input);
    part_two(input); 
}

#[allow(dead_code)]
fn part_one(input: usize) {
    let (buffer, position) = execute_spinlock(input, 2017);
    println!("value after 2017: {}", buffer[position + 1]);
}

#[allow(dead_code)]
fn part_two(input: usize) {
    // calculate the indices that each value will be inserted at
    let (indices, _) = calculate_indices(input, 50_000_000);
    // then work backwards until one of them is inserted at position 1
    for i in (0..=50_000_000).rev() {
        if indices[i] == 0 {
            println!("Shouldn't have anything inserted at 0");
        }
        if indices[i] == 1 {
            println!("Value {i} is inserted at position 1");
            break;
        }
    }
}

fn execute_spinlock(input: usize, num_insertions: usize) -> (VecDeque<usize>, usize) {
    let vec: Vec<usize> = vec![0; 1];
    let mut buffer: VecDeque<usize> = vec.into_iter().collect();
    let mut length = 1;
    let mut position = 0;
    print_buffer(&buffer, length);
    for i in 1..=num_insertions {
        position = (input + position) % length;
        position += 1;
        // eprint!("position: {position}, length: {length}, insert at: {position},\t");
        buffer.insert(position, i);
        length += 1;
        //print_buffer(&buffer, length);   
        if i % 10000 == 0 {
            println!("\rcycle num: {i:#?}");
        }     
    }
    (buffer, position)
}

fn calculate_indices(input: usize, num_insertions: usize) -> (Vec<usize>, usize) {
    let mut position = 0;
    let mut calculated_indices = vec![0; num_insertions + 1];
    for i in 1..=num_insertions {
        position = (position + input) % i;
        position += 1;
        calculated_indices[i] = position;
    }
    (calculated_indices, position)
}

fn print_buffer(buffer: &VecDeque<usize>, length: usize) {
    for i in 0..length {
        print!("{} ", buffer[i]);
    }
    println!("");
}