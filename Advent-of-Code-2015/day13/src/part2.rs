use std::collections::HashMap;
use regex::Regex;
use itertools::Itertools;

fn parse_input(input: &Vec<String>) -> (HashMap<(String, String), i64>, Vec<String>) {

    let mut map = HashMap::new();
    let mut names = vec![];
    let r = Regex::new(r"^(\w+) [\w\s]+ (gain|lose) (\d+)[\w\s]+ (\w+).$").unwrap();

    for line in input {
        let caps = r.captures(&line).unwrap();
        let person = caps.get(1).unwrap().as_str().to_string();
        let gain = caps.get(2).unwrap().as_str() == "gain";
        let mut value = caps.get(3).unwrap().as_str().parse::<i64>().unwrap();
        let other = caps.get(4).unwrap().as_str().to_string();

        if !gain {
            value = -value;
        }

        map.insert((person.clone(), other), value);
        if !names.contains(&person) {
            names.push(person);
        }

    }

    let me = String::from("Me");
    for name in names.iter() {
        map.insert((name.clone(), me.clone()), 0);
        map.insert((me.clone(), name.clone()), 0);
    }

    names.push(me);

    return (map, names);

}

fn get_score(permutation: Vec<String>, map: &HashMap<(String, String), i64>) -> i64 {
    let n = permutation.len();
    let mut score = 0;

    for i in 0..n {
        score += map.get(&(permutation[i].clone(), permutation[(i+1) % n].clone())).unwrap();
        score += map.get(&(permutation[(i+1) % n].clone(), permutation[i].clone())).unwrap();
    }

    score
}

pub fn get_max(map: &HashMap<(String, String), i64>, names: Vec<String>) -> i64 {

    let mut max = 0;
    let n = names.len();
    
    for permutation in names.into_iter().permutations(n) {
        let score = get_score(permutation, map);

        if score > max {
            max = score;
        }
    }

    return max;

}


pub fn part2(input: &Vec<String>) -> i64 {
    let (map, names) = parse_input(input);

    return get_max(&map, names);
}