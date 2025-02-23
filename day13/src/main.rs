#[allow(unused_imports)]
use std::{fs, collections::HashMap, cmp::min, cmp::max};
// use fancy_regex::Regex;
// use regex::Regex;
// use md5::{Digest, Md5};
// use priority_queue::PriorityQueue;

fn main() {
    let input = include_str!("../inputs/puzzle_input.txt");
    // let input = include_str!("../inputs/test_puzzle_input.txt");
    // println!("{:?}", input);
    println!("Input lenght: {}", input.len());

    // part_one(&input); 
    part_two(input);
}

#[allow(dead_code)]
fn part_one(input: &str) {
    let mut layers = parse_input(input);

    let mut total_severity = 0;
    for i in 0..layers.len() {
        if let Some(wall) = &layers[i] {
            if wall.scanner_idx == 0 {
                total_severity += wall.get_severity();
            }
        }
        layers.iter_mut().for_each(|l| {
            if let Some(w) = l {
                w.advance_scanner();
            }
        });
    }
    println!("Total severity of trip: {total_severity}");
}

#[allow(dead_code)]
fn part_two(input: &str) {
    let mut layers = parse_input(input);
    
    let mut delay = 0;
    loop {
        eprint!("\r\tDelay {delay}");
        if attempt_with_delay(&mut layers, delay) {
            break;
        }
        delay += 1;
    }
    println!("Succeeded with delay {}", delay);
}

fn attempt_with_delay(layers: &mut Vec<Option<Layer>>, delay: usize) -> bool {
    layers.iter_mut().for_each(|l| {
        if let Some(w) = l {
            w.advance_scanner_n_times(delay);
        }
    });
    for i in 0..layers.len() {
        if let Some(wall) = &layers[i] {
            if wall.scanner_idx == 0 {
                return false;
            }
        }
        layers.iter_mut().for_each(|l| {
            if let Some(w) = l {
                w.advance_scanner();
            }
        });
    }
    true
}

fn parse_input(input: &str) -> Vec<Option<Layer>> {
    let mut layers = Vec::new(); 
    let mut prev_depth = 0;
    for line in input.lines() {
        let mut parts = line.split(": ");
        let depth = parts.next().unwrap().parse::<usize>().unwrap();
        let range = parts.next().unwrap().parse::<usize>().unwrap();
        let depth_diff = depth - prev_depth;
        if depth_diff > 1 {
            layers.append(&mut (1..depth_diff).map(|_| None).collect());
        }
        layers.push(Some(Layer::new(depth, range)));
        prev_depth = depth;
    }
    layers
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Layer {
    depth: usize,
    range: usize,
    scanner_idx: usize,
    scanner_dir: bool,
    period: usize
}

impl Layer {
    fn get_severity(&self) -> usize {
        self.depth * self.range
    }
    
    fn new(depth: usize, range: usize) -> Self {
        let period = (range - 1) * 2;
        Layer { depth, range, scanner_idx: 0, scanner_dir: true, period }
    }

    fn advance_scanner(&mut self) {
        if self.scanner_dir {
            self.scanner_idx += 1;
            if self.scanner_idx >= self.range {
                // turnaround
                self.scanner_idx -= 2;
                self.scanner_dir = false;
            }
        }
        else {
            if self.scanner_idx == 0 {
                // turnaround
                self.scanner_idx += 1;
                self.scanner_dir = true;
            }
            else {
                self.scanner_idx -= 1;
            }
        }
    }

    fn advance_scanner_n_times(&mut self, n: usize) {
        let offset = n % self.period;
        for _ in 0..offset {
            self.advance_scanner();
        }
    }

    fn scanner_at_0(&self, time: usize) -> bool {
        self.period % time == 0
    }
}