#[allow(unused_imports)]
use std::{cmp::max, cmp::min, collections::HashMap, fs};
use std::{collections::VecDeque, str::FromStr};
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
    let instructions = Instruction::parse_input(input);
    let mut registers = vec![0; 26];
    let mut last_sound = 0;
    let mut cur_instr = 0;
    loop {
        let (halt, instr_offset) =
            instructions[cur_instr].execute_sound(&mut registers, &mut last_sound);
        if halt {
            println!("Last sound: {}", last_sound);
            break;
        }
        cur_instr = (cur_instr as i64 + instr_offset) as usize;
    }
}

#[allow(dead_code)]
fn part_two(input: &str) {
    let mut prog_0 = Program::new(0, Instruction::parse_input(input));
    let mut prog_1 = Program::new(1, Instruction::parse_input(input));
    loop {
        let executed_0 = prog_0.run(&mut prog_1);
        let executed_1 = prog_1.run(&mut prog_0);
        if executed_0 == 0 && executed_1 == 0 {
            break;
        }
    }
    println!("Program 1 sent: {}", prog_1.total_sent);
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Instruction {
    /// snd X plays a sound with a frequency equal to the value of X.
    Send(Value),
    /// set X Y sets register X to the value of Y.
    Set(usize, Value),
    /// add X Y increases register X by the value of Y.
    Add(usize, Value),
    /// mul X Y sets register X to the result of multiplying the value contained in register X by the value of Y.
    Mul(usize, Value),
    /// mod X Y sets register X to the remainder of dividing the value contained in register X by the value of Y (that is, it sets X to the result of X modulo Y).
    Mod(usize, Value),
    /// rcv X recovers the frequency of the last sound played, but only when the value of X is not zero. (If it is zero, the command does nothing.)
    Rcv(usize),
    /// jgz X Y jumps with an offset of the value of Y, but only if the value of X is greater than zero. (An offset of 2 skips the next instruction, an offset of -1 jumps to the previous instruction, and so on.)
    Jump(Value, Value),
}

impl Instruction {
    fn str_to_idx(s: &str) -> usize {
        let char = s.chars().next().unwrap();
        Self::char_to_idx(char)
    }

    fn char_to_idx(c: char) -> usize {
        match c {
            'a'..='z' => (c as u8 - b'a') as usize,
            _ => panic!("Invalid register"),
        }
    }

    fn parse_input(input: &str) -> Vec<Self> {
        let mut instructions = Vec::new();
        for line in input.lines() {
            let mut split = line.split_whitespace();
            instructions.push(match split.next() {
                Some("snd") => {
                    let value = split.next().unwrap().parse().unwrap();
                    Instruction::Send(value)
                }
                Some("set") => {
                    let register = Self::str_to_idx(split.next().unwrap());
                    let value = split.next().unwrap().parse().unwrap();
                    Instruction::Set(register, value)
                }
                Some("add") => {
                    let register = Self::str_to_idx(split.next().unwrap());
                    let value = split.next().unwrap().parse().unwrap();
                    Instruction::Add(register, value)
                }
                Some("mul") => {
                    let register = Self::str_to_idx(split.next().unwrap());
                    let value = split.next().unwrap().parse().unwrap();
                    Instruction::Mul(register, value)
                }
                Some("mod") => {
                    let register = Self::str_to_idx(split.next().unwrap());
                    let value = split.next().unwrap().parse().unwrap();
                    Instruction::Mod(register, value)
                }
                Some("rcv") => {
                    let value = Self::str_to_idx(split.next().unwrap());
                    Instruction::Rcv(value)
                }
                Some("jgz") => {
                    let value = split.next().unwrap().parse().unwrap();
                    let offset = split.next().unwrap().parse().unwrap();
                    Instruction::Jump(value, offset)
                }
                _ => panic!("Invalid instruction"),
            });
        }
        instructions
    }

    fn execute_sound(&self, regs: &mut Vec<i64>, last_sound: &mut i64) -> (bool, i64) {
        let mut halt = false;
        match self {
            Instruction::Send(c) => {
                *last_sound = c.get_value(regs);
            }
            Instruction::Set(c, v) => {
                Self::apply(c, v, regs, |e, v| *e = v);
            }
            Instruction::Add(c, v) => {
                Self::apply(c, v, regs, |e, v| *e += v);
            }
            Instruction::Mul(c, v) => {
                Self::apply(c, v, regs, |e, v| *e *= v);
            }
            Instruction::Mod(c, v) => {
                Self::apply(c, v, regs, |e, v| *e %= v);
            }
            Instruction::Rcv(v) => {
                if regs[*v] != 0 {
                    halt = true;
                }
            }
            Instruction::Jump(c, v) => {
                if c.get_value(regs) > 0 {
                    return (halt, v.get_value(regs));
                }
            }
        }
        (halt, 1)
    }

    fn apply<F>(register: &usize, value: &Value, registers: &mut Vec<i64>, mut f: F)
    where
        F: FnMut(&mut i64, i64),
    {
        let val = value.get_value(registers);
        f(&mut registers[*register], val);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Value {
    Register(usize),
    Number(i64),
}

impl Value {
    fn get_value(&self, registers: &Vec<i64>) -> i64 {
        match self {
            Value::Register(r) => registers[*r],
            Value::Number(n) => *n,
        }
    }
}

impl FromStr for Value {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.chars().next().expect("Failed to get first char") {
            'a'..='z' => Ok(Value::Register(Instruction::char_to_idx(
                s.chars().next().unwrap(),
            ))),
            _ => Ok(Value::Number(s.parse().expect("Failed to parse number"))),
        }
    }
}

struct Program {
    registers: Vec<i64>,
    instructions: Vec<Instruction>,
    cur_instr: usize,
    send_queue: VecDeque<i64>,
    total_sent: usize,
}

impl Program {
    fn new(id: i64, instructions: Vec<Instruction>) -> Self {
        let mut registers = vec![0; 26];
        let p_idx = Instruction::char_to_idx('p');
        registers[p_idx] = id;
        let send_queue = VecDeque::new();
        Program {
            registers,
            instructions,
            cur_instr: 0,
            send_queue,
            total_sent: 0,
        }
    }

    fn run(&mut self, other: &mut Program) -> usize {
        let mut executed = 0;
        loop {
            let (receive, instr_offset) = self.execute();

            if let Some(r) = receive {
                if let Some(value) = other.send_queue.pop_front() {
                    self.registers[r] = value;
                } else {
                    break;
                }
            }
            executed += 1;
            self.cur_instr = (self.cur_instr as i64 + instr_offset) as usize;
        }
        executed
    }

    fn execute(&mut self) -> (Option<usize>, i64) {
        let instr = &self.instructions[self.cur_instr];
        match instr {
            Instruction::Send(c) => {
                self.send_queue.push_back(c.get_value(&self.registers));
                self.total_sent += 1;
            }
            Instruction::Set(c, v) => {
                Instruction::apply(c, v, &mut self.registers, |e, v| *e = v);
            }
            Instruction::Add(c, v) => {
                Instruction::apply(c, v, &mut self.registers, |e, v| *e += v);
            }
            Instruction::Mul(c, v) => {
                Instruction::apply(c, v, &mut self.registers, |e, v| *e *= v);
            }
            Instruction::Mod(c, v) => {
                Instruction::apply(c, v, &mut self.registers, |e, v| *e %= v);
            }
            Instruction::Rcv(v) => {
                return (Some(*v), 1);
            }
            Instruction::Jump(c, v) => {
                if c.get_value(&mut self.registers) > 0 {
                    return (None, v.get_value(&mut self.registers));
                }
            }
        }
        (None, 1)
    }
}


#[test]
fn test_char_to_idx() {
    let mut count = 0;
    for c in 'a'..='z' {
        assert_eq!(Instruction::char_to_idx(c), count);
        count += 1;
    }
}

#[test]
#[should_panic]
fn invalid_char_to_idx_caps() {
    Instruction::char_to_idx('A');
}

#[test]
#[should_panic]
fn invalid_char_to_idx_star() {
    Instruction::char_to_idx('*');
}

#[test]
#[should_panic]
fn invalid_char_to_idx_0() {
    Instruction::char_to_idx('0');
}