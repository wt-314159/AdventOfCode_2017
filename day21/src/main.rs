use anyhow::*;
use itertools::iproduct;
#[allow(unused_imports)]
use std::{cmp::max, cmp::min, collections::HashMap, fs};
use std::{fmt::Display, str::FromStr, time::Instant};
// use fancy_regex::Regex;
// use regex::Regex;
// use md5::{Digest, Md5};
// use priority_queue::PriorityQueue;

fn main() {
    let input = include_str!("../inputs/puzzle_input.txt");
    let start = ".#./..#/###";
    // let input = include_str!("../inputs/test_puzzle_input.txt");
    // println!("{:?}", input);
    println!("Input length: {}", input.len());

    part_two(&input, start);
}

#[allow(dead_code)]
fn part_one(input: &str, start: &str) {
    count_bits_after_iterations(input, start, 5);
}

#[allow(dead_code)]
fn part_two(input: &str, start: &str) {
    count_bits_after_iterations(input, start, 18);
}

fn count_bits_after_iterations(input: &str, start: &str, iterations: usize) {
    let start_time = Instant::now();

    let mut pattern = start
        .parse::<Pattern>()
        .expect("Failed to parse start pattern");
    println!("{pattern}");
    let rules: Vec<Rule> = input
        .lines()
        .map(|l| l.parse::<Rule>().expect("Failed to parse input rules"))
        .collect();

    for _ in 0..iterations {
        pattern.enhance(&rules);
    }

    let elapsed = Instant::now() - start_time;
    if pattern.0.len() < 60 {
        eprintln!("Pattern aftern enhancing: ");
        eprintln!("{pattern}");
    }


    println!("{} bits on after enhancing, found in {elapsed:?}", pattern.count_bits());
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Rotation {
    Zero,
    Quarter,
    Half,
    ThreeQuarters,
}

impl Display for Rotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Rotation::Zero => "0",
                Rotation::Quarter => "90",
                Rotation::Half => "180",
                Rotation::ThreeQuarters => "270",
            }
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Flip {
    None,
    Vertical,
    Horizontal,
}

impl Display for Flip {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Flip::None => "None",
                Flip::Vertical => "Vertical",
                Flip::Horizontal => "Horizontal",
            }
        )
    }
}

fn orientations() -> impl Iterator<Item = (Rotation, Flip)> {
    iproduct!(
        [
            Rotation::Zero,
            Rotation::Quarter,
            Rotation::Half,
            Rotation::ThreeQuarters
        ],
        [Flip::None, Flip::Vertical, Flip::Horizontal]
    )
    .filter(|(r, f)| {
        !((*r == Rotation::Half && *f != Flip::None)
            || (*r == Rotation::ThreeQuarters && *f != Flip::None))
    })
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Pattern(Vec<Vec<bool>>);

impl Display for Pattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in &self.0 {
            for p in line {
                let c = match p {
                    true => '#',
                    false => '.',
                };
                write!(f, "{c}")?;
            }
            writeln!(f, "")?;
        }
        std::result::Result::Ok(())
    }
}

impl FromStr for Pattern {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pixels = s
            .split('/')
            .map(|row| {
                row.trim()
                    .chars()
                    .map(|c| match c {
                        '#' => Ok(true),
                        '.' => Ok(false),
                        other => Err(anyhow!("Failed to parse '{other}'")),
                    })
                    .collect::<Result<Vec<bool>>>()
            })
            .collect::<Result<Vec<Vec<bool>>>>()?;
        Ok(Self(pixels))
    }
}

impl Pattern {
    fn count_bits(&self) -> usize {
        self.0.iter().map(|r| r.iter().filter(|b| **b).count()).sum()
    }

