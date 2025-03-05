#[allow(unused_imports)]
use std::{cmp::max, cmp::min, collections::HashMap, fs};
// use fancy_regex::Regex;
// use regex::Regex;
// use md5::{Digest, Md5};
// use priority_queue::PriorityQueue;

fn main() {
    let input = include_str!("../inputs/puzzle_input.txt");
    // let input = include_str!("../inputs/test_puzzle_input.txt");
    // println!("{:?}", input);
    // let input = "se,sw,se,sw,sw";
    println!("Input lenght: {}", input.len());

    part_two(&input);
    part_one(&input);
}

#[allow(dead_code)]
fn part_one(input: &str) {
    let mut distances = find_axis_distances(input);
    // Steps in each of the 3 directions cancel each other out, so find
    // minimum of the distances above, and take that away from each distance
    // let temp = dbg!([distances.0, distances.1, distances.2]);
    // let min = dbg!(temp.iter().min().unwrap());

    println!("distances: {:?}", distances);
    let shortest_path = distances.find_shortest_path();
    println!("After cancelling down: {:?}", distances);
    println!("Shortest distance: {}", shortest_path);
}

#[allow(dead_code)]
fn part_two(input: &str) {
    // HACK This would be better achieved by incrementally counting the
    // maximum distance from origin, by implementing the shortest_path
    // rules during the stage where we increment each axis, but as a
    // quick and dirty method this will work
    let mut max_distance = 0;
    let mut distances = Distances::new(0, 0, 0);
    #[allow(unused_assignments)]
    let mut temp_distances = Distances::new(0, 0, 0);
    for dir in input.split(',') {
        match dir {
            "n" => distances.n += 1,
            "s" => distances.n -= 1,
            "sw" => distances.sw += 1,
            "ne" => distances.sw -= 1,
            "se" => distances.se += 1,
            "nw" => distances.se -= 1,
            _ => panic!("Unexpected direction! {dir}"),
        }

        temp_distances = distances.clone();
        let cur_distance = temp_distances.find_shortest_path();
        if cur_distance > max_distance {
            max_distance = cur_distance;
        }
    }
    println!("Furthest distance: {}", max_distance);
}

fn find_axis_distances(input: &str) -> Distances {
    let mut distances = Distances::new(0, 0, 0);
    for dir in input.split(',') {
        match dir {
            "n" => distances.n += 1,
            "s" => distances.n -= 1,
            "sw" => distances.sw += 1,
            "ne" => distances.sw -= 1,
            "se" => distances.se += 1,
            "nw" => distances.se -= 1,
            _ => panic!("Unexpeced direction! {dir}"),
        }
    }
    distances
}

#[derive(Debug, Clone, Copy)]
struct Distances {
    n: i32,
    sw: i32,
    se: i32,
}

impl Distances {
    fn new(n: i32, sw: i32, se: i32) -> Distances {
        Distances { n, sw, se }
    }

    fn find_shortest_path(&mut self) -> i32 {
        // If axes in all 3 directions have same sign (all positive
        // or all negative) they will cancel out, find mininum (abs)
        // and remove from all 3 axes
        if (self.n > 0 && self.sw > 0 && self.se > 0) || (self.n < 0 && self.sw < 0 && self.se < 0)
        {
            let temp = [self.n.abs(), self.sw.abs(), self.se.abs()];
            let min = temp.iter().min().unwrap();

            let sign = if self.n > 0 { 1 } else { -1 };
            let num = min * sign;
            self.n -= num;
            self.se -= num;
            self.sw -= num;
        }

        Self::min_2_axes(&mut self.n, &mut self.sw, &mut self.se);
        Self::min_2_axes(&mut self.n,  &mut self.se, &mut self.sw,);
        Self::min_2_axes(&mut self.sw, &mut self.se, &mut self.n);

        self.n.abs() + self.sw.abs() + self.se.abs()
    }

    // A step north and a step south west to a step northwest (-southeast)
    // Similarly, a step south (-north) and northeast (-southwest), equates
    // to a step southeast
    fn min_2_axes(a: &mut i32, b: &mut i32, c: &mut i32) {
        if (*a > 0 && *b > 0) || (*a < 0 && *b < 0) {
            let min = min(a.abs(), b.abs());
            let sign = if *a > 0 { 1 } else { -1 };

            let num = min * sign;
            *a -= num;
            *b -= num;
            *c -= num;
        }
    }
}
