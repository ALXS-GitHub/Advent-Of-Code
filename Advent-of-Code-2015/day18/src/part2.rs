fn step(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {

    let mut new_grid = vec![vec!['.'; 100]; 100];
    new_grid[0][0] = '#';
    new_grid[99][0] = '#';
    new_grid[99][99] = '#';
    new_grid[0][99] = '#';

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            let on_neighbors = get_on_neighbors(grid, i, j);
            if grid[i][j] == '#' && (on_neighbors == 2 || on_neighbors == 3) {
                new_grid[i][j] = '#';
            } else if on_neighbors == 3 {
                new_grid[i][j] = '#';
            }
        }
    }

    new_grid
}

fn get_on_neighbors(grid: &Vec<Vec<char>>, i: usize, j: usize) -> i64 {

    let ia = i as i64 - 1;
    let ib = i as i64 + 1;
    let ja = j as i64 - 1;
    let jb = j as i64 + 1;

    let mut count = 0;

    for _i in ia..=ib {
        for _j in ja..=jb {
            if (_i, _j) == (i as i64, j as i64) {
                continue
            }

            if _i < 0 || _j < 0 || _i >= grid.len() as i64 || _j >= grid.len() as i64 {
                continue
            }

            if grid[_i as usize][_j as usize] == '#' {
                count += 1;
            }

        }
    }
    count

}

fn count_on_lights(grid: &Vec<Vec<char>>) -> i64 {
    let mut count = 0;

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            count += if grid[i][j] == '#' { 1 } else { 0 };
        }
    }

    count
}


pub fn part2(input: &Vec<String>) -> i64 {
    let mut grid: Vec<Vec<char>> = input.iter().map(|l| l.chars().collect()).collect();
    let steps = 100;

    grid[0][0] = '#';
    grid[99][0] = '#';
    grid[99][99] = '#';
    grid[0][99] = '#';

    for _ in 0..steps {
        grid = step(&grid);
    }

    return count_on_lights(&grid);
}