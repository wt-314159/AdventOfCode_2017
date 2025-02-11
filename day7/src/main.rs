use std::{cell::RefCell, rc::Rc};
#[allow(unused_imports)]
use std::{fs, collections::HashMap, cmp::min, cmp::max};
// use fancy_regex::Regex;
// use regex::Regex;
// use md5::{Digest, Md5};
// use priority_queue::PriorityQueue;

fn main() {
    let input = fs::read_to_string("./inputs/puzzle_input.txt").expect("Failed to read input");
    // let input = fs::read_to_string("./inputs/test_puzzle_input.txt").expect("Failed to read test input");
    //println!("{:?}", input);
    println!("Input lenght: {}", input.len());

    part_two(&input);
}

#[allow(dead_code)]
fn part_one(input: &str) {
    let mut towers: HashMap<&str, Tower> = HashMap::new();
    for line in input.split("\n") {
        let tower = Tower::from_line(line, &mut towers);
        towers.insert(&tower.name, tower);
    }

    for (name, tower) in towers.iter() {
        if tower.parent.is_none() {
            println!("Root tower: {}", name);
        }
    }
}

#[allow(dead_code)]
fn part_two(input: &str) {
    let mut towers: HashMap<&str, Tower> = HashMap::new();
    for line in input.split("\n") {
        let tower = Tower::from_line(line, &mut towers);
        towers.insert(&tower.name, tower);
    }

    recursive_weight_check(&"rqwgj", &towers);
}

fn recursive_weight_check(cur_tower: &str, towers: &HashMap<&str, Tower>) -> (i32, bool) {
    let tower = towers.get(cur_tower).unwrap();
    let mut weight = tower.weight;
    let mut children_weight: Option<Vec<(&str, i32)>> = None;

    if let Some(children) = &tower.children {
        for child in children {
            let (child_weight , cont) = recursive_weight_check(child, towers);
            if !cont {
                return (0, false);
            }
            if let Some(ref mut cw) = children_weight {
                cw.push((child, child_weight));
            }
            else {
                let mut weights = Vec::new();
                weights.push((*child, child_weight));
                children_weight = Some(weights); 
            }
            weight += child_weight;
        }
        let weight_vec = children_weight.unwrap();
        let (_, weight) = weight_vec.first().unwrap();
        for (_, w) in &weight_vec {
            if *w != *weight {
                // HACK 
                // This is really hacky code, there's definitely a more efficient and elegant
                // way to do this. Would be better if we created a directed graph structure of 
                // tower nodes, but that would involve using Rc<RefCell<Tower>> and that's a hassle
                println!("Tower: '{}', weight: {}, children_weights: {:?}", cur_tower, tower.weight, weight_vec);
                let mut weight_map = HashMap::new();
                for (name, w) in &weight_vec {
                    let count = weight_map.entry(w).or_insert(0);
                    *count += 1;
                }
                let odd = weight_map.iter().find(|(_, v)| **v == 1).unwrap().0;
                let odd_name = weight_vec.iter().find(|(_, w)| *w == **odd).unwrap().0;
                let odd_tower = towers.get(odd_name).unwrap();
                println!("Odd tower: '{}', weight: '{}'", odd_name, odd_tower.weight);
                return (0, false);
            }
        }
    }
    (weight, true)
}

struct Tower<'a> {
    name: &'a str,
    weight: i32,
    children: Option<Vec<&'a str>>,
    parent: Option<&'a str>
}

impl<'a> Tower<'a> {
    fn new(name: &'a str, weight: i32) -> Tower<'a> {
        Tower {
            name,
            weight: weight,
            children: None,
            parent: None
        }
    }

    fn with_children(name: &'a str, weight: i32, children: Option<Vec<&'a str>>, parent: Option<&'a str>) -> Tower<'a> {
        Tower {
            name: name,
            weight,
            children: children,
            parent,
        }
    }

    fn from_line<'b>(line: &'b str, towers: &mut HashMap<&'b str, Tower<'b>>) -> Tower<'b> {
        let mut parts = line.split_whitespace();
        let name = parts.next().unwrap();
        if name == "ugml" {
            println!("Name: '{}'", name);
        }
        let weight = parts.next().unwrap()
            .trim_matches(|c| c == '(' || c == ')')
            .parse::<i32>().unwrap();

        // If tower has children
        let mut opt_children = None;
        if let Some(_) = parts.next() {
            let mut children = Vec::new();
            while let Some(child) = parts.next() {
                let child = child.trim_matches(|c| c == ',' || c == ' ');
                children.push(child);
            }
            opt_children = Some(children);
        }

        let mut opt_parent = None;
        for (i_name, tower) in towers.iter_mut() {
            if let Some(children) = &opt_children {
                if children.iter().any(|c| **c == **i_name) {
                    tower.parent = Some(i_name);
                }
            }
            if let Some(i_children) = &tower.children {
                if i_children.iter().any(|c| **c == *name) {
                    opt_parent = Some(*i_name);
                } 
            }
        }

        Tower::with_children(name, weight, opt_children, opt_parent)
    }
}