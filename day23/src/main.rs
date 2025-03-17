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
    let mut registers = Registers::default();
    let instructions = parse_instructions(input).collect::<Vec<_>>();
    let mut instr_idx: i32 = 0;
    let instr_len = instructions.len() as i32;
    while instr_idx >= 0 && instr_idx < instr_len {
        instr_idx += registers.apply_instruction(&instructions[instr_idx as usize]);
    }
    println!("Num multiplications: {}", registers.num_mults);
}

#[allow(dead_code)]
fn part_two(_input: &str) {
    let h = translated_code();
    println!("Value in h: {h}");
}

fn translated_code() -> i32 {
    // Hand translated and simplified code from input (assuming a = 1)
    // essentially looking for number of non-prime number between 2 large
    // numbers
    let mut h = 0;
    let b = 79 * 100 + 100_000;
    let c = b + 17_000;

    for num in (b..=c).step_by(17) {
        // If num isn't prime, increment h
        let len = num / 2;
        for d in 2..len {
            if num % d == 0 {
                h += 1;
                break;
            }
        }
    }
    h
}

fn parse_instructions(input: &str) -> impl Iterator<Item = Instruction> + use<'_> {
    input.lines().map(|l| {
        let mut parts = l.split_whitespace();
        Instruction::parse(
            parts.next().unwrap(),
            parts
                .next()
                .unwrap()
                .parse::<Value>()
                .expect("Failed to parse value"),
            parts
                .next()
                .unwrap()
                .parse::<Value>()
                .expect("Failed to parse value"),
        )
        .expect("Failed to parse instruction")
    })
}

#[allow(dead_code)]
fn find_program_exit_points(instructions: &Vec<Instruction>) {
    for (i, instr) in instructions.iter().enumerate() {
        match instr {
            Instruction::Jnz(_, value) => match value {
                Value::Number(n) => {
                    if i as i32 + *n >= instructions.len() as i32 || i as i32 + n < 0 {
                        eprintln!(
                            "Program will exit if line {0} is reached, {0}: '{instr:?}'",
                            i + 1
                        );
                    }
                }
                Value::Register(c) => {
                    eprintln!("Program could exit at line {0}, if register {c} has value x >= {1} or x < -{2}, {0}: '{instr:?}'",
                            i + 1,
                            instructions.len() - i,
                            i)
                }
            },
            _ => {}
        }
    }
}

#[allow(dead_code)]
fn find_writes_to_h(instructions: &Vec<Instruction>) {
    let sets_h = instructions
        .iter()
        .enumerate()
        .filter(|(_, instr)| match instr {
            Instruction::Set(value, _) => value.register_or_none() == Some(&'h'),
            Instruction::Sub(value, _) => value.register_or_none() == Some(&'h'),
            Instruction::Mul(value, _) => value.register_or_none() == Some(&'h'),
            Instruction::Jnz(_, _) => false,
        });
    for (i, instr) in sets_h {
        eprintln!("line {} sets 'h' to {:?}", i + 1, instr.get_read_value());
    }
}

#[derive(Debug, Clone, Copy)]
enum Value {
    Number(i32),
    Register(char),
}

impl FromStr for Value {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let first = s.chars().next().unwrap();
        if first.is_alphabetic() {
            Ok(Value::Register(first))
        } else {
            Ok(Value::Number(s.parse::<i32>()?))
        }
    }
}

impl Value {
    fn get_value(&self, registers: &Registers) -> i32 {
        match self {
            Value::Number(num) => *num,
            Value::Register(c) => (*registers).get_register_value(c),
        }
    }

    fn register_or_panic<F>(&self, run: F)
    where
        F: FnOnce(&char),
    {
        match self {
            Value::Number(_) => panic!("Not a register!"),
            Value::Register(c) => run(c),
        }
    }

    fn register_or_none(&self) -> Option<&char> {
        match self {
            Value::Number(_) => None,
            Value::Register(c) => Some(c),
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Set(Value, Value),
    Sub(Value, Value),
    Mul(Value, Value),
    Jnz(Value, Value),
}

impl Instruction {
    fn parse(s: &str, val1: Value, val2: Value) -> Result<Self, String> {
        match s {
            "set" => Ok(Self::Set(val1, val2)),
            "sub" => Ok(Self::Sub(val1, val2)),
            "mul" => Ok(Self::Mul(val1, val2)),
            "jnz" => Ok(Self::Jnz(val1, val2)),
            other => Err(format!("Unrecognised instruction: {other}")),
        }
    }

    fn get_read_value(&self) -> &Value {
        match self {
            Instruction::Set(_, value1) => value1,
            Instruction::Sub(_, value1) => value1,
            Instruction::Mul(_, value1) => value1,
            Instruction::Jnz(value1, _) => value1,
        }
    }
}

#[derive(Debug, Clone, Default)]
struct Registers {
    registers: [i32; 8],
    num_mults: usize,
}

impl Display for Registers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for (i, c) in ('a'..='h').enumerate() {
            write!(f, "{c}: {}, ", self.registers[i])?;
        }
        Ok(())
    }
}

impl Registers {
    fn get_register_index(c: &char) -> usize {
        match c {
            'a'..='h' => *c as usize - 'a' as usize,
            _ => panic!("Register '{c}' is not defined"),
        }
    }

    fn get_register_value(&self, c: &char) -> i32 {
        self.registers[Self::get_register_index(c)]
    }

    fn set_register_value(&mut self, c: &char, val: i32) {
        self.registers[Self::get_register_index(c)] = val;
    }

    fn apply_instruction(&mut self, instruction: &Instruction) -> i32 {
        match instruction {
            Instruction::Set(value, value1) => {
                value.register_or_panic(|c| self.set_register_value(c, value1.get_value(self)))
            }
            Instruction::Sub(value, value1) => value.register_or_panic(|c| {
                self.set_register_value(c, self.get_register_value(c) - value1.get_value(self))
            }),
            Instruction::Mul(value, value1) => {
                value.register_or_panic(|c| {
                    self.set_register_value(c, self.get_register_value(c) * value1.get_value(self))
                });
                self.num_mults += 1;
            }
            Instruction::Jnz(value, value1) => {
                if value.get_value(self) != 0 {
                    return value1.get_value(self);
                }
            }
        }
        1
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_ascii() {
        assert_eq!('a' as i32, 97);
        assert_eq!('b' as i32 - 'a' as i32, 1);
        assert_eq!('h' as i32 - 'a' as i32, 7);
    }
}
