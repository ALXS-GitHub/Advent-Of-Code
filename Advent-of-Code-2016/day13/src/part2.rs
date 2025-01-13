use std::collections::HashMap;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

type Grid = Vec<Vec<bool>>; // 0 -> wall, 1 -> open space

fn is_open_space(fav_num: i64, (i, j): (i64, i64)) -> bool {
    let mut eq = j * j + 3 * j + 2 * i * j + i + i * i;
    eq += fav_num;
    let ones = eq.count_ones();
    if ones % 2 == 0 {
        return true
    }
    false
}

fn build_maze(size: usize, fav_num: i64) -> Grid {

    let mut grid = vec![vec![false;size]; size];

    for i in 0..size {
        for j in 0..size {
            grid[i][j] = is_open_space(fav_num, (i as i64, j as i64));
        }
    }
    grid
}

fn display_maze(maze: &Grid) {
    for l in maze {
        for b in l {
            if *b {
                print!(".");
            } else {
                print!("#");
            }
        }
        println!()
    }
}

fn dijkstra(maze: &Grid, start: (i64, i64), end: (i64, i64)) -> i64 {
    let mut visited = HashMap::new();
    let mut heap = BinaryHeap::new();

    heap.push(Reverse((0, start)));

    while !heap.is_empty() {
        let Reverse((dist, (i, j))) = heap.pop().unwrap();
        // if (i, j) == end {
        //     return dist;
        // }
        if visited.contains_key(&(i, j)) {
            continue;
        }
        visited.insert((i, j), dist);

        for (di, dj) in [(-1, 0), (1, 0), (0, -1), (0, 1)].iter() {
            let (ni, nj) = (i + di, j + dj);
            if ni < 0 || nj < 0 || ni >= maze.len() as i64 || nj >= maze[0].len() as i64 {
                continue;
            }
            if maze[ni as usize][nj as usize] {
                heap.push(Reverse((dist + 1, (ni, nj))));
            }
        }
    }

    let mut counter = 0;
    for (_, v) in visited.iter() {
        if *v <= 50 {
            counter += 1;
        }
    }

    return counter;

}

pub fn part2(input: &Vec<String>) -> i64 {

    let fav_num = input[0].parse::<i64>().unwrap();

    let (si, sj) = (1, 1);
    let (ei, ej) = (51, 51);
    let size = 52; // estimation of required size

    let maze = build_maze(size, fav_num);
    // display_maze(&maze);

    return dijkstra(&maze, (si, sj), (ei, ej));
}