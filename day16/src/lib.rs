
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub struct Programs(pub [char; 16]);

impl Programs {
    pub fn new() -> Self {
        Programs([
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
        ])
    }

    pub fn is_start(&self) -> bool {
        for (i, c) in ('a'..='p').enumerate() {
            if self.0[i] != c {
                return false
            }
        }
        true
    }

    pub fn find_shortest_moves(&self, other: &mut Self) -> Vec<Moves> {
        let mut swaps = Vec::new();
        for (i, c) in self.0.iter().enumerate() {
            let start_index = other.0.iter().position(|t| t == c).unwrap();
            if i != start_index {
                let sw = Moves::Exchange(i, start_index);
                sw.execute_move(other);
                swaps.push(sw);
            }
        }
        assert_eq!(*self, *other);
        swaps
    }
}

impl Display for Programs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for c in self.0 {
            write!(f, "{c}")?;
        }
        Ok(())
    }
}

pub enum Moves {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}

impl Moves {
    #[inline(always)]
    pub fn execute_move(&self, programs: &mut Programs) {
        match self {
            Self::Spin(num) => {
                programs.0.rotate_right(*num);
            }
            Self::Exchange(a, b) => {
                programs.0.swap(*a, *b);
            }
            Self::Partner(a, b) => {
                let a = programs.0.iter().position(|c| c == a).unwrap();
                let b = programs.0.iter().position(|c| c == b).unwrap();
                programs.0.swap(a, b);
            }
        }
    }

    pub fn condense_moves(moves: impl Iterator<Item = Self>) -> Vec<Self> {
        let mut condensed = Vec::new();
        let mut comparison = Programs::new();
        let mut programs = Programs::new();
        let mut count = 0;
        for m in moves {
            match m {
                Moves::Partner(_, _) => {
                    if count != 0 {
                        let mut swaps = programs.find_shortest_moves(&mut comparison);
                        condensed.append(&mut swaps);
                    }
                    m.execute_move(&mut programs);
                    condensed.push(m);
                    comparison = programs.clone();
                    count = 0;
                }
                _ => {
                    m.execute_move(&mut programs);
                    count += 1;
                }
            }
        }
        condensed
    }

    pub fn parse_moves(input: &str) -> impl Iterator<Item = Moves> + use<'_> {
        input
            .split(',')
            .map(|move_str| match move_str.chars().nth(0) {
                Some('s') => {
                    Moves::Spin(move_str[1..].parse::<usize>().expect("Failed to parse int"))
                }
                Some('x') => {
                    let rest = &move_str[1..];
                    let mut split = rest.split('/');
                    let a = split
                        .next()
                        .unwrap()
                        .parse::<usize>()
                        .expect("a: failed to parse to int");
                    let b = split
                        .next()
                        .unwrap()
                        .parse::<usize>()
                        .expect("b: failed to parse to int");
                    Moves::Exchange(a, b)
                }
                Some('p') => {
                    let rest = &move_str[1..];
                    let a = rest.chars().nth(0).unwrap();
                    let b = rest.chars().nth(2).unwrap();
                    Moves::Partner(a, b)
                }
                _ => panic!("Can't parse {move_str}"),
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_condensed_moves() {
        let input = include_str!("../inputs/puzzle_input.txt");
        let mut programs = Programs::new();

        for moves in Moves::parse_moves(input) {
            moves.execute_move(&mut programs);
        }

        let mut comparison = Programs::new();
        let condensed = Moves::condense_moves(Moves::parse_moves(input));
        for moves in condensed {
            moves.execute_move(&mut comparison);
        }
        assert_eq!(programs, comparison);
    }
}
