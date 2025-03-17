#[allow(unused_imports)]
use std::{cmp::max, cmp::min, collections::HashMap, fs};
use std::{fmt::Display, num::ParseIntError, str::FromStr};
// use fancy_regex::Regex;
// use regex::Regex;
// use md5::{Digest, Md5};
// use priority_queue::PriorityQueue;

fn main() {
    let input = include_str!("../inputs/puzzle_input.txt");
    // let input = include_str!("../inputs/test_puzzle_input.txt");
    // println!("{:?}", input);
    println!("Input length: {}", input.len());

    part_two(&input);
}

#[allow(dead_code)]
fn part_one(input: &str) {
    let mut components = input
        .lines()
        .map(|l| l.parse::<Component>().expect("Can't parse line"))
        .collect::<Vec<_>>();
    components.sort_by(|a, b| a.strength.cmp(&b.strength));

    let used = components.iter().map(|_| false).collect();
    let strongest = find_strongest_bridge(&components, used, 0, 0, 0, None);

    println!("Strongest bridge found had strength: {strongest}");
}

#[allow(dead_code)]
fn part_two(input: &str)  {
    let mut components = input
        .lines()
        .map(|l| l.parse::<Component>().expect("Can't parse line"))
        .collect::<Vec<_>>();
    components.sort_by(|a, b| a.strength.cmp(&b.strength));

    let used = components.iter().map(|_| false).collect();

    let longest = find_longest_bridge(&components, used, 0, 0, 0, 0, 0, None);
    eprintln!("Longest bridge found had length: {}", longest.1);
    println!("Strength of the longest bridge was: {}", longest.0);
}

fn find_strongest_bridge(
    components: &Vec<Component>,
    mut used: Box<[bool]>,
    mut strongest: usize,
    cur_pin: usize,
    cur_strength: usize,
    prev_used: Option<usize>,
) -> usize {
    if let Some(idx) = prev_used {
        used[idx] = true;
    }
    let mut found_any = false;
    for (i, c) in components
        .iter()
        .enumerate()
        .filter(|(i, c)| !used[*i] && c.contains(cur_pin))
    {
        found_any = true;
        let ret_strongest = find_strongest_bridge(
            components,
            used.clone(),
            strongest,
            c.other_if_matches(cur_pin).unwrap(),
            cur_strength + c.strength,
            Some(i),
        );
        if ret_strongest > strongest {
            eprintln!("Better bridge found, strength: {ret_strongest}");
            strongest = ret_strongest;
        }
    }
    if !found_any {
        // we can't find any matching components, now compare the current strength
        // to the strongest strength bridge we've found
        if cur_strength > strongest {
            eprintln!("End of strongest bridge yet found: {cur_strength}");
            return cur_strength;
        }
    }
    strongest
}

fn find_longest_bridge(
    components: &Vec<Component>,
    mut used: Box<[bool]>,
    mut longest_strength: usize,
    mut longest: usize,
    cur_pin: usize,
    cur_strength: usize,
    cur_length: usize,
    prev_used: Option<usize>,
) -> (usize, usize) {
    if let Some(idx) = prev_used {
        used[idx] = true;
    }
    let mut found_any = false;
    for (i, c) in components
        .iter()
        .enumerate()
        .filter(|(i, c)| !used[*i] && c.contains(cur_pin))
    {
        found_any = true;
        let (ret_strongest, ret_longest) = find_longest_bridge(
            components,
            used.clone(),
            longest_strength,
            longest,
            c.other_if_matches(cur_pin).unwrap(),
            cur_strength + c.strength,
            cur_length + 1,
            Some(i),
        );
        if ret_longest > longest || (ret_longest == longest && ret_strongest > longest_strength) {
            eprintln!("Longer bridge found, length: {ret_longest}");
            longest = ret_longest;
            longest_strength = ret_strongest;
        }
    }
    if !found_any {
        // we can't find any matching components, now compare the current strength
        // to the strongest strength bridge we've found
        if cur_length > longest || (cur_length == longest && cur_strength > longest_strength) {
            eprintln!("End of longest bridge yet found: {cur_length}");
            longest = cur_length;
            longest_strength = cur_strength;
        }
    }
    (longest_strength, longest)
}

#[derive(Debug, Clone, Copy)]
struct Component {
    ports: (usize, usize),
    strength: usize,
}

impl Display for Component {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.ports.0, self.ports.1)
    }
}

impl FromStr for Component {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split('/');
        let ports = (
            split.next().unwrap().parse::<usize>()?,
            split.next().unwrap().parse::<usize>()?,
        );
        Ok(Component {
            ports,
            strength: ports.0 + ports.1,
        })
    }
}

impl Component {
    fn contains(&self, val: usize) -> bool {
        self.ports.0 == val || self.ports.1 == val
    }

    fn other_if_matches(&self, val: usize) -> Option<usize> {
        if self.ports.0 == val {
            Some(self.ports.1)
        } else if self.ports.1 == val {
            Some(self.ports.0)
        } else {
            None
        }
    }
}
