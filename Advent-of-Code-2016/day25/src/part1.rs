use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Clone)]
enum Instruction {
    COPY(Value, Value),
    INC(Value),
    DEC(Value),
    JUMP(Value, Value),
    TOGGLE(Value),
    OUT(Value)
}

#[derive(Debug, Clone)]
enum Value {
    CHAR(char),
    INT(i64),
}

fn parse_input(input: &Vec<String>) -> Vec<Instruction> {

    let mut instructions = Vec::new();

    let r = Regex::new(r"(\w+) (-?\d+|\w)(\s\w|\s-?\d+)?").unwrap();

    for line in input {
        let caps = r.captures(line).unwrap();
        let instr = caps.get(1).unwrap().as_str();

        let instruction = match instr {
            "inc" => {
                Instruction::INC(Value::CHAR(caps.get(2).unwrap().as_str().chars().collect::<Vec<_>>()[0]))
            },
            "dec" => {
                Instruction::DEC(Value::CHAR(caps.get(2).unwrap().as_str().chars().collect::<Vec<_>>()[0]))
            },
            "tgl" => {
                let a = caps.get(2).unwrap().as_str().trim();
                if a.chars().all(|c| c.is_digit(10)) {
                    Instruction::TOGGLE(
                        Value::INT(a.parse::<i64>().unwrap()),
                    )
                } else {
                    Instruction::TOGGLE(
                        Value::CHAR(a.chars().collect::<Vec<_>>()[0]),
                    )
                }
            },
            "cpy" => {

                let a = caps.get(2).unwrap().as_str().trim();
                if a.chars().any(|c| c.is_digit(10)) {
                    Instruction::COPY(
                        Value::INT(a.parse::<i64>().unwrap()),
                        Value::CHAR(caps.get(3).unwrap().as_str().trim().chars().collect::<Vec<_>>()[0])
                    )
                } else {
                    Instruction::COPY(
                        Value::CHAR(a.chars().collect::<Vec<_>>()[0]),
                        Value::CHAR(caps.get(3).unwrap().as_str().trim().chars().collect::<Vec<_>>()[0])
                    )
                }
            },
            "jnz" => {
                let a = caps.get(2).unwrap().as_str().trim();
                let b = caps.get(3).unwrap().as_str().trim();

                let b = match b.chars().any(|c| c.is_digit(10)) {
                    true => Value::INT(b.parse::<i64>().unwrap()),
                    false => Value::CHAR(b.chars().collect::<Vec<_>>()[0])
                };

                if a.chars().all(|c| c.is_digit(10)) {
                    Instruction::JUMP(
                        Value::INT(a.parse::<i64>().unwrap()),
                        b
                    )
                } else {
                    Instruction::JUMP(
                        Value::CHAR(a.chars().collect::<Vec<_>>()[0]),
                        b
                    )
                }
            },
            "out" => {
                let a = caps.get(2).unwrap().as_str().trim();
                if a.chars().all(|c| c.is_digit(10)) {
                    Instruction::OUT(
                        Value::INT(a.parse::<i64>().unwrap()),
                    )
                } else {
                    Instruction::OUT(
                        Value::CHAR(a.chars().collect::<Vec<_>>()[0]),
                    )
                }
            }
            _ => panic!("unknown")
        };

        instructions.push(instruction)
    }

    instructions

}

fn compute(instructions: &mut Vec<Instruction>, registers: &mut HashMap<char, i64>, signal_length: &mut usize) -> bool {

    let mut cursor: usize = 0;
    let n = instructions.len();
    let mut last_signal = 1;

    while cursor < n {

        if *signal_length == 0 {
            return true
        }

        let instruction = &instructions[cursor];

        // println!("cursor : {}", cursor);
        // println!("reg : {:?}", registers);
        // println!("instr : {:?}", instruction);

        match instruction {

            Instruction::COPY(a, b) => {
                match a {
                    Value::CHAR(name) => {
                        let val = *registers.get(name).unwrap();
                        match b {
                            Value::CHAR(b_name) => {
                                *registers.get_mut(b_name).unwrap() = val;
                            },
                            _ => {}
                        }
                    },
                    Value::INT(val) => {
                        match b {
                            Value::CHAR(b_name) => {
                                *registers.get_mut(b_name).unwrap() = *val;
                            },
                            _ => {}
                        }
                    }
                }
            },
            Instruction::JUMP(r, o) => {
                match r {
                    Value::CHAR(name) => {
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
                    }
                    Value::INT(v) => {
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
                }
            },
            Instruction::INC(r) => {
                match r {
                    Value::CHAR(name) => {
                        *registers.get_mut(name).unwrap() += 1;
                    }
                    _ => {}
                }
            },
            Instruction::DEC(r) => {
                match r {
                    Value::CHAR(name) => {
                        *registers.get_mut(name).unwrap() -= 1;
                    }
                    _ => {}
                }
            },
            Instruction::TOGGLE(o) => {
                let offset = match o {
                    Value::CHAR(name) => {
                        let offset = *registers.get(name).unwrap();
                        if offset + (cursor as i64) >= n as i64 || offset + (cursor as i64) < 0 {
                            cursor += 1;
                            continue
                        }
                        offset
                    },
                    Value::INT(offset) => {
                        if offset + (cursor as i64) >= n as i64 || offset + (cursor as i64) < 0 {
                            cursor += 1;
                            continue
                        }
                        *offset
                    }
                };
                let instr_idx = (offset + cursor as i64) as usize;
                let instr = &mut instructions[instr_idx];
                match instr {
                    Instruction::INC(r) => {
                        *instr = Instruction::DEC(r.clone());
                    },
                    Instruction::DEC(r) => {
                        *instr = Instruction::INC(r.clone());
                    },
                    Instruction::TOGGLE(r) => {
                        *instr = Instruction::INC(r.clone());
                    },
                    Instruction::COPY(a, b) => {
                        *instr = Instruction::JUMP(a.clone(), b.clone());
                    },
                    Instruction::JUMP(a, b) => {
                        *instr = Instruction::COPY(a.clone(), b.clone());
                    },
                    _ => {}
                }
            }, 
            Instruction::OUT(r) => {
                if let Value::CHAR(name) = r {
                    let v = *registers.get(name).unwrap();
                    if v != 0 && v != 1 {
                        continue
                    }
                    if v == last_signal {
                        return false
                    }
                    last_signal = v;
                    *signal_length -= 1;
                }
            }
            _ => {}
        }

        cursor += 1;

    }

    return false

}

pub fn part1(input: &Vec<String>) -> i64 {
    let mut instructions = parse_input(input);
    let mut signal_length = 1000; // testing for x times if the signal is alternating

    let mut a = 0;
    let res = loop {
        let mut registers = HashMap::from([
            ('a', a),
            ('b', 0),
            ('c', 0),
            ('d', 0),
        ]);
        let r = compute(&mut instructions, &mut registers, &mut signal_length);

        if r {
            break a
        }

        a += 1;
    };

    return res;
}