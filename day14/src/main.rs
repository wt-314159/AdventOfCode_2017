use hex;
#[allow(unused_imports)]
use std::{cmp::max, cmp::min, collections::HashMap, fs};
// use fancy_regex::Regex;
// use regex::Regex;
// use md5::{Digest, Md5};
// use priority_queue::PriorityQueue;

const NUM_VALS: usize = 256;
const NUM_ROWS: usize = 128;

macro_rules! bits {
    (0) => { false };
    (1) => { true };
    ($($t:tt),+) => {
        [$(bits!($t)),+]
    }
}

fn main() {
    let input = "jxqlasbh";
    // let input = "flqrgnkx";
    // println!("{:?}", input);
    println!("Input lenght: {}", input.len());

    // part_one(&input);
    part_two(&input);
}

#[allow(dead_code)]
fn part_one(input: &str) {
    let hashes = create_hashes(input);
    let mut set_bits = 0;
    for i in 0..NUM_ROWS {
        for j in 0..16 {
            set_bits += hashes[i][j].count_ones();
        }
    }
    println!("There are {set_bits} set bits");
}

#[allow(dead_code)]
fn part_two(input: &str) {
    let hashes = create_hashes(input);
    let mut bits = [[false; 128]; 128];
    for i in 0..NUM_ROWS {
        for j in 0..16 {
            let byte = hashes[i][j].reverse_bits();
            for b in 0..8 {
                let bit = (byte >> b & 1) != 0;
                let col = j * 8 + b;
                bits[i][col] = bit;
            }
        }
    }
    count_groups(bits);
}

fn count_groups(bits: [[bool; 128]; 128]) {
    let mut groups: HashMap<usize, Group> = HashMap::new();
    let mut group_members = [[0; 128]; 128];
    let mut group_id = 1;

    for i in 0..128 {       // row
        for j in 0..128 {   // col
            if bits[i][j] {
                // check squares above and to left
                let mut added_to_group = false;
                let mut group_above = 0;
                if i > 0 {
                    // bit above
                    group_above = group_members[i-1][j];
                    if group_above != 0 {
                        let group_above = groups.get_mut(&group_above).unwrap();
                        group_above.bits.push((i, j));
                        group_members[i][j] = group_above.id;
                        added_to_group = true;
                    }
                }
                if j > 0 {
                    // bit to left
                    let group_left = group_members[i][j-1];
                    if group_left != 0 {
                        if group_above != 0 && group_left != group_above {
                            // There's a different group above and to left, merge them
                            let group_to_merge = max(group_left, group_above);
                            let first_group = min(group_left, group_above);
                            let group_to_merge = groups.remove(&group_to_merge).unwrap();
                            let first_group = groups.get_mut(&first_group).unwrap();
                            first_group.merge(group_to_merge, &mut group_members);
                            // add current bit
                            first_group.bits.push((i, j));
                            group_members[i][j] = first_group.id;

                        } else {
                            let group_left = groups.get_mut(&group_left).unwrap();
                            group_left.bits.push((i, j));
                            group_members[i][j] = group_left.id;
                        }
                        added_to_group = true;
                    }
                }
                if !added_to_group {
                    let new_group = Group::new(group_id, (i, j));
                    group_members[i][j] = group_id;
                    groups.insert(group_id, new_group);
                    group_id += 1;
                }
            }
        }
    }

    print_groups(group_members);
    println!("There are {} distinct groups", groups.len());
}

fn print_groups(group_members: [[usize; 128]; 128]) {
    for i in 0..8 {
        for j in 0..8 {
            match group_members[i][j] {
                0 => print!(". "),
                num => print!("{num} ")
            }
        }
        println!("-");
    }
}

fn create_hashes(input: &str) -> [[u8; 16]; NUM_ROWS] {
    let mut hashes = [[0; 16]; NUM_ROWS];
    for i in 0..NUM_ROWS {
        let mut hash_input = input.to_owned().clone();
        hash_input.push_str(&format!("-{i}"));

        hashes[i] = hash(&hash_input);
    }
    hashes
}

fn hash(input: &str) -> [u8; 16] {
    let mut lengths = Vec::from(input.as_bytes());
    lengths.append(&mut vec![17, 31, 73, 47, 23]);
    let mut values = create_values();
    let mut cur_idx = 0;
    let mut skip_size = 0;

    for _ in 0..64 {
        (cur_idx, skip_size) = apply_hash(&mut values, &lengths, cur_idx, skip_size);
    }

    let dense_hash = create_dense_hash(values);
    // dense_hash_to_char_array(dense_hash)
    dense_hash
}

