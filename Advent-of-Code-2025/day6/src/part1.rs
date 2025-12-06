pub enum Operation {
    Add,
    Multiply,
}

pub fn parse_input(input: &Vec<String>) -> (Vec<Vec<i64>>, Vec<Operation>) {
    let mut grid: Vec<Vec<i64>> = Vec::new();
    let mut operators: Vec<Operation> = Vec::new();
    for (i, line) in input.iter().enumerate() {
        if i == input.len() - 1 {
            // operators
            let ops = line.split_whitespace().collect::<Vec<&str>>();
            for op in ops {
                match op {
                    "+" => operators.push(Operation::Add),
                    "*" => operators.push(Operation::Multiply),
                    _ => panic!("Unknown operator"),
                }
            }
        } else {
            let elements = line
                .split_whitespace()
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();
            grid.push(elements);
        }
    }
    return (grid, operators);
}



pub fn part1(input: &Vec<String>) -> i64 {
    let (grid, operators) = parse_input(input);
    let mut total = 0;

    for col in 0..grid[0].len() {
        let mut result = grid[0][col];
        for row in 1..grid.len() {
            match operators[col] {
                Operation::Add => result += grid[row][col],
                Operation::Multiply => result *= grid[row][col],
            }
        }
        total += result;
    }

    return total;
}