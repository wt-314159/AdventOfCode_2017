use regex::Regex;
#[allow(unused_imports)]
use std::{};
use std::{
    hash::Hash, num::ParseIntError, 
    ops::{Add, AddAssign, Sub, SubAssign}, 
    str::FromStr,
    collections::HashMap,
};
// use fancy_regex::Regex;
// use md5::{Digest, Md5};
// use priority_queue::PriorityQueue;

fn main() {
    let input = include_str!("../inputs/puzzle_input.txt");
    // let input = include_str!("../inputs/test_puzzle_input.txt");
    // println!("{:?}", input);
    eprintln!("Input length: {}", input.len());

    part_two(&input);
}

#[allow(dead_code)]
fn part_one(input: &str) {
    let mut simulation = input
        .parse::<Simulation>()
        .expect("Failed to parse simulation");
    let maybe = simulation.closest_after(1_000);
    eprintln!("Particle {} maybe stays closest", maybe.id);
}

#[allow(dead_code)]
fn part_two(input: &str) {
    let steps = 10_000;
    let mut simulation = input
        .parse::<Simulation>()
        .expect("Failed to parse simulation");
    let num_remaining = simulation.find_left_particles(steps);
    eprintln!("{num_remaining} particles remaining after {steps} steps");
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd)]
struct Coord {
    x: i64,
    y: i64,
    z: i64,
}

impl FromStr for Coord {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // trim starting and trailing < and >
        let str = &s[1..s.len() - 1];
        let mut parts = str.split(',');

        let x = parts.next().expect("Missing 'x' part").parse::<i64>()?;
        let y = parts.next().expect("Missing 'y' part").parse::<i64>()?;
        let z = parts.next().expect("Missing 'z' part").parse::<i64>()?;

        Ok(Coord { x, y, z })
    }
}

impl Add for Coord {
    type Output = Coord;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign for Coord {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub for Coord {
    type Output = Coord;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl SubAssign for Coord {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Ord for Coord {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.manhattan_distance().cmp(&other.manhattan_distance())
    }
}

impl Hash for Coord {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        (self.x, self.y, self.z).hash(state);
    }
}

impl Coord {
    fn manhattan_distance(&self) -> i64 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Particle {
    id: usize,
    position: Coord,
    velocity: Coord,
    acceleration: Coord,
}

impl FromStr for Particle {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let regex = Regex::new(r#"<-?[0-9]+,-?[0-9]+,-?[0-9]+>"#).unwrap();
        let mut mats = regex.find_iter(s);

        let position = mats.next().expect("Missing position capture").as_str();
        let velocity = mats.next().expect("Missing velocity capture").as_str();
        let acceleration = mats.next().expect("Missing acceleration capture").as_str();

        let position = position.parse::<Coord>()?;
        let velocity = velocity.parse::<Coord>()?;
        let acceleration = acceleration.parse::<Coord>()?;

        Ok(Particle {
            id: 0,
            position,
            velocity,
            acceleration,
        })
    }
}

impl Particle {
    fn update(&mut self) {
        self.velocity += self.acceleration;
        self.position += self.velocity;
    }

    fn manhattan_distance(&self) -> i64 {
        self.position.manhattan_distance()
    }
}

struct Simulation {
    particles: Vec<Particle>,
}

impl FromStr for Simulation {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut particles = Vec::new();
        for (i, line) in s.lines().enumerate() {
            let mut particle = line.parse::<Particle>()?;
            particle.id = i;
            particles.push(particle);
        }

        Ok(Simulation { particles })
    }
}

impl Simulation {
    #[allow(dead_code)]
    fn min_acceleration(&self) -> &Particle {
        self.particles
            .iter()
            .min_by(|p1, p2| p1.acceleration.cmp(&p2.acceleration))
            .unwrap()
    }

    fn step_by(&mut self, steps: usize) {
        for _ in 0..steps {
            self.step();
        }
    }

    fn closest_after(&mut self, steps: usize) -> &Particle {
        self.step_by(steps);
        self.closest_to_origin()
    }

    #[allow(dead_code)]
    fn find_stays_closest(&mut self, step_by: usize, max_steps: usize) -> Option<Particle> {
        let mut steps: usize = 0;
        loop {
            self.step_by(step_by);
            let cur_closest = self.closest_to_origin();
            let any = self.particles.iter().any(|p| {
                p.velocity < cur_closest.velocity || p.acceleration < cur_closest.acceleration
            });
            steps += step_by;
            if !any {
                return Some(cur_closest.clone());
            }
            else if steps + step_by > max_steps {
                return None
            }
        }
    }

    fn closest_to_origin(&self) -> &Particle {
        self.particles
            .iter()
            .min_by(|p1, p2| p1.manhattan_distance().cmp(&p2.manhattan_distance()))
            .unwrap()
    }

    fn step(&mut self) {
        self.particles.iter_mut().for_each(|p| p.update());
    }

    fn find_left_particles(&mut self, steps: usize) -> usize {
        for _ in 0..steps {
            self.step();
            self.remove_collided_particles();
        }
        self.particles.len()
    }

    fn remove_collided_particles(&mut self){
        let mut map = HashMap::new();
        for p in &self.particles {
            let entry = map.entry(p.position).or_insert(0);
            *entry += 1;
        }
        self.particles.retain(|p| map[&p.position] == 1);
    }
}
