use regex::Regex;
use std::fmt::Display;

type Grid = Vec<Vec<bool>>;

struct Instruction {
    instr: String,
    a: i32,
    b: i32,
}

fn parse_input(input: &Vec<String>) -> Vec<Instruction> {

    let mut instructions: Vec<Instruction> = Vec::new();
    let r = Regex::new(r"(rect|rotate row|rotate column)[^\d]+(\d+)[^\d]+(\d+)").unwrap();

    for line in input {
        let caps = r.captures(line).unwrap();
        instructions.push(Instruction {
            instr: caps.get(1).unwrap().as_str().to_string(),
            a: caps.get(2).unwrap().as_str().parse::<i32>().unwrap(),
            b: caps.get(3).unwrap().as_str().parse::<i32>().unwrap(),
        })
    }
    instructions
}

fn rect(a: i32, b: i32, grid: &mut Grid) {
    for i in 0..b {
        for j in 0..a {
            grid[i as usize][j as usize] = true;
        }
    }
}

fn rot_row(a: i32, b: i32, grid: &mut Grid) {
    let mut row = &grid[a as usize];
    let (v1, v2) = row.split_at(row.len() - b as usize);
    let (v1, mut v2) = (v1.to_vec(), v2.to_vec());
    v2.extend(v1);
    grid[a as usize] = v2;
}

fn rot_col(a: i32, b: i32, grid: &mut Grid) {
    let mut col: Vec<bool> = grid.iter().map(|r| r[a as usize]).collect();
    let (v1, v2) = col.split_at(col.len() - b as usize);
    let (v1, mut v2) = (v1.to_vec(), v2.to_vec());
    v2.extend(v1);
    for i in 0..grid.len() {
        grid[i][a as usize] = v2[i];
    }
}

fn follow_instructions(grid: &mut Grid, instructions: &Vec<Instruction>) {
    for instruction in instructions {
        match instruction.instr.as_str() {
            "rect" => rect(instruction.a, instruction.b, grid),
            "rotate row" => rot_row(instruction.a, instruction.b, grid),
            "rotate column" => rot_col(instruction.a, instruction.b, grid),
            _ => panic!("unknown")
        }
    }
}

fn count(grid: &Grid) -> i64 {
    let mut counter = 0;

    for r in grid {
        for c in r {
            if *c {
                counter += 1;
            }
        }
    }

    counter
}

fn display_grid(grid: &Grid) {

    for r in grid {
        for c in r {
            if *c {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

pub fn part2(input: &Vec<String>) -> String {

    let instructions = parse_input(input);
    let mut grid: Grid = vec![vec![false; 50]; 6];

    follow_instructions(&mut grid, &instructions);

    // display_grid(&grid);

    return "RURUCEOEIL".to_string()
}