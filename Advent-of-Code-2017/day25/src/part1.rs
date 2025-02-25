use regex::Regex;
use std::collections::{HashMap, BTreeSet};

#[derive(Debug, Clone, Copy)]
struct Instruction {
    write: bool,
    mv: i32,
    next_state: char,
}

impl Instruction {
    fn new() -> Self {
        Self {
            write: false,
            mv: 0,
            next_state: ' ',
        }
    }
}

fn parse_input(input: &Vec<String>) ->(char, i32, HashMap<char, (Instruction, Instruction)>) {

    let mut instructions = HashMap::new();
    let start = Regex::new(r"in state (\w)").unwrap().captures(&input[0]).unwrap().get(1).unwrap().as_str().chars().nth(0).unwrap();
    let steps = Regex::new(r"after (\d+) steps").unwrap().captures(&input[1]).unwrap().get(1).unwrap().as_str().parse::<i32>().unwrap();

    let state_r = Regex::new(r"In state (\w)").unwrap();
    let current_value = Regex::new(r"current value is (\d)").unwrap();
    let write_value = Regex::new(r"value (\d)").unwrap();
    let mv = Regex::new(r"to the (right|left)").unwrap();
    let next_state_r = Regex::new(r"with state (\w)").unwrap();

    let mut current_state = ' ';
    let mut current_val = 0;
    let mut current_0 = Instruction::new();
    let mut current_1 = Instruction::new();

    for i in 3..input.len() {
        if input[i] == "" {
            instructions.insert(current_state, (current_0.clone(), current_1.clone()));
            continue;
        }

        if let Some(caps) = state_r.captures(&input[i]) {
            current_state = caps.get(1).unwrap().as_str().chars().nth(0).unwrap();
        } else if let Some(caps) = current_value.captures(&input[i]) {
            current_val = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
        } else if let Some(caps) = write_value.captures(&input[i]) {
            if current_val == 0 {
                current_0.write = caps.get(1).unwrap().as_str().parse::<i32>().unwrap() != 0;
            } else {
                current_1.write = caps.get(1).unwrap().as_str().parse::<i32>().unwrap() != 0;
            }
        } else if let Some(caps) = mv.captures(&input[i]) {
            if current_val == 0 {
                current_0.mv = if caps.get(1).unwrap().as_str() == "right" { 1 } else { -1 };
            } else {
                current_1.mv = if caps.get(1).unwrap().as_str() == "right" { 1 } else { -1 };
            }
        } else if let Some(caps) = next_state_r.captures(&input[i]) {
            if current_val == 0 {
                current_0.next_state = caps.get(1).unwrap().as_str().chars().nth(0).unwrap();
            } else {
                current_1.next_state = caps.get(1).unwrap().as_str().chars().nth(0).unwrap();
            }
        }

        
    }

    instructions.insert(current_state, (current_0.clone(), current_1.clone()));

    (start, steps, instructions)
}

fn solve(start: char, steps: i32, instructions: &HashMap<char, (Instruction, Instruction)>) -> i64 {

    let mut cursor = 0;
    let mut data = BTreeSet::new();
    let mut state = start;

    for _ in 0..steps {
        let instr = match data.get(&cursor) {
            Some(_) => {
                instructions.get(&state).unwrap().1
            }, 
            None => {
                instructions.get(&state).unwrap().0
            }
        };

        match instr.write {
            true => {
                data.insert(cursor);
            },
            false => {
                data.remove(&cursor);
            }
        }

        cursor += instr.mv;

        state = instr.next_state;
    }

    return data.len() as i64
}

pub fn part1(input: &Vec<String>) -> i64 {

    let (start, steps, instructions) = parse_input(input);

    let res = solve(start, steps, &instructions);

    return res;
}