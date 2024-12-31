use std::collections::HashMap;
use regex::Regex;
use itertools::Itertools;

fn parse_input(input: &Vec<String>) -> (HashMap<String, i64>, Vec<Vec<i64>>) {

    let mut cities = HashMap::new();
    let mut idx = 0;

    let r = Regex::new(r"^(\w+) to (\w+) = (\d+)$").unwrap();

    for line in input {
        let caps = r.captures(&line).unwrap();
        let c1 = caps[1].to_string();
        let c2 = caps[2].to_string();

        if let Some(_) = cities.get(&c1) {
        } else {
            cities.insert(c1, idx);
            idx += 1;
        }

        if let Some(_) = cities.get(&c2) {
        } else {
            cities.insert(c2, idx);
            idx += 1;
        }
    }

    let mut dist_matrix = vec![vec![0; cities.len()]; cities.len()];

    for line in input {
        let caps = r.captures(&line).unwrap();
        let c1 = caps[1].to_string();
        let c2 = caps[2].to_string();
        let dist = caps[3].parse::<i64>().unwrap();

        if let (Some(i1), Some(i2)) = (cities.get(&c1), cities.get(&c2)) {
            let i1 = *i1 as usize;
            let i2 = *i2 as usize;
            dist_matrix[i1][i2] = dist;
            dist_matrix[i2][i1] = dist;
        }
    }

    (cities, dist_matrix)
}

fn get_path_distance(permutation: Vec<String>, cities: HashMap<String,i64>, dist_matrix: Vec<Vec<i64>>) -> i64 {

    let mut dist = 0;

    for i in 0..permutation.len() - 1 {
        let id1 = *cities.get(&permutation[i]).unwrap();
        let id2 = *cities.get(&permutation[i+1]).unwrap();

        dist += dist_matrix[id1 as usize][id2 as usize];
    }

    return dist;

}

fn find_longest_path(cities: HashMap<String,i64>, dist_matrix: Vec<Vec<i64>>) -> i64 {

    let mut d = 0;

    let cities_vec: Vec<String> = cities.clone().into_keys().collect();

    for permutation in cities_vec.into_iter().permutations(cities.len()) {
        let distance = get_path_distance(permutation, cities.clone(), dist_matrix.clone());

        if distance > d {
            d = distance;
        }
    }

    return d;

}

pub fn part2(input: &Vec<String>) -> i64 {
    let (cities, dist_matrix) = parse_input(input);

    let res = find_longest_path(cities, dist_matrix);

    return res;
}