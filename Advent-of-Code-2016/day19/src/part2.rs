#[derive(Debug, Clone)]
struct Elve {
    id: usize,
    presents: usize,
}

// too slow method
fn get_winner(elves: &mut Vec<Elve>) -> usize {

    let mut idx = 0;

    while elves.len() > 1 {
        let mut offset = (elves.len() / 2) + idx;
        let rm_idx = (offset + elves.len()) % elves.len();
        // println!("{:?} - {} - {} - {}", elves, idx, offset, rm_idx);

        if !(offset >= elves.len()) {
            idx += 1;
        }

        elves.remove(rm_idx);
        if idx >= elves.len() {
            idx = 0;
        }
    } 

    elves[0].id
}

fn solve_pow_3(num_elves: usize) -> usize {
    let mut pow_3a = 0;
    let mut pow_3b = 1;
    while pow_3b < num_elves {
        pow_3a = pow_3b;
        pow_3b *= 3;
    }

    let diff = pow_3b - pow_3a;
    let mid_diff = diff / 2;
    let pow_3a_mid = pow_3a + mid_diff;

    if num_elves > pow_3a_mid {
        let delta = num_elves - pow_3a_mid;
        return delta * 2 + mid_diff;
    } else {
        let delta = num_elves - pow_3a;
        return delta;
    }
}

pub fn part2(input: &Vec<String>) -> i64 {

    let num_elves = input[0].parse::<usize>().unwrap();

    let res = solve_pow_3(num_elves);

    return res as i64;
}