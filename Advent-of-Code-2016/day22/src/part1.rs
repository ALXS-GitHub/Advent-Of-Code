use regex::Regex;

#[derive(Debug, Clone)]
struct Node {
    size: i64,
    used: i64,
}

type Grid = Vec<Vec<Node>>;

fn parse_input(input: &Vec<String>) -> Grid {
    let mut grid:Grid = vec![vec![Node {size: 0, used: 0}; 34]; 30];
    let r = Regex::new(r"/dev/grid/node-x(\d+)-y(\d+)\s+(\d+)T\s+(\d+)T").unwrap();

    for i in 2..input.len() {
        let caps = r.captures(&input[i]).unwrap();
        let x = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
        let y = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
        let size = caps.get(3).unwrap().as_str().parse::<i64>().unwrap();
        let used = caps.get(4).unwrap().as_str().parse::<i64>().unwrap();

        grid[y][x] = Node {
            size,
            used
        }
    }

    grid
}

fn count_viable_pairs(grid: &Grid) -> i64 {

    let flat_grid = grid.into_iter().flatten().collect::<Vec<_>>();

    let mut counter = 0;

    for i in 0..flat_grid.len() {
        for j in 0..flat_grid.len() {
            if flat_grid[i].used == 0 { continue }
            if i == j { continue }
            if flat_grid[i].used <= flat_grid[j].size - flat_grid[j].used {
                counter += 1;
            }
        }
    }

    counter
}

pub fn part1(input: &Vec<String>) -> i64 {

    let grid = parse_input(input);

    let res = count_viable_pairs(&grid);

    return res;
}