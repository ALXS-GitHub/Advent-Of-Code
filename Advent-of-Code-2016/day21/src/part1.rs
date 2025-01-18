use regex::Regex;

#[derive(Debug, Clone)]
enum Instruction {
    SwapIdx(usize, usize),
    SwapC(char, char),
    RotL(usize),
    RotR(usize),
    RotC(char),
    Rev(usize, usize),
    Move(usize, usize)
}

fn parse_input(input: &Vec<String>) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    let rsi = Regex::new(r"swap position (\d+) with position (\d+)").unwrap();
    let rsc = Regex::new(r"swap letter (\w) with letter (\w)").unwrap();
    let rrl = Regex::new(r"rotate left (\d+) step").unwrap();
    let rrr = Regex::new(r"rotate right (\d+) step").unwrap();
    let rrc = Regex::new(r"rotate based on position of letter (\w)").unwrap();
    let rrv = Regex::new(r"reverse positions (\d+) through (\d+)").unwrap();
    let rmv = Regex::new(r"move position (\d+) to position (\d+)").unwrap();

    for line in input.iter() {
        if let Some(caps) = rsi.captures(line) {
            instructions.push(Instruction::SwapIdx(
                caps.get(1).unwrap().as_str().parse::<usize>().unwrap(),
                caps.get(2).unwrap().as_str().parse::<usize>().unwrap(),
            ))
        } 
        else if let Some(caps) = rsc.captures(line) {
            instructions.push(Instruction::SwapC(
                caps.get(1).unwrap().as_str().chars().nth(0).unwrap(),
                caps.get(2).unwrap().as_str().chars().nth(0).unwrap(),
            ))
        } 
        else if let Some(caps) = rrl.captures(line) {
            instructions.push(Instruction::RotL(
                caps.get(1).unwrap().as_str().parse::<usize>().unwrap(),
            ))
        } 
        else if let Some(caps) = rrr.captures(line) {
            instructions.push(Instruction::RotR(
                caps.get(1).unwrap().as_str().parse::<usize>().unwrap(),
            ))
        } 
        else if let Some(caps) = rrc.captures(line) {
            instructions.push(Instruction::RotC(
                caps.get(1).unwrap().as_str().chars().nth(0).unwrap(),
            ))
        } 
        else if let Some(caps) = rrv.captures(line) {
            instructions.push(Instruction::Rev(
                caps.get(1).unwrap().as_str().parse::<usize>().unwrap(),
                caps.get(2).unwrap().as_str().parse::<usize>().unwrap(),
            ))
        } 
        else if let Some(caps) = rmv.captures(line) {
            instructions.push(Instruction::Move(
                caps.get(1).unwrap().as_str().parse::<usize>().unwrap(),
                caps.get(2).unwrap().as_str().parse::<usize>().unwrap(),
            ))
        } 
        else {
            panic!("Unknown")
        }
    }

    instructions
}

fn compute_password(password: &mut Vec<char>, instructions: Vec<Instruction>) -> String {

    for instr in instructions.into_iter() {
        match instr {
            Instruction::SwapIdx(i, j) => {
                password.swap(i, j);
            },
            Instruction::SwapC(x, y) => {
                let xi = password.iter().position(|c| c == &x).unwrap();
                let yi = password.iter().position(|c| c == &y).unwrap();
                password.swap(xi, yi);
            },
            Instruction::RotL(i) => {
                password.rotate_left(i);
            },
            Instruction::RotR(i) => {
                password.rotate_right(i);
            },
            Instruction::RotC(x) => {
                let xi = password.iter().position(|c| c == &x).unwrap();
                if xi >= 4 {
                    password.rotate_right(1);
                }
                password.rotate_right(1);
                password.rotate_right(xi);
            },
            Instruction::Rev(i, j) => {
                password[i..=j].reverse();
            },
            Instruction::Move(i, j) => {
                let x = password.remove(i);
                password.insert(j, x);
            }
        }
    }

    return password.clone().into_iter().collect::<String>();
}

pub fn part1(input: &Vec<String>) -> String {

    let instructions = parse_input(input);
    let mut password = "abcdefgh".chars().collect::<Vec<_>>();

    let password = compute_password(&mut password, instructions);

    return password;
}