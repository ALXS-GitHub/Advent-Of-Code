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
        }
    }

    let mut current_group: Vec<i64> = Vec::new();
    for j in (0..input[0].len()).rev() {
        let mut current_number: Vec<char> = Vec::new();
        for i in 0..input.len() - 1 {
            let line = &input[i];
            let ch = line.chars().nth(j).unwrap();
            if !ch.is_whitespace() {
                current_number.push(ch);
            }
        }
        if !current_number.is_empty() {
            let num_str: String = current_number.iter().collect();
            let num = num_str.parse::<i64>().unwrap();
            current_group.push(num);
            current_number.clear();
        } else {
            grid.push(current_group);
            current_group = Vec::new();
            continue;
        }
    }

    if !current_group.is_empty() {
        grid.push(current_group);
    }

    grid.reverse();
    return (grid, operators);
}



pub fn part2(input: &Vec<String>) -> i64 {
    let (grid, operators) = parse_input(input);
    let mut total = 0;

    for col in 0..grid.len() {
        let mut result = grid[col][0];
        for row in 1..grid[col].len() {
            match operators[col] {
                Operation::Add => result += grid[col][row],
                Operation::Multiply => result *= grid[col][row],
            }
        }
        total += result;
    }

    return total;
}