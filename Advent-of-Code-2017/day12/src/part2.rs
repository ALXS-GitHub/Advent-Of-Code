use std::collections::{HashMap, HashSet};

fn parse_input(input: &Vec<String>) -> HashMap<i64, Vec<i64>> {
    let mut ids = HashMap::new();

    for line in input {
        let mut parts = line.split("<->");
        let id = parts.next().unwrap().trim().parse::<i64>().unwrap();
        let connections = parts.next().unwrap().trim().split(",").map(|x| x.trim().parse::<i64>().unwrap()).collect::<Vec<i64>>();
        ids.insert(id, connections);
    }
    
    ids
}

fn get_group(ids: &HashMap<i64, Vec<i64>>, id: i64, group: &mut HashSet<i64>) {

    group.insert(id);
    for connection in ids.get(&id).unwrap() {
        if group.contains(connection) {
            continue;
        }
        get_group(ids, *connection, group);
    }
}

pub fn part2(input: &Vec<String>) -> i64 {
    let ids = parse_input(input);

    let mut counter = 0;
    let mut seen = HashSet::new();

    for i in 0..ids.len() {

        if seen.contains(&(i as i64)) {
            continue
        }

        let mut group = HashSet::new();
        get_group(&ids, i as i64, &mut group);
        for id in group {
            seen.insert(id);
        }
        counter += 1;
    } 

    return counter;
}
