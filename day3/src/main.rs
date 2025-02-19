#[allow(unused_imports)]
use std::{fs, collections::HashMap, cmp::min, cmp::max};
// use fancy_regex::Regex;
// use regex::Regex;
// use md5::{Digest, Md5};
// use priority_queue::PriorityQueue;

fn main() {
    let input = 265149; 
    // let input = fs::read_to_string("./inputs/test_puzzle_input.txt").expect("Failed to read test input");
    println!("{:?}", input);

    part_two(input);  
}

#[allow(dead_code)]
fn part_one(input: i32) {
    let (x, y) = find_coords(input as usize);
    println!("Distance: {}", x.abs() + y.abs());
}

#[allow(dead_code)]
fn part_two(input: i32) {
    let mut hashmap = HashMap::new();
    hashmap.insert((0, 0), 1);
    let mut x = 0;
    let mut y = 0;
    let mut len = 2;
    let mut cur_len = 1;
    let mut dir = 0;

    loop {
        if cur_len == len {
            dir = (dir + 1) % 4;
            if dir % 2 == 0 {
                len += 1;
            }
            cur_len = 1;
        }
        match dir {
            0 => x += 1,
            1 => y += 1,
            2 => x -= 1,
            3 => y -= 1,
            _ => panic!("Invalid direction"),
        }

        let mut sum = 0;
        for i in -1..=1 {
            for j in -1..=1 {
                if i == 0 && j == 0 {
                    continue;
                }
                if let Some(val) = hashmap.get(&(x + i, y + j)) {
                    sum += val;
                }
            }
        }
        if sum > input {
            println!("First value larger than input: {}", sum);
            break;
        }
        hashmap.insert((x, y), sum);
        cur_len += 1;

    }
}

fn find_coords(n: usize) -> (isize, isize) {
    let mut x = 0;
    let mut y = 0;
    let mut len = 2;
    let mut cur_len = 1;
    let mut dir = 0;

    for _ in 1..n {
        if cur_len == len {
            dir = (dir + 1) % 4;
            if dir % 2 == 0 {
                len += 1;
            }
            cur_len = 1;
        }
        match dir {
            0 => x += 1,
            1 => y += 1,
            2 => x -= 1,
            3 => y -= 1,
            _ => panic!("Invalid direction"),
        }
        cur_len += 1;
    }
    (x, y)
}