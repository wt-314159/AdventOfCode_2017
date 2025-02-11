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

    part_one(&input);
}

#[allow(dead_code)]
fn part_one(input: &str) {
    let towers = create_graph(input);
    let (_, root) = towers.iter()
        .find(|(_, t)| t.borrow().parent.is_none())
        .expect("Failed to find any nodes without parents");
    println!("Root node: {}", root.borrow().name);
}

#[allow(dead_code)]
fn part_two(input: &str) {
    let towers = create_graph(input);

    // recursive_weight_check(&"rqwgj", &towers);
}

#[derive(Debug)]
struct Tower<'a> {
    name: &'a str,
    weight: i32,
    children: Option<Vec<&'a str>>,
    parent: Option<Rc<RefCell<Tower<'a>>>>
}

fn create_graph(input: &str) -> HashMap<&str, Rc<RefCell<Tower>>> {
    let mut towers: HashMap<&str, Rc<RefCell<Tower>>> = HashMap::new();
    for line in input.split("\n") {
        insert_tower(line, &mut towers);
    }
    towers
}

fn insert_tower<'a>(line: &'a str, towers: &mut HashMap<&'a str, Rc<RefCell<Tower<'a>>>>) {
    let mut parts = line.split_whitespace();
    let name = parts.next().unwrap();
    let weight = parts.next().unwrap()
        .trim_matches(|c| c == '(' || c == ')')
        .parse::<i32>()
        .expect("Failed  to parse value");

    let mut opt_children = None;
    if let Some(_) = parts.next() {
        let mut children = Vec::new();
        while let Some(child) = parts.next() {
            children.push(child.trim_matches(|c| c == ',' || c == ' '));
        }
        opt_children = Some(children);
    }
    
    let mut opt_parent = None;
    let tower = Tower::new(name, weight, opt_children, None);
    let tower = Rc::new(RefCell::new(tower));

    for (&i_name, i_tower) in towers.iter_mut() {
        // Check all existing towers to see if any of their children is current tower
        // If so, make current tower's parent that tower
        if let Some(ref i_children) = i_tower.borrow().children {
            if let Some(par) = i_children.iter()
                .find(|i_c| ***i_c == *name) 
            {
                opt_parent = Some(Rc::clone(&i_tower));
            }
        }
        // Check all existing towers to see if any of current towers children already exist
        // If so, set that tower's parent to current tower
        if let Some(ref children) = tower.borrow().children {
            if let Some(par) = children.iter()
                .find(|c| ***c == *i_name) 
            {
                i_tower.borrow_mut().parent = Some(Rc::clone(&tower));
            }
        }
    }

    tower.borrow_mut().parent = opt_parent;
    // insert tower into hashmap
    towers.insert(name, tower);
}

impl<'a> Tower<'a> {
    fn new(name: &'a str, weight: i32, children: Option<Vec<&'a str>>, parent: Option<Rc<RefCell<Tower<'a>>>>) -> Tower<'a> {
        Tower {
            name,
            weight: weight,
            children,
            parent,
        }
    }
}