use std::collections::HashMap;
use regex::Regex;

fn parse_input(input: &Vec<String>) -> HashMap<i64, Vec<i64>> {

    let mut sorted_input = input.clone();
    sorted_input.sort();

    let rguard = Regex::new(r"Guard #(\d+)").unwrap();
    let r = Regex::new(r"\[(\d{4})-(\d{2})-(\d{2}) (\d{2}):(\d{2})\] (wakes up|falls asleep)").unwrap();

    let mut map = HashMap::new();
    let mut current_set = &mut Vec::new();
    let mut last_action= 0;
    let mut last_minute= 0;

    for line in sorted_input {
        let capsguard = rguard.captures(&line);
        let caps = r.captures(&line);

        if let Some(c) = capsguard {
            let current_guard = c.get(1).unwrap().as_str().parse::<i64>().unwrap();
            current_set = map.entry(current_guard).or_insert(vec![0; 60]);
            last_action = 0;
            last_minute = 0;
        }
        else if let Some (c) = caps {
            let action = c.get(6).unwrap().as_str();
            let action = if action == "wakes up" { 0 } else { 1 };
            let minutes = c.get(5).unwrap().as_str().parse::<usize>().unwrap();
            
            if last_action == 1 {
                for i in last_minute..minutes {
                    current_set[i] += 1;
                }
            }

            last_action = action;
            last_minute = minutes;
        }

    }

    map

}

fn find_most_asleep_guard(map: &HashMap<i64, Vec<i64>>) -> i64 {
    *map.into_iter()
        .max_by_key(|(_, minutes)| minutes.iter().sum::<i64>())
        .map(|(guard_id, _)| guard_id)
        .unwrap_or(&0)
}

fn find_most_asleep_minute(vec: &Vec<i64>) -> i64 {
    let max = vec.iter().max().unwrap();
    vec.iter().position(|x| x == max).unwrap() as i64
}

pub fn part1(input: &Vec<String>) -> i64 {
    let map = parse_input(input);
    let most_asleep_guard = find_most_asleep_guard(&map);
    let most_asleep_minute = find_most_asleep_minute(map.get(&most_asleep_guard).unwrap());
    return most_asleep_guard * most_asleep_minute;
}