    fn enhance(&mut self, rules: &Vec<Rule>) {
        let split_patterns = self.split();

        let new_patterns = split_patterns
            .iter()
            .map(|r| {
                r.iter()
                    .map(|p| {
                        for r in rules {
                            if let Some(output) = r.matches(p) {
                                return output;
                            }
                        }
                        panic!("No match found for pattern {p}")
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        self.0 = Self::combine_patterns(new_patterns).0;
    }

    fn combine_patterns(patterns: Vec<Vec<&Pattern>>) -> Pattern {
        let num_inner_rows = patterns[0][0].0.len();
        let num_inner_cols = patterns[0][0].0[0].len();
        let split_rows = patterns.len();
        let split_cols = patterns[0].len();
        let new_pat_rows = split_rows * num_inner_rows;
        let new_pat_cols = split_cols * num_inner_cols;

        let mut new_pat_vec = Vec::new();
        for r in 0..new_pat_rows {
            let mut new_pat_row = Vec::new();
            let outer_row = r / num_inner_rows;
            let inner_row = r - (outer_row * num_inner_rows);
            for j in 0..new_pat_cols {
                let outer_col = j / num_inner_cols;
                let inner_col = j - (outer_col * num_inner_cols);

                new_pat_row.push(patterns[outer_row][outer_col].0[inner_row][inner_col]);
            }
            new_pat_vec.push(new_pat_row);
        }
        Pattern(new_pat_vec)
    }

    fn split(&self) -> Vec<Vec<Pattern>> {
        let chunk_size = if self.0.len() % 2 == 0 { 2 } else { 3 };
        (0..self.0.len())
            .step_by(chunk_size)
            .map(move |r_c| {
                (0..self.0.len())
                    .step_by(chunk_size)
                    .map(move |c_c| {
                        Pattern(
                            (0..chunk_size)
                                .map(move |r| {
                                    let row = r_c + r;
                                    (0..chunk_size)
                                        .map(move |c| self.0[row][c_c + c])
                                        .collect::<Vec<_>>()
                                })
                                .collect::<Vec<_>>(),
                        )
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
    }

    #[allow(dead_code)]
    fn matches(&self, pattern: &Pattern, (rotation, flip): (Rotation, Flip)) -> bool {
        let len = pattern.0.len();
        if len != self.0.len() {
            return false;
        }
        assert_eq!(len, pattern.0[0].len());
        assert_eq!(len, self.0[0].len());

        for (rule_row, rule_col, pat_row, pat_col) in get_translated_ranges(len, (rotation, flip)) {
            if pattern.0[pat_row][pat_col] != self.0[rule_row][rule_col] {
                return false;
            }
        }
        true
    }

    fn direct_match(&self, pattern: &Pattern) -> bool {
        let len = pattern.0.len();
        if len != self.0.len() {
            return false;
        }

        for i in 0..len {
            for j in 0..len {
                if pattern.0[i][j] != self.0[i][j] {
                    return false;
                }
            }
        }
        true
    }

    fn generate_translated_patterns(&self) -> Vec<Pattern> {
        let mut translated = Vec::new();
        let len = self.0.len();
        let ranges = orientations().map(|o| get_translated_ranges(len, o));
        for mut range in ranges {
            let mut pattern = Vec::new();
            for _ in 0..len {
                let mut row = Vec::new();
                for _ in 0..len {
                    let origin = range.next().unwrap();
                    row.push(self.0[origin.0][origin.1]);
                }
                pattern.push(row);
            }
            let pattern = Pattern(pattern);
            translated.push(pattern);
        }

        translated
    }
}

fn get_translated_ranges(
    len: usize,
    (rotation, flip): (Rotation, Flip),
) -> impl Iterator<Item = (usize, usize, usize, usize)> {
    let (r_range, c_range): (Vec<usize>, Vec<usize>) = match flip {
        Flip::Vertical => ((0..len).rev().collect(), (0..len).collect()),
        Flip::Horizontal => ((0..len).collect(), (0..len).rev().collect()),
        _ => ((0..len).collect(), (0..len).collect()),
    };
    match rotation {
        Rotation::Zero => r_range
            .into_iter()
            .enumerate()
            .map(|(i, r)| {
                c_range
                    .clone()
                    .into_iter()
                    .enumerate()
                    .map(move |(j, c)| (r, c, i, j))
            })
            .flatten()
            .collect::<Vec<_>>()
            .into_iter(),
        Rotation::Quarter => c_range
            .into_iter()
            .enumerate()
            .map(|(i, c)| {
                r_range
                    .clone()
                    .into_iter()
                    .rev()
                    .enumerate()
                    .map(move |(j, r)| (r, c, i, j))
            })
            .flatten()
            .collect::<Vec<_>>()
            .into_iter(),
        Rotation::Half => r_range
            .into_iter()
            .rev()
            .enumerate()
            .map(|(i, r)| {
                c_range
                    .clone()
                    .into_iter()
                    .rev()
                    .enumerate()
                    .map(move |(j, c)| (r, c, i, j))
            })
            .flatten()
            .collect::<Vec<_>>()
            .into_iter(),
        Rotation::ThreeQuarters => c_range
            .into_iter()
            .rev()
            .enumerate()
            .map(|(i, c)| {
                r_range
                    .clone()
                    .into_iter()
                    .enumerate()
                    .map(move |(j, r)| (r, c, i, j))
            })
            .flatten()
            .collect::<Vec<_>>()
            .into_iter(),
    }
}

struct Rule {
    input_patterns: Vec<Pattern>,
    output_pattern: Pattern,
}

impl FromStr for Rule {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut split = s.split("=>");
        let (input_pattern, output_pattern) = (
            split
                .next()
                .ok_or_else(|| anyhow!("Failed to parse '{s}'"))?
                .parse::<Pattern>()?,
            split
                .next()
                .ok_or_else(|| anyhow!("Failed to parse '{s}'"))?
                .parse::<Pattern>()?,
        );
        let input_patterns = input_pattern.generate_translated_patterns();
        Ok(Rule {
            input_patterns,
            output_pattern,
        })
    }
}

impl Rule {
    fn matches(&self, other: &Pattern) -> Option<&Pattern> {
        // rotate, flip etc the inupt pattern and see if any matches other
        for pat in &self.input_patterns {
            if pat.direct_match(other) {
                return Some(&self.output_pattern);
            }
        }
        None
    }
}

#[allow(unused_assignments)]
#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use crate::*;

    macro_rules! range_2d(
        ($($t:tt),+) => {{
            let mut range = Vec::new();
            let mut count = 0;
            let mut row = 0;
            let mut col = 0;
            $(
                range.push(($t.0, $t.1, row, col));
                count += 1;
                col += 1;
                if count % 2 == 0 {
                    row += 1;
                    col = 0;
                }
            )+
            range
        }}
    );

    macro_rules! pattern{
        (#) => { true };
        (.) => { false };
        ($(( $( $b:tt )+ )),+) => {{
            let mut pixels = Vec::new();
            $(
                let mut row = Vec::new();
                $(
                    row.push(pattern!($b));
                )+
                pixels.push(row);
            )+
            Pattern(pixels)
        }};
    }

    // 1 2
    // 3 4

    // 3 1
    // 4 2
    #[test]
    fn quarter_rotation_range() {
        let expected = range_2d!((1, 0), (0, 0), (1, 1), (0, 1));
        assert_eq!(
            expected,
            get_translated_ranges(2, (Rotation::Quarter, Flip::None)).collect::<Vec<_>>()
        );
    }

    // 4 3
    // 2 1
    #[test]
    fn half_rotation_range() {
        let expected = range_2d!((1, 1), (1, 0), (0, 1), (0, 0));
        assert_eq!(
            expected,
            get_translated_ranges(2, (Rotation::Half, Flip::None)).collect::<Vec<_>>()
        );
    }

    // 2 4
    // 1 3
    #[test]
    fn three_quarter_rotation_range() {
        let expected = range_2d!((0, 1), (1, 1), (0, 0), (1, 0));
        assert_eq!(
            expected,
            get_translated_ranges(2, (Rotation::ThreeQuarters, Flip::None)).collect::<Vec<_>>()
        );
    }

    // 2 1
    // 4 3
    #[test]
    fn horizontal_flip_range() {
        let expected = range_2d!((0, 1), (0, 0), (1, 1), (1, 0));
        assert_eq!(
            expected,
            get_translated_ranges(2, (Rotation::Zero, Flip::Horizontal)).collect::<Vec<_>>()
        );
    }

    // 3 4
    // 2 1
    #[test]
    fn vertical_flip_range() {
        let expected = range_2d!((1, 0), (1, 1), (0, 0), (0, 1));
        assert_eq!(
            expected,
            get_translated_ranges(2, (Rotation::Zero, Flip::Vertical)).collect::<Vec<_>>()
        );
    }

    // 1 3
    // 2 4
    #[test]
    fn flip_and_rotate_range() {
        let expected = range_2d!((0, 0), (1, 0), (0, 1), (1, 1));
        assert_eq!(
            expected,
            get_translated_ranges(2, (Rotation::Quarter, Flip::Vertical)).collect::<Vec<_>>()
        )
    }

    #[test]
    fn any_duplicate_ranges() {
        let mut map: HashMap<Vec<(usize, usize, usize, usize)>, (Rotation, Flip)> = HashMap::new();
        for (rotation, flip) in orientations() {
            let range = get_translated_ranges(2, (rotation, flip)).collect::<Vec<_>>();
            if map.contains_key(&range) {
                eprintln!(
                    "Orientations: ({}, {}) and ({}, {}) both produce range {:?}",
                    map[&range].0,
                    map[&range].1,
                    rotation,
                    flip,
                    range.iter().map(|(r, c, _, _)| (r, c)).collect::<Vec<_>>()
                );
                assert!(false);
            }
            map.insert(range, (rotation, flip));
        }
    }

    // #..#
    // ....
    // ....
    // #..#
    #[test]
    fn test_pattern_split() {
        let pattern = pattern!(
            (# . . #),
            (. . . .),
            (. . . .),
            (# . . #));
        let split = pattern.split();

        let top_left = pattern!(
            (# .),
            (. .)
        );
        let top_right = pattern!(
            (. #),
            (. .)
        );
        let bot_left = pattern!(
            (. .),
            (# .)
        );
        let bot_right = pattern!(
            (. .),
            (. #)
        );
        assert_eq!(split[0][0], top_left);
        assert_eq!(split[0][1], top_right);
        assert_eq!(split[1][0], bot_left);
        assert_eq!(split[1][1], bot_right);
    }

    // # . | . #
    // . . | . .
    // ----+----
    // . . | . .
    // # . | . #
    #[test]
    fn test_pattern_combine() {
        let top_left = pattern!(
            (# .),
            (. .)
        );
        let top_right = pattern!(
            (. #),
            (. .)
        );
        let bot_left = pattern!(
            (. .),
            (# .)
        );
        let bot_right = pattern!(
            (. .),
            (. #)
        );

        let expected = pattern!(
            (# . . #),
            (. . . .),
            (. . . .),
            (# . . #));
        println!("{expected}");
        println!("===========");
        let combined = Pattern::combine_patterns(vec![
            vec![&top_left, &top_right],
            vec![&bot_left, &bot_right],
        ]);
        println!("{combined}");

        assert_eq!(expected, combined);
    }

    #[test]
    fn test_enhance() {
        let rules = 
r#"../.# => ##./#../...
.#./..#/### => #..#/..../..../#..#"#
            .lines()
            .map(|l| l.parse::<Rule>().unwrap())
            .collect::<Vec<_>>();
        let mut start = pattern!(
            (. # .),
            (. . #),
            (# # #)
        );

        start.enhance(&rules);

        let expected = pattern!(
            (# . . #),
            (. . . .),
            (. . . .),
            (# . . #)
        );
        assert_eq!(expected, start);
        
        let expected2 = pattern!(
            (# # . # # .),
            (# . . # . .),
            (. . . . . .),
            (# # . # # .),
            (# . . # . .),
            (. . . . . .)
        );

        start.enhance(&rules);
        assert_eq!(expected2, start);
    }

    #[allow(dead_code)]
    /// Some possible permutations of ranges aren't
    /// actually valid, e.g. (0,0), (0.1), (1,1), (1,0),
    /// which corresponds to just the bottom half of
    /// the pattern being flipped horizontally.
    /// No combination of rotations or flips would
    /// result in that pattern
    fn any_missing_ranges() {
        let ranges = iproduct!(0..=1, 0..=1);
        let expected_ranges = ranges.permutations(4);

        let translated_ranges = orientations()
            .map(|(r, f)| {
                get_translated_ranges(2, (r, f))
                    .map(|(r, c, _, _)| (r, c))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        for tr in &translated_ranges {
            eprintln!("{tr:?}");
        }

        for r in expected_ranges {
            if !translated_ranges.iter().any(|tr| {
                for i in 0..tr.len() {
                    if r[i].0 != tr[i].0 || r[i].1 != tr[i].1 {
                        return false;
                    }
                }
                true
            }) {
                eprintln!("No translated range produced for {r:?}");
                assert!(false);
            }
        }
    }
}
