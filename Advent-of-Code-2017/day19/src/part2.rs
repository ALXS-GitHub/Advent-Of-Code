pub fn follow(start: (i32, i32), maze: &Vec<Vec<char>>) -> i32 {
    let mut res_string = String::new();
    let mut counter = 0;

    let mut dirs = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];
    let mut next_pos: Option<(i32, i32)> = Some(start);
    let mut dir = dirs[0];

    while let Some(pos) = next_pos {
        counter += 1;
        
        let current_char = maze[pos.0 as usize][pos.1 as usize];

        next_pos = match current_char {
            '|' => {
                if pos.0 + dir.0 >= maze.len() as i32 || pos.1 + dir.1 >= maze[0].len() as i32 || pos.0 + dir.0 < 0 || pos.1 + dir.1 < 0 {
                    None
                } else {
                    Some((pos.0 + dir.0, pos.1 + dir.1))
                }
            }, 
            '-' => {
                if pos.0 + dir.0 >= maze.len() as i32 || pos.1 + dir.1 >= maze[0].len() as i32 || pos.0 + dir.0 < 0 || pos.1 + dir.1 < 0 {
                    None
                } else {
                    Some((pos.0 + dir.0, pos.1 + dir.1))
                }
            },
            '+' => {
                for d in dirs.clone() {
                    if d == (-dir.0, -dir.1) {
                        continue;
                    }

                    let np = (pos.0 + d.0, pos.1 + d.1);
                    if np.0 >= 0 && np.1 >= 0 && np.0 < maze.len() as i32 && np.1 < maze[np.0 as usize].len() as i32 {
                        let c = maze[np.0 as usize][np.1 as usize];
                        if c == '-' && (d == (0, -1) || d == (0, 1)) || c == '|' && (d == (-1, 0) || d == (1, 0)) {
                            dir = d;
                            break;
                        } else if c != ' ' {
                            dir = d;
                            break;
                        }
                    }
                }
                if pos.0 + dir.0 >= maze.len() as i32 || pos.1 + dir.1 >= maze[0].len() as i32 || pos.0 + dir.0 < 0 || pos.1 + dir.1 < 0 {
                    None
                } else {
                    Some((pos.0 + dir.0, pos.1 + dir.1))
                }
            },
            ' ' => {
                counter -= 1;
                None
            },
            _ => {
                res_string.push(current_char);
                if pos.0 + dir.0 >= maze.len() as i32 || pos.1 + dir.1 >= maze[0].len() as i32 || pos.0 + dir.0 < 0 || pos.1 + dir.1 < 0 {
                    None
                } else {
                    Some((pos.0 + dir.0, pos.1 + dir.1))
                }
            }
        };


    }

    counter
}

pub fn part2(input: &Vec<String>) -> i32 {

    let mut maze = input.iter().map(|l| l.chars().collect::<Vec<_>>()).collect::<Vec<_>>();

    let start = (0i32, maze[0].iter().position(|c| c == &'|').unwrap() as i32);

    let res = follow(start, &maze);

    return res;
}