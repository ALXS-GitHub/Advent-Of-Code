use regex::Regex;
use std::cmp::max;
use itertools::Itertools;

fn parse_input(input: &Vec<String>) -> Vec<Vec<i64>> {
    let r = Regex::new(r"(\d+)\s+(\d+)\s+(\d+)").unwrap();
    let mut triangles = Vec::new();

    for line in input {
        let caps = r.captures(line).unwrap();
        let mut triangle = vec![];
        triangle.push(caps.get(1).unwrap().as_str().parse::<i64>().unwrap());
        triangle.push(caps.get(2).unwrap().as_str().parse::<i64>().unwrap());
        triangle.push(caps.get(3).unwrap().as_str().parse::<i64>().unwrap());
        triangles.push(triangle);
    }

    triangles
} 

fn is_triangle_valid(triangle: Vec<i64>) -> bool {
    let max = triangle.iter().max().unwrap();
    let max_idx = triangle.iter().position(|x| x == max).unwrap();
    let mut t = triangle.clone();
    t.remove(max_idx);

    let s = t.iter().sum::<i64>();

    if s <= *max {
        return false;
    }

    return true;
}

pub fn part1(input: &Vec<String>) -> i64 {

    let triangles = parse_input(input);

    let mut count = 0;

    for t in triangles {
        if is_triangle_valid(t) {
            count += 1;
        }
    }
    
    return count;
}