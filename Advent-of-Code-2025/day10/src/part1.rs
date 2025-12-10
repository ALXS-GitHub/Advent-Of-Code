use regex::Regex;
use std::collections::{VecDeque, HashSet};

#[derive(Debug)]
pub struct Machine {
    // pub lights: Vec<bool>,
    pub goal_lights: Vec<bool>,
    pub buttons: Vec<Vec<usize>>,
    pub joltage: Vec<usize>,
}

pub fn parse_input(input: &Vec<String>) -> Vec<Machine> {

    let light_re = Regex::new(r"\[([.#]+)\]").unwrap();
    let button_re = Regex::new(r"\(([\d,]+)\)").unwrap();
    let joltage_re = Regex::new(r"\{([\d,]+)\}").unwrap();

    let mut machines: Vec<Machine> = Vec::new();

    for line in input {
        let light_caps = light_re.captures(line).unwrap();
        let light_str = &light_caps[1];
        let goal_lights: Vec<bool> = light_str.chars().map(|c| c == '#').collect();
        // let lights: Vec<bool> = vec![false; goal_lights.len()];

        let mut buttons: Vec<Vec<usize>> = Vec::new();
        for button_cap in button_re.captures_iter(line) {
            let button_str = &button_cap[1];
            let mut button_vec: Vec<usize> = Vec::new();
            for num_str in button_str.split(',') {
                button_vec.push(num_str.parse::<usize>().unwrap());
            }
            buttons.push(button_vec);
        }

        let mut joltage: Vec<usize> = Vec::new();
        let joltage_caps = joltage_re.captures(line).unwrap();
        let joltage_str = &joltage_caps[1];
        for num_str in joltage_str.split(',') {
            joltage.push(num_str.parse::<usize>().unwrap());
        }

        machines.push(Machine {
            // lights,
            goal_lights,
            buttons,
            joltage,
        });
    }

    return machines;
}

pub fn bfs(machine: &Machine) -> i64 {
    
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    
    let start = vec![false; machine.goal_lights.len()];
    queue.push_back((start.clone(), 0));
    visited.insert(start);
    
    while let Some((state, presses)) = queue.pop_front() {
        if state == machine.goal_lights {
            return presses;
        }
        
        for button in &machine.buttons {
            let mut new_state = state.clone();
            for &light_idx in button {
                new_state[light_idx] = !new_state[light_idx];
            }
            
            if !visited.contains(&new_state) {
                visited.insert(new_state.clone());
                queue.push_back((new_state, presses + 1));
            }
        }
    }
    
    -1
}

pub fn part1(input: &Vec<String>) -> i64 {
    let mut machines = parse_input(input);
    let mut total = 0;

    for machine in machines.iter_mut() {
        total += bfs(machine);
    }

    return total;
}