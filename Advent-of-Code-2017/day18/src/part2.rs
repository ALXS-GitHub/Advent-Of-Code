use regex::Regex;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone)]
enum Instruction {
    SEND(Value),
    SET(Value, Value),
    ADD(Value, Value),
    MUL(Value, Value),
    MOD(Value, Value),
    RECEIVE(Value),
    JUMP(Value, Value),
}

#[derive(Debug, Clone)]
enum Value {
    CHAR(char),
    INT(i64),
}

#[derive(Debug, Clone)]
struct Program {
    registers: HashMap<char, i64>,
    cursor: usize,
    send_queue: VecDeque<i64>,
    send_counter: i64,
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
            "snd" => {
                Instruction::SEND(Value::CHAR(caps.get(2).unwrap().as_str().chars().nth(0).unwrap()))
            },
            "rcv" => {
                Instruction::RECEIVE(Value::CHAR(caps.get(2).unwrap().as_str().chars().nth(0).unwrap()))
            },
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
            "add" => {
                let a = caps.get(2).unwrap().as_str().trim();
                let b = caps.get(3).unwrap().as_str().trim();

                let a = Value::CHAR(a.chars().collect::<Vec<_>>()[0]);
                let b = match b.chars().any(|c| c.is_digit(10)) {
                    true => Value::INT(b.parse::<i64>().unwrap()),
                    false => Value::CHAR(b.chars().collect::<Vec<_>>()[0])
                };
                
                Instruction::ADD(a, b)
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
            "mod" => {
                let a = caps.get(2).unwrap().as_str().trim();
                let b = caps.get(3).unwrap().as_str().trim();

                let a = Value::CHAR(a.chars().collect::<Vec<_>>()[0]);
                let b = match b.chars().any(|c| c.is_digit(10)) {
                    true => Value::INT(b.parse::<i64>().unwrap()),
                    false => Value::CHAR(b.chars().collect::<Vec<_>>()[0])
                };
                
                Instruction::MOD(a, b)
            },
            "jgz" => {
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

fn compute(instructions: &mut Vec<Instruction>, programs: &mut HashMap<i64, Rc<RefCell<Program>>>, program_id: i64) {

    let mut program = programs.get(&program_id).unwrap().borrow_mut();
    let n = instructions.len();

    while program.cursor < n {

        let instruction = &instructions[program.cursor].clone();

        match instruction {

            Instruction::JUMP(r, o) => {
                if let Value::CHAR(name) = r {
                    // println!("{name}");
                    let v = *program.registers.get_mut(name).unwrap();
                    if v > 0 {
    
                        if let Value::INT(offset) = o {
                            program.cursor = (program.cursor as i64 + *offset) as usize;
                        } else if let Value::CHAR(offset) = o {
                            let ov = *program.registers.get(offset).unwrap();
                            program.cursor = (program.cursor as i64 + ov) as usize;
                        }
                        continue;
                    }
                } else if let Value::INT(v) = r {
                    if *v > 0 {
                        if let Value::INT(offset) = o {
                            program.cursor = (program.cursor as i64 + *offset) as usize;
                        } else if let Value::CHAR(offset) = o {
                            let ov = *program.registers.get(offset).unwrap();
                            program.cursor = (program.cursor as i64 + ov) as usize;
                        }
                        continue;
                    }
                }              
            },
            Instruction::ADD(r, y) => {
                if let Value::CHAR(name) = r {
                    match *y {
                        Value::CHAR(name2) => {
                            let v2 = *program.registers.get(&name2).unwrap();
                            *program.registers.get_mut(name).unwrap() += v2;
                        },
                        Value::INT(v2) => {
                            *program.registers.get_mut(name).unwrap() += v2;
                        }
                    }
                }
            },
            Instruction::MUL(r, y) => {
                if let Value::CHAR(name) = r {
                    match *y {
                        Value::CHAR(name2) => {
                            let v2 = *program.registers.get(&name2).unwrap();
                            *program.registers.get_mut(name).unwrap() *= v2;
                        },
                        Value::INT(v2) => {
                            *program.registers.get_mut(name).unwrap() *= v2;
                        }
                    }
                }
            },
            Instruction::MOD(r, y) => {
                if let Value::CHAR(name) = r {
                    match *y {
                        Value::CHAR(name2) => {
                            let v2 = *program.registers.get(&name2).unwrap();
                            *program.registers.get_mut(name).unwrap() %= v2;
                        },
                        Value::INT(v2) => {
                            *program.registers.get_mut(name).unwrap() %= v2;
                        }
                    }
                }
            },
            Instruction::SET(r, y) => {
                if let Value::CHAR(name) = r {
                    match *y {
                        Value::CHAR(name2) => {
                            let v2 = *program.registers.get(&name2).unwrap();
                            *program.registers.get_mut(name).unwrap() = v2;
                        },
                        Value::INT(v2) => {
                            *program.registers.get_mut(name).unwrap() = v2;
                        }
                    }
                }
            },
            Instruction::SEND(r) => {
                if let Value::CHAR(name) = r {
                    let v = *program.registers.get(name).unwrap();
                    program.send_queue.push_back(v);
                    program.send_counter += 1;
                }
            },
            Instruction::RECEIVE(r) => {
                if let Value::CHAR(name) = r {
                    let other_program_id = if program_id == 0 { 1 } else { 0 };
                    let mut other_program = programs.get(&other_program_id).unwrap().borrow_mut();
                    if other_program.send_queue.len() == 0 && program.send_queue.len() == 0 { // if both locked on recv, it's a lock
                        if let Instruction::RECEIVE(_) = instructions[other_program.cursor] {
                            return
                        }
                    }
                    if !(other_program.send_queue.len() > 0) {
                        drop(other_program); // release locks
                        drop(program);
                        compute(instructions, programs, other_program_id);
                        program = programs.get(&program_id).unwrap().borrow_mut();
                        other_program = programs.get(&other_program_id).unwrap().borrow_mut();
                    }
                    // if still empty -> deadlock
                    if !(other_program.send_queue.len() > 0) {
                        return
                    }
                    let v = program.registers.get_mut(name).unwrap();
                    let rcv = other_program.send_queue.pop_front().unwrap();
                    *v = rcv;
                }
            },
        }

        program.cursor += 1;

    }
}

pub fn part2(input: &Vec<String>) -> i64 {
    let (mut instructions, registers) = parse_input(input);

    let mut programs = HashMap::new();
    programs.insert(0, Rc::new(RefCell::new(Program { registers: registers.clone(), cursor: 0, send_queue: VecDeque::new(), send_counter: 0 })));
    programs.insert(1, Rc::new(RefCell::new(Program { registers: registers.clone(), cursor: 0, send_queue: VecDeque::new(), send_counter: 0 })));
    let p1 = programs.get(&1).unwrap();
    *p1.borrow_mut().registers.get_mut(&'p').unwrap() = 1;

    compute(&mut instructions, &mut programs, 0);

    let p1 = programs.get(&1).unwrap();
    let res = p1.borrow().send_counter;

    return res;
}