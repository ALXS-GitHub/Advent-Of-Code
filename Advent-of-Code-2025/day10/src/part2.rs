use regex::Regex;
use std::collections::{BinaryHeap, HashSet, VecDeque};
use std::cmp::Ordering;
use z3::ast::{Ast, Int};
use z3::{Config, Context, Optimize};

// ! Warning this file is long to compile due to Z3
// ! Make sure to only compile when necessary or only test part1 and remove the z3 dependency from Cargo.toml

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

fn solve_machine_z3(machine: &Machine) -> i64 {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let optimizer = Optimize::new(&ctx);

    // Create variables for each button (number of times we press it)
    let button_vars: Vec<Int> = (0..machine.buttons.len())
        .map(|i| Int::new_const(&ctx, format!("button_{}", i)))
        .collect();

    for button_var in &button_vars {
        optimizer.assert(&button_var.ge(&Int::from_i64(&ctx, 0)));
    }

    // Create constraints for each joltage counter
    for (counter_idx, &target_joltage) in machine.joltage.iter().enumerate() {
        let mut counter_sum = Int::from_i64(&ctx, 0);
        
        for (button_idx, button) in machine.buttons.iter().enumerate() {
            if button.contains(&counter_idx) {
                counter_sum = counter_sum + &button_vars[button_idx];
            }
        }
        
        optimizer.assert(&counter_sum._eq(&Int::from_i64(&ctx, target_joltage as i64)));
    }

    let total_presses: Int = button_vars.iter()
        .fold(Int::from_i64(&ctx, 0), |acc, var| acc + var);
    
    optimizer.minimize(&total_presses);

    // Solve
    match optimizer.check(&[]) {
        z3::SatResult::Sat => {
            let model = optimizer.get_model().unwrap();
            let result = model.eval(&total_presses, true).unwrap().as_i64().unwrap();
            result
        }
        _ => {
            -1
        }
    }
}

pub fn part2(input: &Vec<String>) -> i64 {
    let machines = parse_input(input);
    let mut total = 0;

    for (idx, machine) in machines.iter().enumerate() {
        let result = solve_machine_z3(machine);
        if result == -1 {
            continue;
        }
        total += result;
    }

    return total;
}