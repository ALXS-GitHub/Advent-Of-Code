use std::collections::VecDeque;

struct State {
    hash: String,
    position: (i64, i64),
    distance: i64,
}

fn md5(input: &String) -> String {
    let digest = md5::compute(input.as_bytes());
    format!("{:x}", digest)
}

fn add_tuple(a: (i64, i64), b: (i64, i64)) -> (i64, i64) {
    return (a.0 + b.0, a.1 + b.1)
}

fn bfs(hash: &String, grid_size: usize) -> String {
    let mut queue = VecDeque::new();
    queue.push_back(State {
        hash: hash.clone(),
        position: (0, 0),
        distance: 0
    });

    let opened: Vec<char> = vec!['b', 'c', 'd', 'e', 'f'];
    let dirs = vec!['U', 'D', 'L', 'R'];

    while let Some(state) = queue.pop_front() {

        let locks = &md5(&state.hash)[..4].chars().collect::<Vec<_>>();
        
        for i in 0..locks.len() {
            let l = locks[i];
            if opened.contains(&l) {
                
                let new_pos = match dirs[i] {
                    'U' => {add_tuple(state.position, (-1, 0))},
                    'D' => {add_tuple(state.position, (1, 0))},
                    'L' => {add_tuple(state.position, (0, -1))},
                    'R' => {add_tuple(state.position, (0, 1))},
                    _ => panic!("unknown")
                };
                
                if new_pos.0 < 0 || new_pos.0 >= grid_size as i64 || new_pos.1 < 0 || new_pos.1 >= grid_size as i64 {
                    continue;
                }

                let new_hash = state.hash.clone() + &dirs[i].to_string();

                let new_state = State {
                    hash: new_hash.clone(),
                    position: new_pos,
                    distance: state.distance + 1,
                };

                if new_state.position.0 == grid_size as i64 - 1 && new_state.position.1 == grid_size as i64 - 1 {
                    return new_hash;
                } 
                
                queue.push_back(new_state);
            }
        }

    }

    return "".to_string()
}

pub fn part1(input: &Vec<String>) -> String {

    let mut init_hash = &input[0];
    let grid_size: usize = 4;

    let res = bfs(init_hash, grid_size);

    let res = res[init_hash.len()..].to_string();

    return res;
}