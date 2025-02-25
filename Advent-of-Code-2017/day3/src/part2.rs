use std::collections::HashMap;

fn add_tuple(a: (i64, i64), b: (i64, i64)) -> (i64, i64) {
    return (a.0 + b.0, a.1 + b.1)
}

fn is_corner(square_size: i64, pos: (i64, i64)) -> bool {
    let offset = square_size / 2;
    return pos.0.abs() == offset && pos.1.abs() == offset
}

fn is_bottom_corner(square_size: i64, pos: (i64, i64)) -> bool {
    let offset = square_size / 2;
    return pos.0 == offset && pos.1 == -offset
}

fn calculate_value(pos: (i64, i64), visited: &HashMap<(i64, i64), i64>) -> i64 {
    let mut val = 0;
    for dx in -1..=1 {
        for dy in -1..=1 {
            if dx == 0 && dy == 0 {
                continue
            }
            let d = (dx, dy);
            let vp = add_tuple(pos, d);
            if let Some(p) = visited.get(&vp) {
                val += *p;
            }
        }
    }
    val
}

fn find_larger(index: i64) -> i64 {

    let directions = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];
    let mut dir_idx = 0;
    let mut pos = (0, 0);
    let mut square_size = 1;

    let mut visited = HashMap::new();
    visited.insert(pos, 1);

    let mut last_val = 1;
    
    while last_val <= index {
        // first check the bottom corner
        if is_bottom_corner(square_size, pos) {
            square_size += 2;
            pos = add_tuple(pos, directions[dir_idx]);
            dir_idx = (dir_idx + 1) % directions.len();
        } else if is_corner(square_size, pos) {
            dir_idx = (dir_idx + 1) % directions.len();
            pos = add_tuple(pos, directions[dir_idx]);
        } else {
            pos = add_tuple(pos, directions[dir_idx]);
        }
        let v = calculate_value(pos, &visited);
        last_val = v;
        visited.insert(pos, v);
    }

    last_val
}

pub fn part2(input: &Vec<String>) -> i64 {

    let index = input[0].parse::<i64>().unwrap();

    let res = find_larger(index);

    return res;
}