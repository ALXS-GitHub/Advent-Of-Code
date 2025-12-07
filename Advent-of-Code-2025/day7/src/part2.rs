pub fn parse_input(input: &Vec<String>) -> (usize, usize) {

    let mut start: usize = 0;
    let first_line = &input[0];
    for (j, c) in first_line.chars().enumerate() {
        if c == 'S' {
            start = j;
            break;
        }
    }
    (0, start)
}

pub fn simulate_step(input: &Vec<Vec<char>>, grid: &mut Vec<Vec<i64>>, i: usize) -> i64 {
    let mut split_count = 0;
    for j in 0..input[0].len() {
        if grid[i][j] > 0 {
            match input[i+1][j] {
                '.' => {
                    grid[i+1][j] += grid[i][j];
                },
                '^' => {
                    if j > 0 {
                        grid[i+1][j-1] += grid[i][j];
                    }
                    if j < input[0].len() - 1 {
                        grid[i+1][j+1] += grid[i][j];
                    }
                    split_count += grid[i][j] as i64;
                },
                _ => panic!("Unexpected character"),
            }
            
        }
    }
    split_count
}

pub fn part2(input: &Vec<String>) -> i64 {
    let start = parse_input(input);
    let mut grid = vec![vec![0; input[0].len()]; input.len()];
    grid[start.0][start.1] = 1;
    let input: Vec<Vec<char>> = input.iter().map(|line| line.chars().collect()).collect();

    let mut total_splits = 1;

    for i in 0..input.len() - 1 {
        total_splits += simulate_step(&input, &mut grid, i);
    }
    return total_splits;
}