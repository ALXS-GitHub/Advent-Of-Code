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

fn set_node_types(grid: &Grid) -> (Vec<Vec<char>>, (usize, usize)) {
    let mut m_size = i64::MAX;
    let mut M_used = 0;
    let flat_grid = grid.into_iter().flatten().collect::<Vec<_>>();

    for n in flat_grid.iter() {
        if n.size <= 100 && n.used <= 100 {
            if n.size < m_size {
                m_size = n.size;
            }
            if n.used > M_used {
                M_used = n.used;
            }
        }
    }

    // println!("{} - {}", m_size, M_used);
    assert!(m_size > M_used);

    let mut type_grid = vec![vec!['.'; 34]; 30];
    let mut start_pos = (0, 0);

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x].used >= 100 {
                type_grid[y][x] = '#';
            }
            if grid[y][x].used == 0 {
                type_grid[y][x] = ' ';
                start_pos = (y, x);
            }
        }
    }

    (type_grid, start_pos)
}

fn display_type_grid(type_grid: Vec<Vec<char>>){
    for line in type_grid {
        let line = line.iter().collect::<String>();
        println!("{}", line);
    }
}

fn bfs(type_grid: &Vec<Vec<char>>, start_pos: (usize, usize), end_pos: (usize, usize)) -> i64 {
    let mut queue = Vec::new();
    let mut visited = vec![vec![false; 34]; 30];

    queue.push((start_pos, 0));
    visited[start_pos.0][start_pos.1] = true;

    while !queue.is_empty() {
        let (pos, steps) = queue.remove(0);

        if pos == end_pos {
            return steps;
        }

        let (y, x) = pos;

        let directions = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];

        for dir in directions {
            let new_y = y as i64 + dir.0;
            let new_x = x as i64 + dir.1;

            if new_y < 0 || new_y >= 30 || new_x < 0 || new_x >= 34 {
                continue;
            }

            if visited[new_y as usize][new_x as usize] || type_grid[new_y as usize][new_x as usize] == '#' {
                continue;
            }

            visited[new_y as usize][new_x as usize] = true;
            queue.push(((new_y as usize, new_x as usize), steps + 1));
        }
    }

    return -1;
}

pub fn part2(input: &Vec<String>) -> i64 {
    let grid = parse_input(input);

    let (type_grid, start_pos) = set_node_types(&grid);
    let end_pos = (0, 33);

    // display_type_grid(type_grid);

    let mut distance = bfs(&type_grid, start_pos, end_pos);
    distance += 5 * (grid[0].len() as i64 - 2); // loops for moving the goal node

    return distance;
}