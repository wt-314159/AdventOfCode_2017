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

    calculate_registers(&input); 
}

#[allow(dead_code)]
fn calculate_registers(input: &str) {
    let mut registers = create_registers(input);
    let mut max = 0;
    for line in input.split("\n") {
        let parts = Parts::from_line(line);
        let comp_register = registers.get(parts.comp_register)
            .expect("Failed to get the comp register");

        if calculate(parts.operation, comp_register, &parts.comp_amount) {
            let mut entry = registers.entry(parts.register_name).or_default();
            *entry += parts.increment;
            if *entry > max {
                max = *entry;
            }
        }
    }
    let largest = registers.iter().max_by(|(_, &x), (_, y)| x.cmp(y))
        .expect("Failed to find largest value");
    println!("Largest value was register '{}' with value '{}'", largest.0, largest.1);
    println!("Largest value ever held was: {}", max);
}

fn create_registers(input: &str) -> HashMap<&str, i32> {
    let mut register = HashMap::new();
    for line in input.split("\n") {
        let name = line.split_whitespace().next()
            .expect("Line didn't have a register name.");
        register.insert(name, 0);
    }
    register
}

fn calculate(op_string: &str, op1: &i32, op2: &i32) -> bool {
    match op_string {
        "==" => op1 == op2,
        "!=" => op1 != op2,
        ">" => op1 > op2,
        ">=" => op1 >= op2,
        "<" => op1 < op2,
        "<=" => op1 <= op2,
        other=> panic!("Operation string not recognised: {}", other)
    }
}

struct Parts<'a> {
    register_name: &'a str,
    increment: i32,
    comp_register: &'a str,
    operation: &'a str,
    comp_amount: i32
}

impl<'a> Parts<'a> {
    fn from_line(line: &'a str) -> Parts<'a> {
        let mut parts = line.split_whitespace();
        let register_name = parts.next().expect("Register name not found");
        let dec = parts.next().unwrap();
        let increment = parts.next().expect("Increment amount");
        parts.next();
        let comp_register = parts.next().expect("Comp register name not found");
        let operation = parts.next().expect("Operation string not found");
        let comp_amount = parts.next().expect("Comp amount not found");

        let comp_amount = comp_amount.parse::<i32>()
            .expect("Failed to parse comp amount");
        let mut increment = increment.parse::<i32>()
            .expect("Failed to parse increment amount");
        if dec == "dec" {
            increment *= -1;
        }

        Parts { register_name, increment, comp_register, operation, comp_amount }
    }
}