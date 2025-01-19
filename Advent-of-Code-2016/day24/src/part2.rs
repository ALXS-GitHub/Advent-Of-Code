use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    i: usize,
    j: usize,
    obtained: Vec<char>
}

fn bfs(grid: &Vec<Vec<char>>, start_pos: (usize, usize), targets: Vec<char>) -> usize {
    let mut queue = VecDeque::new();
    queue.push_back(State { i: start_pos.0, j: start_pos.1, obtained: Vec::new() });
    let mut visited = HashMap::new();
    visited.insert(State {
        i: start_pos.0,
        j: start_pos.1,
        obtained: Vec::new()
    }, 0);
    
    while !queue.is_empty() {
        let current = queue.pop_front().unwrap();
        for (di, dj) in vec![(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let new_i = current.i as i32 + di;
            let new_j = current.j as i32 + dj;
            if new_i >= 0 && new_i < grid.len() as i32 && new_j >= 0 && new_j < grid[0].len() as i32 {
                let new_i = new_i as usize;
                let new_j = new_j as usize;
                let mut new_obtained = current.obtained.clone();
                if grid[new_i][new_j] == '#' {
                    continue;
                }
                if grid[new_i][new_j].is_digit(10) && !new_obtained.contains(&grid[new_i][new_j]) && grid[new_i][new_j] != '0' {
                    new_obtained.push(grid[new_i][new_j]);
                    new_obtained.sort();
                }
                if new_obtained == targets && grid[new_i][new_j] == '0' {
                    return visited.get(&current).unwrap() + 1;
                }
                let new_state = State { i: new_i, j: new_j, obtained: new_obtained };
                if !visited.contains_key(&new_state) {
                    visited.insert(new_state.clone(), visited.get(&current).unwrap() + 1);
                    queue.push_back(new_state);
                }
            }
        }
    }

    return 0;

}

fn get_infos(input: &Vec<String>) -> ((usize, usize), Vec<char>) {
    let mut targets = Vec::new();
    let mut start_pos = (0, 0);
    for i in 0..input.len() {
        for j in 0..input[0].len() {
            if input[i].chars().nth(j).unwrap() == '0' {
                start_pos = (i, j);
            } else if input[i].chars().nth(j).unwrap().is_digit(10) {
                targets.push(input[i].chars().nth(j).unwrap());
            }
        }
    }
    targets.sort();
    (start_pos, targets)
}

pub fn part2(input: &Vec<String>) -> i64 {

    let (start_pos, targets) = get_infos(input);
    let grid = input.into_iter().map(|l| l.chars().collect::<Vec<_>>()).collect::<Vec<_>>();

    let dist = bfs(&grid, start_pos, targets);

    return dist as i64;
}