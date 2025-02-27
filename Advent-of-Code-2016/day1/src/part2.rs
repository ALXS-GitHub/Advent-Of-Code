use std::collections::HashSet;

fn parse_input(line: &String) -> Vec<(char, i32)> {

    let moves = line.split(", ").collect::<Vec<_>>();

    let mut mvs = Vec::new();

    for m in moves.into_iter() {
        let m: String = m.chars().collect();
        let rot = m.chars().next().unwrap();
        let dist = m.chars().skip(1).collect::<String>().parse::<i32>().unwrap();
        mvs.push((rot, dist));
    }

    return mvs
}

fn solve(moves: Vec<(char, i32)>) -> i32 {

    let dirs = vec![(0, -1), (1, 0), (0, 1), (-1, 0)];
    let mut d: i32 = 0;
    let mut pos: (i32, i32) = (0, 0);
    let mut visited = HashSet::new();
    visited.insert((0, 0));

    for (dir, dist) in moves {
        if dir == 'R' {
            d = (d + 1 + 4) % 4;
        } else {
            d = (d - 1 + 4) % 4;
        }

        for _ in 0..dist {

            let dir = (dirs[d as usize].0, dirs[d as usize].1);
    
            pos = (pos.0 + dir.0, pos.1 + dir.1);
            if visited.contains(&pos) {
                return pos.0.abs() + pos.1.abs()
            }
            visited.insert(pos);
        }

    }
    
    return pos.0.abs() + pos.1.abs()
}

pub fn part2(input: &Vec<String>) -> i64 {
    let moves = parse_input(&input[0]);
    return solve(moves) as i64;
}