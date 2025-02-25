use std::collections::HashSet;
use std::collections::VecDeque;

fn hash(lengths: &Vec<usize>, list: &mut Vec<i64>, current_pos: &mut usize, skip: &mut usize) {

    for l in lengths {
        let split_off = (*current_pos + l) % list.len();
        let mut sub = if split_off < *current_pos {
            let mut sub = list[*current_pos..list.len()].to_vec();
            sub.extend(list[..split_off].to_vec());
            sub
        } else {
            let sub = list[*current_pos..split_off].to_vec();
            sub
        };

        sub.reverse();

        if split_off < *current_pos {
            let end_len = list.len() - *current_pos;
            list[*current_pos..].copy_from_slice(&sub[..end_len]);
            list[..split_off].copy_from_slice(&sub[end_len..]);
        } else {
            list[*current_pos..split_off].copy_from_slice(&sub);
        }

        *current_pos = (*current_pos + l + *skip) % list.len();
        *skip += 1;

    }
}

fn xor(chunk: Vec<u8>) -> u8 {
    let mut res = chunk[0];
    for i in 1..chunk.len() {
        res ^= chunk[i];
    }
    res
}

fn knot_hash(input: &String) -> String {
    let mut lengths = input.clone().into_bytes().iter().map(|x| *x as usize).collect::<Vec<_>>();
    lengths.extend_from_slice(&[17, 31, 73, 47, 23]);
    let rounds = 64;

    let mut current_pos = 0;
    let mut skip = 0;
    let mut list = vec![0; 256];
    for i in 0..list.len() {
        list[i] = i as i64;
    }

    for _ in 0..rounds {
        hash(&lengths, &mut list, &mut current_pos, &mut skip);
    }

    let mut dense_hash = Vec::new();
    for chunk in list.chunks(16) {
        let chunk = chunk.to_vec().iter().map(|x| *x as u8).collect::<Vec<_>>();
        dense_hash.push(xor(chunk));
    }

    let mut hexa = String::new();
    for h in dense_hash {
        hexa.push_str(&format!("{:02x}", h));
    }

    return hexa.clone();
}

fn hex_to_bin(hex: &String) -> String {
    let mut bin = String::new();
    for c in hex.chars() {
        let num = i8::from_str_radix(&c.to_string(), 16).unwrap();
        let num_bin = format!("{:04b}", num);
        bin.push_str(&num_bin);
    }
    bin.clone()
}

fn count_used(input: &String) -> i64 {
    let mut counter = 0;
    for i in 0..128 {
        let hash_input = format!("{}-{}", input, i);
        let hex = knot_hash(&hash_input);
        let bin = hex_to_bin(&hex);
        let ones = bin.chars().filter(|&c| c == '1').count();
        counter += ones as i64;
    }
    counter
}

fn get_grid(input: &String) -> Vec<Vec<char>> {
    let mut grid = Vec::new();
    for i in 0..128 {
        let hash_input = format!("{}-{}", input, i);
        let hex = knot_hash(&hash_input);
        let bin = hex_to_bin(&hex);
        grid.push(bin.chars().collect::<Vec<_>>());
    }
    grid
}

fn propagate_region(grid: &mut Vec<Vec<char>>, start: (usize, usize)) {
    grid[start.0][start.1] = 'X'; // seen
    let mut queue = VecDeque::new();
    queue.push_back(start);
    while !queue.is_empty() {
        let current = queue.pop_front().unwrap();
        let i = current.0;
        let j = current.1;
        if i > 0 && grid[i - 1][j] == '1' {
            grid[i - 1][j] = 'X';
            queue.push_back((i - 1, j));
        }
        if i < grid.len() - 1 && grid[i + 1][j] == '1' {
            grid[i + 1][j] = 'X';
            queue.push_back((i + 1, j));
        }
        if j > 0 && grid[i][j - 1] == '1' {
            grid[i][j - 1] = 'X';
            queue.push_back((i, j - 1));
        }
        if j < grid[i].len() - 1 && grid[i][j + 1] == '1' {
            grid[i][j + 1] = 'X';
            queue.push_back((i, j + 1));
        }
    }

}

fn count_regions(grid: &mut Vec<Vec<char>>) -> i64 {
    let mut counter = 0;

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == '1' {
                counter += 1;
                propagate_region(grid, (i, j));
            }
        }
    }
    counter
}

pub fn part2(input: &Vec<String>) -> i64 {
    let mut grid = get_grid(&input[0]);
    let res = count_regions(&mut grid);
    return res;
}