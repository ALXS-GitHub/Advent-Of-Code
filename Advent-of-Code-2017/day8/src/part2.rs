use regex::Regex;
use reqwest::blocking::get;
use std::collections::HashMap;

enum Condition {
    GTE(String, i64),
    GT(String, i64),
    LTE(String, i64),
    LT(String, i64),
    EQ(String, i64),
    NEQ(String, i64)
}

enum Operator {
    INC(String, i64),
    DEC(String, i64)
}

struct Instruction {
    op: Operator,
    cond: Condition
}

fn parse_input(input: &Vec<String>) -> (HashMap<String, i64>, Vec<Instruction>) {

    let mut registers = HashMap::new();
    let mut instructions = Vec::new();

    let r = Regex::new(r"(\w+) (inc|dec) ([\d-]+) if (\w+) ([<>!=]+) ([\d-]+)").unwrap();

    for line in input.iter() {
        let caps = r.captures(line).unwrap();
        let reg = caps.get(1).unwrap().as_str().to_string();
        let op = caps.get(2).unwrap().as_str();
        let offset = caps.get(3).unwrap().as_str().parse::<i64>().unwrap();
        let operator = match op {
            "inc" => Operator::INC(reg.clone(), offset),
            "dec" => Operator::DEC(reg.clone(), offset),
            _ => panic!("Invalid OP")
        };
        let cd_reg = caps.get(4).unwrap().as_str().to_string();
        let cd = caps.get(5).unwrap().as_str();
        let cd_val = caps.get(6).unwrap().as_str().parse::<i64>().unwrap();
        let condition = match cd {
            "==" => Condition::EQ(cd_reg.clone(), cd_val),
            "!=" => Condition::NEQ(cd_reg.clone(), cd_val),
            "<=" => Condition::LTE(cd_reg.clone(), cd_val),
            "<" => Condition::LT(cd_reg.clone(), cd_val),
            ">=" => Condition::GTE(cd_reg.clone(), cd_val),
            ">" => Condition::GT(cd_reg.clone(), cd_val),
            _ => panic!("Invalid CD")
        };

        instructions.push(Instruction {
            op: operator,
            cond: condition
        });

        registers.insert(reg, 0);
        registers.insert(cd_reg, 0);

    }

    (registers, instructions)
}

fn execute(registers: &mut HashMap<String, i64>, instructions: &Vec<Instruction>) -> i64 {

    let mut max = i64::MIN;

    let nm = get_max_reg(registers);
    if nm > max {
        max = nm;
    }
    
    for instruction in instructions {
        let mut true_cd = false;
        match instruction.cond {
            Condition::GTE(ref reg, val) => {
                true_cd = registers[reg] >= val;
            },
            Condition::GT(ref reg, val) => {
                true_cd = registers[reg] > val;
            },
            Condition::LTE(ref reg, val) => {
                true_cd = registers[reg] <= val;
            },
            Condition::LT(ref reg, val) => {
                true_cd = registers[reg] < val;
            },
            Condition::EQ(ref reg, val) => {
                true_cd = registers[reg] == val;
            },
            Condition::NEQ(ref reg, val) => {
                true_cd = registers[reg] != val;
            }
        }

        if true_cd {
            match &instruction.op {
                Operator::INC(r, val) => {
                    let reg = registers.get_mut(r).unwrap();
                    *reg += val;

                },
                Operator::DEC(r, val) => {
                    let reg = registers.get_mut(r).unwrap();
                    *reg -= val;
                }
            }
        }

        let nm = get_max_reg(registers);
        if nm > max {
            max = nm;
        }

    }

    max

}

fn get_max_reg(registers: &HashMap<String, i64>) -> i64 {
    let mut max = i64::MIN;

    for (reg, val) in registers.iter() {
        if *val > max {
            max = *val;
        }
    }

    max
}

pub fn part2(input: &Vec<String>) -> i64 {

    let (mut registers, instructions) = parse_input(input);

    let res = execute(&mut registers, &instructions);

    return res;
}