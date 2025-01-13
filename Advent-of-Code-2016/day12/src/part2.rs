use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Clone)]
enum Instruction {
    COPY(Value, Value),
    INC(Value),
    DEC(Value),
    JUMP(Value, Value)
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
            "cpy" => {

                let a = caps.get(2).unwrap().as_str().trim();
                if a.chars().all(|c| c.is_digit(10)) {
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
                if a.chars().all(|c| c.is_digit(10)) {
                    Instruction::JUMP(
                        Value::INT(a.parse::<i64>().unwrap()),
                        Value::INT(caps.get(3).unwrap().as_str().trim().parse::<i64>().unwrap())
                    )
                } else {
                    Instruction::JUMP(
                        Value::CHAR(a.chars().collect::<Vec<_>>()[0]),
                        Value::INT(caps.get(3).unwrap().as_str().trim().parse::<i64>().unwrap())
                    )
                }
            },
            _ => panic!("unknown")
        };

        instructions.push(instruction)
    }

    instructions

}

fn compute(instructions: &Vec<Instruction>, registers: &mut HashMap<char, i64>) {

    let mut cursor: usize = 0;
    let n = instructions.len();

    while cursor < n {
        // println!("{}", cursor);
        // let b = *registers.get(&'c').unwrap();
        // println!("c : {}", b);

        let instruction = &instructions[cursor];

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
                            }
                            continue;
                        }
                    }
                    Value::INT(v) => {
                        if *v != 0 {
                            if let Value::INT(offset) = o {
                                cursor = (cursor as i64 + *offset) as usize;
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
            _ => {}
        }

        cursor += 1;

    }

}

pub fn part2(input: &Vec<String>) -> i64 {

    let instructions = parse_input(input);
    let mut registers = HashMap::from([
        ('a', 0),
        ('b', 0),
        ('c', 1),
        ('d', 0),
    ]);

    // println!("{:?}", instructions);

    compute(&instructions, &mut registers);

    return *registers.get(&'a').unwrap();
}