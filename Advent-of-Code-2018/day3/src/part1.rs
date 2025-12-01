use regex::Regex;
use std::collections::HashSet;

struct Claim {
    id: usize,
    x: usize,
    y: usize,
    w: usize,
    h: usize,
}

impl Claim {
    fn overlap(&self, other: &Claim) -> Option<Claim> {
        let left = self.x.max(other.x);
        let top = self.y.max(other.y);
        let right = (self.x + self.w - 1).min(other.x + other.w - 1);
        let bottom = (self.y + self.h - 1).min(other.y + other.h - 1);

        if left <= right && top <= bottom {
            Some(Claim {
                id: 0,
                x: left,
                y: top,
                w: right - left + 1,
                h: bottom - top + 1,
            })
        } else {
            None
        }
    }

    fn get_positions(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        (0..self.w).flat_map(move |x| {
            (0..self.h).map(move |y| (self.x + x, self.y + y))
        })
    }
}

fn parse_input(input: &Vec<String>) -> Vec<Claim> {

    let mut claims = Vec::new();

    let r = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();

    for line in input {
        let caps = r.captures(line).unwrap();
        claims.push(Claim {
            id: caps.get(1).unwrap().as_str().parse::<usize>().unwrap(),
            x: caps.get(2).unwrap().as_str().parse::<usize>().unwrap(),
            y: caps.get(3).unwrap().as_str().parse::<usize>().unwrap(),
            w: caps.get(4).unwrap().as_str().parse::<usize>().unwrap(),
            h: caps.get(5).unwrap().as_str().parse::<usize>().unwrap(),
        });
    }

    claims
}



pub fn part1(input: &Vec<String>) -> i64 {

    let mut claims = parse_input(&input);

    let mut positions = HashSet::new();

    for i in 0..claims.len() {
        for j in i+1..claims.len() {
            if let Some(c) = claims[i].overlap(&claims[j]) {
                positions.extend(c.get_positions());
            }
        }
    }

    return positions.len() as i64;
}