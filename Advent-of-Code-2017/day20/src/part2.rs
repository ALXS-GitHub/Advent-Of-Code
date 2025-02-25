use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Copy, Eq, Hash)]
struct Vec3 {
    x: i64,
    y: i64,
    z: i64,
}

impl Vec3 {
    fn new(x: i64, y: i64, z: i64) -> Self {
        Self {
            x, y, z
        }
    }

    fn manhattan_0(&self) -> i64 {
        return self.x.abs() + self.y.abs() + self.z.abs();
    }

    fn add(&mut self, other: Vec3) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

#[derive(Debug, Clone)]
struct Particle {
    p: Vec3,
    v: Vec3,
    a: Vec3,
}

impl Particle {
    fn update(&mut self) {
        self.v.add(self.a);
        self.p.add(self.v);
    }
}

fn parse_input(input: &Vec<String>) -> Vec<Particle> {
    let mut particles = vec![];

    let r = Regex::new(r"^p=<(-?\d+),(-?\d+),(-?\d+)>, v=<(-?\d+),(-?\d+),(-?\d+)>, a=<(-?\d+),(-?\d+),(-?\d+)>").unwrap();

    for line in input.iter() {
        let caps = r.captures(line).unwrap();
        let particle = Particle {
            p: Vec3::new(
                caps.get(1).unwrap().as_str().parse::<i64>().unwrap(),
                caps.get(2).unwrap().as_str().parse::<i64>().unwrap(),
                caps.get(3).unwrap().as_str().parse::<i64>().unwrap(),
            ),
            v: Vec3::new(
                caps.get(4).unwrap().as_str().parse::<i64>().unwrap(),
                caps.get(5).unwrap().as_str().parse::<i64>().unwrap(),
                caps.get(6).unwrap().as_str().parse::<i64>().unwrap(),
            ),
            a: Vec3::new(
                caps.get(7).unwrap().as_str().parse::<i64>().unwrap(),
                caps.get(8).unwrap().as_str().parse::<i64>().unwrap(),
                caps.get(9).unwrap().as_str().parse::<i64>().unwrap(),
            )
        };
        particles.push(particle);
    }

    particles
}

fn simulate_collisions(mut particles: Vec<Particle>) -> Vec<Particle> {
    let mut positions = HashMap::new();

    for particle in particles.iter_mut() {
        particle.update();
        let entry = positions.entry(particle.p).or_insert(0);
        *entry += 1;
    }

    particles.retain(|particle| *positions.get(&particle.p).unwrap() == 1);

    particles
}

fn find_remaining_particles(particles: &Vec<Particle>) -> i64 {
    let mut current_particles = particles.clone();

    // Simulate collisions over time
    for _ in 0..1000 {
        current_particles = simulate_collisions(current_particles);
    }

    current_particles.len() as i64
}

pub fn part2(input: &Vec<String>) -> i64 {

    let particles = parse_input(input);

    let res = find_remaining_particles(&particles);

    return res;
}