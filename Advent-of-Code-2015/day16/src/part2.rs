use std::collections::{BTreeMap, HashMap};
use regex::Regex;

fn parse_input(input: &Vec<String>) -> BTreeMap<i64, HashMap<&str, i64>> {

    let mut map = BTreeMap::new();
    let r = Regex::new(r"^Sue (\d+): (\w+): (\d+), (\w+): (\d+), (\w+): (\d+)").unwrap();

    for line in input {
        let caps = r.captures(&line).unwrap();
        let n = caps[1].parse::<i64>().unwrap();
        let k1 = caps.get(2).unwrap().as_str();
        let v1 = caps[3].parse::<i64>().unwrap();
        let k2 = caps.get(4).unwrap().as_str();
        let v2 = caps[5].parse::<i64>().unwrap();
        let k3 = caps.get(6).unwrap().as_str();
        let v3 = caps[7].parse::<i64>().unwrap();

        let current = HashMap::from([
            (k1, v1),
            (k2, v2),
            (k3, v3)
        ]);

        map.insert(n, current);

    }

    return map;

}

fn match_aunt_sue(aunt_sue: &HashMap<&str, i64>, sue: HashMap<&str, i64>) -> bool {

    let mut res = true;
    for (key, value) in sue.into_iter() {
        let v = aunt_sue.get(key).unwrap();
        if key == "cats" || key == "trees" {
            if value <= *v {
                res = false
            }
        } else if key == "pomeranians" || key == "goldfish" {
            if value >= *v {
                res = false
            }
        }
        else {
            if *v != value {
                res = false;
            }
        }
    }
    return res;
}


pub fn part2(input: &Vec<String>) -> i64 {
    let aunt_sue = HashMap::from([
        ("children", 3),
        ("cats", 7),
        ("samoyeds", 2),
        ("pomeranians", 3),
        ("akitas", 0),
        ("vizslas", 0),
        ("goldfish", 5),
        ("trees", 3),
        ("cars", 2),
        ("perfumes", 1), 
    ]);

    let sues = parse_input(input);

    for (n, sue) in sues {
        if match_aunt_sue(&aunt_sue, sue) {
            return n
        }
    }

    return 0;
}