fn create_values() -> [u8; NUM_VALS] {
    let mut values = [0u8; NUM_VALS];
    for i in 0..NUM_VALS {
        values[i] = i as u8;
    }
    values
}

fn apply_hash(
    values: &mut [u8; NUM_VALS],
    lengths: &Vec<u8>,
    mut cur_idx: usize,
    mut skip_size: usize,
) -> (usize, usize) {
    for len in lengths {
        reverse_values(values, cur_idx, *len);
        cur_idx += (*len as usize) + skip_size;
        skip_size += 1;

        // Make sure we loop back to beginning
        while cur_idx > values.len() {
            cur_idx -= values.len();
        }
    }
    (cur_idx, skip_size)
}

fn reverse_values(values: &mut [u8; NUM_VALS], cur_idx: usize, length: u8) {
    let u_length = length as usize;
    if cur_idx + u_length < values.len() {
        values[cur_idx..cur_idx + u_length].reverse();
    } else {
        let overflow = cur_idx + u_length - values.len();
        if overflow > values.len() {
            panic!("Overflow was greater than values array size");
        }
        let mut rev_vals = Vec::from(&values[cur_idx..]);
        for i in 0..overflow {
            rev_vals.push(values[i]);
        }
        rev_vals.reverse();

        for i in 0..rev_vals.len() {
            let mut idx = cur_idx + i;
            if idx >= values.len() {
                idx -= values.len();
            }

            values[idx] = rev_vals[i];
        }
    }
}

fn create_dense_hash(values: [u8; NUM_VALS]) -> [u8; 16] {
    let mut dense_hash = [0u8; 16];
    for i in 0..16 {
        let offset = i * 16;
        dense_hash[i] = xor_values(&values[offset..offset + 16]);
    }
    dense_hash
}

fn xor_values(values: &[u8]) -> u8 {
    let mut result = values[0];
    for i in 1..values.len() {
        result ^= values[i];
    }
    result
}


#[allow(dead_code)]
fn dense_hash_to_string(dense_hash: &[u8]) -> String {
    let mut output = String::new();
    for i in 0..dense_hash.len() {
        output.push_str(&format!("{:02x}", dense_hash[i]));
    }
    output
}


#[allow(dead_code)]
fn dense_hash_to_char_array(dense_hash: [u8; 16]) -> [char; 32] {
    let mut output = ['z'; 32];
    for i in 0..16 {
        let digits = hex::encode([dense_hash[i]]);
        let idx = i * 2;
        output[idx] = digits.chars().nth(0).unwrap();
        output[idx + 1] = digits.chars().nth(1).unwrap();
    }
    output
}


#[allow(dead_code)]
fn char_to_bits(c: char) -> [bool; 4] {
    match c {
        '0' => bits![0, 0, 0, 0],
        '1' => bits![0, 0, 0, 1],
        '2' => bits![0, 0, 1, 0],
        '3' => bits![0, 0, 1, 1],
        '4' => bits![0, 1, 0, 0],
        '5' => bits![0, 1, 0, 1],
        '6' => bits![0, 1, 1, 0],
        '7' => bits![0, 1, 1, 1],
        '8' => bits![1, 0, 0, 0],
        '9' => bits![1, 0, 0, 1],
        'a' => bits![1, 0, 1, 0],
        'b' => bits![1, 0, 1, 1],
        'c' => bits![1, 1, 0, 0],
        'd' => bits![1, 1, 0, 1],
        'e' => bits![1, 1, 1, 0],
        'f' => bits![1, 1, 1, 1],
        _ => panic!("Can't convert {c} to bits!"),
    }
}

#[allow(dead_code)]
fn print_bits(bit: bool) {
    let c = match bit {
        true => '#',
        false => '.',
    };
    print!("{c}");
}


struct Group {
    id: usize,
    bits: Vec<(usize, usize)>
}

impl Group {
    fn new(id: usize, cur_bit: (usize, usize)) -> Group {
        Group { id, bits: vec![cur_bit] }
    }

    fn merge(&mut self, other: Group, grid: &mut [[usize; 128]; 128]) {
        for bit in other.bits {
            grid[bit.0][bit.1] = self.id;
            self.bits.push(bit);
        }
    }
}