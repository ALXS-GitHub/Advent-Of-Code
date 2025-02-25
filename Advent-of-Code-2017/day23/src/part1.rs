use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Clone)]
enum Instruction {
    SET(Value, Value),
    SUB(Value, Value),
    MUL(Value, Value),
    JUMP(Value, Value),
}

#[derive(Debug, Clone)]
enum Value {
    CHAR(char),
    INT(i64),
}

fn parse_input(input: &Vec<String>) -> (Vec<Instruction>, HashMap<char, i64>) {

    let mut instructions = Vec::new();
    let mut registers = HashMap::new();

    let r = Regex::new(r"(\w+) (\w)(\s\w|\s-?\d+)?$").unwrap();

    for line in input {
        let caps = r.captures(line).unwrap();
        let instr = caps.get(1).unwrap().as_str();
        let reg = caps.get(2).unwrap().as_str().chars().nth(0).unwrap();
        if !reg.is_digit(10) {
            registers.insert(reg, 0);
        }

        let instruction = match instr {
            "set" => {
                let a = caps.get(2).unwrap().as_str().trim();
                let b = caps.get(3).unwrap().as_str().trim();

                let a = Value::CHAR(a.chars().collect::<Vec<_>>()[0]);
                let b = match b.chars().any(|c| c.is_digit(10)) {
                    true => Value::INT(b.parse::<i64>().unwrap()),
                    false => Value::CHAR(b.chars().collect::<Vec<_>>()[0])
                };

                Instruction::SET(a, b)
            },
            "sub" => {
                let a = caps.get(2).unwrap().as_str().trim();
                let b = caps.get(3).unwrap().as_str().trim();

                let a = Value::CHAR(a.chars().collect::<Vec<_>>()[0]);
                let b = match b.chars().any(|c| c.is_digit(10)) {
                    true => Value::INT(b.parse::<i64>().unwrap()),
                    false => Value::CHAR(b.chars().collect::<Vec<_>>()[0])
                };
                
                Instruction::SUB(a, b)
            },
            "mul" => {
                let a = caps.get(2).unwrap().as_str().trim();
                let b = caps.get(3).unwrap().as_str().trim();

                let a = Value::CHAR(a.chars().collect::<Vec<_>>()[0]);
                let b = match b.chars().any(|c| c.is_digit(10)) {
                    true => Value::INT(b.parse::<i64>().unwrap()),
                    false => Value::CHAR(b.chars().collect::<Vec<_>>()[0])
                };
                
                Instruction::MUL(a, b)
            },
            "jnz" => {
                let a = caps.get(2).unwrap().as_str().trim();
                let b = caps.get(3).unwrap().as_str().trim();

                let a = match a.chars().any(|c| c.is_digit(10)) {
                    true => Value::INT(a.parse::<i64>().unwrap()),
                    false => Value::CHAR(a.chars().collect::<Vec<_>>()[0])
                };
                let b = match b.chars().any(|c| c.is_digit(10)) {
                    true => Value::INT(b.parse::<i64>().unwrap()),
                    false => Value::CHAR(b.chars().collect::<Vec<_>>()[0])
                };
                
                Instruction::JUMP(a, b)
            },
            _ => panic!("unknown")
        };

        instructions.push(instruction)
    }

    (instructions, registers)

}

fn compute(instructions: &mut Vec<Instruction>, registers: &mut HashMap<char, i64>) -> i64 {

    let mut cursor: usize = 0;
    let n = instructions.len();
    let mut last_sound = -1;
    let mut mul_counter = 0;

    while cursor < n {

        let instruction = &instructions[cursor];

        // println!("{:?}", registers);
        // println!("{:?}", instruction);

        match instruction {

            Instruction::JUMP(r, o) => {
                if let Value::CHAR(name) = r {
                    let v = *registers.get_mut(name).unwrap();
                    if v != 0 {
    
                        if let Value::INT(offset) = o {
                            cursor = (cursor as i64 + *offset) as usize;
                        } else if let Value::CHAR(offset) = o {
                            let ov = *registers.get(offset).unwrap();
                            cursor = (cursor as i64 + ov) as usize;
                        }
                        continue;
                    }
                } else if let Value::INT(v) = r {
                    if *v != 0 {
                        if let Value::INT(offset) = o {
                            cursor = (cursor as i64 + *offset) as usize;
                        } else if let Value::CHAR(offset) = o {
                            let ov = *registers.get(offset).unwrap();
                            cursor = (cursor as i64 + ov) as usize;
                        }
                        continue;
                    }
                }              
            },
            Instruction::SUB(r, y) => {
                if let Value::CHAR(name) = r {
                    match *y {
                        Value::CHAR(name2) => {
                            let v2 = *registers.get(&name2).unwrap();
                            *registers.get_mut(name).unwrap() -= v2;
                        },
                        Value::INT(v2) => {
                            *registers.get_mut(name).unwrap() -= v2;
                        }
                    }
                }
            },
            Instruction::MUL(r, y) => {
                mul_counter += 1;
                if let Value::CHAR(name) = r {
                    match *y {
                        Value::CHAR(name2) => {
                            let v2 = *registers.get(&name2).unwrap();
                            *registers.get_mut(name).unwrap() *= v2;
                        },
                        Value::INT(v2) => {
                            *registers.get_mut(name).unwrap() *= v2;
                        }
                    }
                }
            },
            Instruction::SET(r, y) => {
                if let Value::CHAR(name) = r {
                    match *y {
                        Value::CHAR(name2) => {
                            let v2 = *registers.get(&name2).unwrap();
                            *registers.get_mut(name).unwrap() = v2;
                        },
                        Value::INT(v2) => {
                            *registers.get_mut(name).unwrap() = v2;
                        }
                    }
                }
            },
        }

        cursor += 1;

    }

    mul_counter
}

pub fn part1(input: &Vec<String>) -> i64 {
    let (mut instructions, mut registers) = parse_input(input);

    let res = compute(&mut instructions, &mut registers);

    return res;
}