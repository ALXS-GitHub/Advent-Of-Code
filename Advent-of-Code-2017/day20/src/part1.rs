use regex::Regex;

#[derive(Debug, Clone)]
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
}

#[derive(Debug, Clone)]
struct Particle {
    p: Vec3,
    v: Vec3,
    a: Vec3,
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

fn find_long_term_closest(particles: &Vec<Particle>) -> i64 {

    let mut possible_res = vec![];

    let mut min = i64::MAX;

    // first check for lowest acceleration
    for i in 0..particles.len() {
        let particle = &particles[i];
        let da = particle.a.manhattan_0();
        if da == min {
            possible_res.push(i);
        } else if da < min {
            possible_res.clear();
            min = da;
            possible_res.push(i);
        }
    }

    // if there are multiple particles with same acceleration, check for lowest velocity
    let mut min = i64::MAX;
    let mut res = 0;
    for i in possible_res {
        let particle = &particles[i];
        let dv = particle.v.manhattan_0();
        if dv < min {
            min = dv;
            res = i;
        }
    }

    res as i64
}

pub fn part1(input: &Vec<String>) -> i64 {

    let particles = parse_input(input);

    let res = find_long_term_closest(&particles);

    return res;
}