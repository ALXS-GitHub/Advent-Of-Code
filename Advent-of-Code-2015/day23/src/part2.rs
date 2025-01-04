use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Instruction {
    instr: String,
    register: char,
    offset: i64,
}

fn parse_input(input: &Vec<String>) -> Vec<Instruction> {
    let mut instructions = vec![];

    for line in input {
        let (instr, values) = line.split_once(" ").unwrap();

        if instr == "inc" || instr == "tpl" || instr == "hlf" {
            instructions.push(Instruction {
                instr: instr.to_string(),
                register: values.chars().collect::<Vec<_>>()[0],
                offset: 0,
            })
        } else if instr == "jmp" {
            let (sign, offset) = values.split_at(1);
            let sign = if sign == "+" { 1 } else { -1 };
            let offset = offset.parse::<i64>().unwrap();

            instructions.push(Instruction {
                instr: instr.to_string(),
                register: ' ',
                offset: offset * sign,
            })
        } else if instr == "jie" || instr == "jio" {
            let (reg, o) = values.split_once(", ").unwrap();
            let (sign, offset) = o.split_at(1);
            let sign = if sign == "+" { 1 } else { -1 };
            let offset = offset.parse::<i64>().unwrap();

            instructions.push(Instruction {
                instr: instr.to_string(),
                register: reg.chars().collect::<Vec<_>>()[0],
                offset: offset * sign,
            })
        }

    }

    return instructions;
}

fn run_code(registers: &mut HashMap<char, u64>, instructions: &Vec<Instruction>) {

    let mut cursor: i64 = 0;

    while cursor >= 0 && cursor < instructions.len() as i64 {

        let instruction = &instructions[cursor as usize];

        match instruction.instr.as_str() {
            "inc" => {
                *registers.get_mut(&instruction.register).unwrap() += 1;
                cursor+=1;
            },
            "tpl" => {
                *registers.get_mut(&instruction.register).unwrap() *= 3;
                cursor+=1;
            },
            "hlf" => {
                *registers.get_mut(&instruction.register).unwrap() /= 2;
                cursor+=1;
            },
            "jmp" => {
                cursor += instruction.offset;
            },
            "jie" => {
                if *registers.get(&instruction.register).unwrap() % 2 == 0 {
                    cursor += instruction.offset;
                } else {
                    cursor += 1;
                }
            },
            "jio" => {
                if *registers.get(&instruction.register).unwrap() == 1 {
                    cursor += instruction.offset;
                } else {
                    cursor += 1;
                }
            },
            _ => panic!("Invalid instruction")
        }

    }
}

pub fn part2(input: &Vec<String>) -> i64 {

    let mut registers: HashMap<char, u64> = HashMap::from([
        ('a', 1),
        ('b', 0)
    ]);

    let instructions = parse_input(input);

    run_code(&mut registers, &instructions);

    return *registers.get(&'b').unwrap() as i64;
}