#[allow(unused_imports)]
use std::{cmp::max, cmp::min, collections::HashMap, fs};
// use fancy_regex::Regex;
// use regex::Regex;
// use md5::{Digest, Md5};
// use priority_queue::PriorityQueue;

const NUM_VALS: usize = 256;

fn main() {
    let input = include_str!("../inputs/puzzle_input.txt");
    // let input = include_str!("../inputs/test_puzzle_input.txt");
    // println!("{:?}", input);
    println!("Input lenght: {}", input.len());
    
    // part_one(input);
    part_two(input);
}

#[allow(dead_code)]
fn part_one(input: &str) {
    let lengths = get_lengths(input);
    let mut values = create_values();
    let _hash = hash(&mut values, &lengths, 0, 0).0;
    println!(
        "Multiple = {} * {} = {}",
        values[0],
        values[1],
        (values[0] as u32) * (values[1] as u32)
    );
}

#[allow(dead_code)]
fn part_two(input: &str) {
    let mut lengths = Vec::from(input.as_bytes());
    lengths.append(&mut vec![17, 31, 73, 47, 23]);
    let mut cur_idx = 0;
    let mut skip_size = 0;
    let mut values = create_values();
    for _ in 0..64 {
        (cur_idx, skip_size) = hash(&mut values, &lengths, cur_idx, skip_size);
    }
    
    // Create dense hash of the output
    let dense_hash = create_dense_hash(values);
    let output = dense_hash_to_string(&dense_hash);
    println!("{output}");
}

fn get_lengths(input: &str) -> Vec<u8> {
    let parts = input.split(',');
    let mut vec = Vec::new();
    for len in parts {
        let len = len.parse::<u8>().unwrap();
        vec.push(len);
    }
    vec
}

fn create_values() -> [u8; NUM_VALS] {
    let mut values = [0u8; NUM_VALS];
    for i in 0..NUM_VALS {
        values[i] = i as u8;
    }
    values
}

fn hash(
    values: &mut [u8; NUM_VALS],
    lengths: &Vec<u8>,
    mut cur_idx: usize,
    mut skip_size: usize,
) -> (usize, usize) {
    for len in lengths {
        reverse_values(values, cur_idx, *len);
        cur_idx += (*len as usize) + skip_size;
        skip_size += 1;

        // Make sure we loop back to beginning
        while cur_idx > values.len() {
            cur_idx -= values.len();
        }
    }
    (cur_idx, skip_size)
}

fn reverse_values(values: &mut [u8; NUM_VALS], cur_idx: usize, length: u8) {
    let u_length = length as usize;
    if cur_idx + u_length < values.len() {
        values[cur_idx..cur_idx + u_length].reverse();
    } else {
        let overflow = cur_idx + u_length - values.len();
        if overflow > values.len() {
            panic!("Overflow was greater than values array size");
        }
        let mut rev_vals = Vec::from(&values[cur_idx..]);
        for i in 0..overflow {
            rev_vals.push(values[i]);
        }
        rev_vals.reverse();

        for i in 0..rev_vals.len() {
            let mut idx = cur_idx + i;
            if idx >= values.len() {
                idx -= values.len();
            }

            values[idx] = rev_vals[i];
        }
    }
}

fn create_dense_hash(values: [u8; NUM_VALS]) -> [u8; 16] {
    let mut dense_hash = [0u8; 16];
    for i in 0..16 {
        let offset = i * 16;
        dense_hash[i] = xor_values(&values[offset..offset+16]);
    }
    dense_hash
}

fn xor_values(values: &[u8]) -> u8 {
    let mut result = values[0];
    for i in 1..values.len() {
        result ^= values[i];
    }
    result
}

fn dense_hash_to_string(dense_hash: &[u8]) -> String {
    let mut output = String::new();
    for i in 0..dense_hash.len() {
        output.push_str(&format!("{:02x}", dense_hash[i]));
    }
    output
}

