#[allow(unused_imports)]
use std::{fs, collections::HashMap, cmp::min, cmp::max};
// use fancy_regex::Regex;
// use regex::Regex;
// use md5::{Digest, Md5};
// use priority_queue::PriorityQueue;

fn main() {
    let input = (699, 124);
    // let input = (65, 8921);
    // println!("{:?}", input);
    println!("Input: {input:?}");

    part_two(input); 
}

#[allow(dead_code)]
fn part_one(input: (u64, u64)) {
    let mut gen_a = Generator::new(input.0, 16807, 4);
    let mut gen_b = Generator::new(input.1, 48271, 8);
    // println!("--Gen. A--  --Gen. B--");
    let mut a;
    let mut b;
    let mut matches = 0;
    for _ in 0..40_000_000 {
        a = gen_a.gen_next_val();
        b = gen_b.gen_next_val();
        // println!("{:>10}  {:>10}", a, b);
        if compare_last_bits(a, b) {
            matches += 1;
        }
    }
    println!("There were {} matches", matches);
}

#[allow(dead_code)]
fn part_two(input: (u64, u64)) {
    let mut gen_a = Generator::new(input.0, 16807, 4);
    let mut gen_b = Generator::new(input.1, 48271, 8);
    // println!("--Gen. A--  --Gen. B--");
    let mut a;
    let mut b;
    let mut matches = 0;
    for _ in 0..5_000_000 {
        a = gen_a.gen_picky_val();
        b = gen_b.gen_picky_val();
        // println!("{:>10}  {:>10}", a, b);
        if compare_last_bits(a, b) {
            matches += 1;
        }
    }
    println!("There were {} matches", matches);
}

fn compare_last_bits(val1: u64, val2: u64) -> bool {
    for i in 0..16 {
        if (val1 >> i) & 1 != (val2 >> i) & 1 {
            return false;
        }
    }
    true
}

struct Generator {
    prev_val: u64,
    factor: u64,
    divisor: u64,
    criteria: u64,
}

impl Generator {
    fn new(starting_val: u64, factor: u64, criteria: u64) -> Self {
        Self { prev_val: starting_val, factor, divisor: 2147483647, criteria }
    }

    fn gen_next_val(&mut self) -> u64 {
        self.prev_val = (self.prev_val * self.factor) % self.divisor;
        self.prev_val
    }

    fn gen_picky_val(&mut self) -> u64 {
        self.prev_val = (self.prev_val * self.factor) % self.divisor;
        while self.prev_val % self.criteria != 0 {
            self.prev_val = (self.prev_val * self.factor) % self.divisor;
        }
        self.prev_val
    } 
}