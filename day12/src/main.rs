use std::{cell::RefCell, collections::HashSet, rc::Rc};
#[allow(unused_imports)]
use std::{cmp::max, cmp::min, collections::HashMap, fs};
// use fancy_regex::Regex;
// use regex::Regex;
// use md5::{Digest, Md5};
// use priority_queue::PriorityQueue;

fn main() {
    let input = include_str!("../inputs/puzzle_input.txt");
    // let input = include_str!("../inputs/test_puzzle_input.txt");
    // println!("{:?}", input);
    println!("Input lenght: {}", input.len());

    part_two(&input);
}

#[allow(dead_code)]
fn part_one(input: &str) {
    let houses = parse_input(input);
    eprintln!("Parsed input");
    let con_to_zero = connected_to_house(0, &houses);
    println!("Connected to zero: {}", con_to_zero.len());
}

#[allow(dead_code)]
fn part_two(input: &str) {
    let mut houses = parse_input(input);
    eprintln!("Parsed input");
    let mut groups = 0;
    while houses.len() > 0 {
        // choose a house as a nucleus basically at random
        let group_nucleus_id = houses.iter().next().unwrap().0;
        // find all connections to that house
        let con_to_id = connected_to_house(*group_nucleus_id, &houses);
        // only keep the houses which weren't in current group
        houses.retain(|id, _| !con_to_id.contains_key(id));
        groups += 1;
    }
    println!("Number of groups: {groups}");
}

fn parse_input(input: &str) -> HashMap<usize, House> {
    let mut houses = HashMap::new();
    for line in input.lines() {
        let mut parts = line.split_whitespace();
        let id = parts
            .next()
            .unwrap()
            .parse::<usize>()
            .expect(&format!("Failed to parse {line} to usize"));

        let entry = houses.entry(id).or_insert(House::new(id));
        for connected in parts.skip(1) {
            let con_id = connected
                .trim_end_matches(',')
                .parse::<usize>()
                .expect(&format!("Failed to parse {connected} to usize"));
            entry.connected.push(con_id);
        }
    }
    houses
}

fn connected_to_house(id: usize, houses: &HashMap<usize, House>) -> HashMap<usize, bool> {
    let mut con_to_id = Rc::new(RefCell::new(HashMap::new()));
    con_to_id.borrow_mut().insert(id, false);

    loop {
        let new_cons: Vec<usize> = con_to_id
            .borrow()
            .iter()
            .filter(|(_, b)| !**b)
            .map(|(c, _)| c.clone())
            .collect();
        if new_cons.len() == 0 {
            break;
        }
        for con in new_cons {
            // eprintln!("Looking at connections to {con}");
            let entry = &houses[&con];
            for dif_con in &entry.connected {
                con_to_id.borrow_mut().entry(*dif_con).or_insert(false);
            }
            let mut mut_borrow = con_to_id.borrow_mut();
            let finished = mut_borrow.entry(con).or_insert(false);
            *finished = true;
        }
    }
    con_to_id.take()
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct House {
    id: usize,
    connected: Vec<usize>,
}

impl<'a> House {
    fn new(id: usize) -> House {
        House {
            id,
            connected: Vec::new(),
        }
    }
}
