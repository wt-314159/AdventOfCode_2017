#[allow(unused_imports)]
use std::{fs, collections::HashMap, cmp::min, cmp::max};
// use fancy_regex::Regex;
// use regex::Regex;
// use md5::{Digest, Md5};
// use priority_queue::PriorityQueue;

fn main() {
    let input = fs::read_to_string("./inputs/puzzle_input.txt").expect("Failed to read input");
    // let input = fs::read_to_string("./inputs/test_puzzle_input.txt").expect("Failed to read test input");
    println!("{:?}", input);
    println!("Input lenght: {}", input.len());

    part_two(&input);
}

#[allow(dead_code)]
fn part_one(input: &str) {
    let mut count = 0;
    for line in input.split("\n") {
        if passphrase_valid(line) {
            count += 1;
        }
    }
    println!("Valid passphrases: {}", count);
}

#[allow(dead_code)]
fn part_two(input: &str) {
    let mut count = 0;
    for line in input.split("\n") {
        if passphrase_valid_anagram(line) {
            count += 1;
        }
    }   
    println!("Valid passphrases: {}", count);
}

fn passphrase_valid(pass: &str) -> bool {
    let mut words = HashMap::new();
    for word in pass.split_whitespace() {
        if words.contains_key(word) {
            return false;
        }
        words.insert(word, 1);
    }
    true
}

fn passphrase_valid_anagram(pass: &str) -> bool {
    let words: Vec<Word> = pass.split_whitespace()
        .map(|w| Word::new(w))
        .collect();

    for i in 0..words.len() {
        for j in i+1..words.len() {
            if is_anagram(&words[i], &words[j]) {
                return false;
            }
        }
    }
    true
}

fn is_anagram(word1: &Word, word2: &Word) -> bool {
    if word1.word.len() != word2.word.len() {
        return false;
    }
    for (c, count) in word1.characters.iter() {
        if word2.characters.get(c) != Some(count) {
            return false;
        }
    }
    true
}

struct Word<'a> {
    word: &'a str,
    characters: HashMap<char, i32>
}

impl<'a> Word<'a> {
    fn new(word: &str) -> Word {
        let mut characters: HashMap<char, i32> = HashMap::new();
        for c in word.chars() {
            characters.entry(c).and_modify(|count| *count += 1).or_insert(1);
        }
        Word { word, characters}
    }
}