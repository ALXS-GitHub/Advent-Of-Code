pub fn parse_input(input: &Vec<String>) -> Vec<Vec<bool>> {
    let mut grid: Vec<Vec<bool>> = Vec::new();
    for line in input {
        let row: Vec<bool> = line
            .chars()
            .map(|c| if c == '@' { true } else { false })
            .collect();
        grid.push(row);
    }
    return grid;
}

pub fn count_neighbors(grid: &Vec<Vec<bool>>, i: usize, j: usize) -> i64 {
    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    let mut count = 0;
    let rows = grid.len() as isize;
    let cols = grid[0].len() as isize;

    for (di, dj) in directions.iter() {
        let ni = i as isize + di;
        let nj = j as isize + dj;
        if ni >= 0 && ni < rows && nj >= 0 && nj < cols {
            if grid[ni as usize][nj as usize] {
                count += 1;
            }
        }
    }
    return count;
}

pub fn part2(input: &Vec<String>) -> i64 {
    let mut count = 0;
    let mut grid = parse_input(input);
    let rows = grid.len();
    let cols = grid[0].len();
    let mut removed = 0;
    loop {
        for i in 0..rows {
            for j in 0..cols {
                if grid[i][j] {
                    let neighbors = count_neighbors(&grid, i, j);
                    if neighbors < 4 {
                        count += 1;
                        removed += 1;
                        grid[i][j] = false;
                    }
                }
            }
        }
        if removed == 0 {
            break;
        }
        removed = 0;
    }
    return count;
}