use md5;
use std::collections::VecDeque;

fn hash(string: &str) -> String {
    let mut hash = md5::compute(string);
    let mut str_hash = format!("{:x}", hash);
    for _ in 0..2016 {
        hash = md5::compute(&str_hash);
        str_hash = format!("{:x}", hash);
    }
    str_hash
}

fn find_triple(hash: &str) -> char {
    let chars = hash.chars().collect::<Vec<_>>();
    for w in chars.windows(3) {
        if w[0] == w[1] && w[0] == w[2] {
            return w[0]
        }
    }  
    return ' '
} 

fn has_n_consecutive(hash: &str, n: usize, c: char) -> bool {
    let chars = hash.chars().collect::<Vec<_>>();
    for w in chars.windows(n) {
        let mut found = true;
        for i in 0..n {
            if c != w[i] {
                found = false;
                break;
            }
        }
        if found {
            return true
        }
    }  
    return false
}

fn find_keys(size: usize, queue: &mut VecDeque<String>, salt: &String) -> (Vec<char>, i64) {

    let mut keys = Vec::new();
    let mut counter = 0;
    let n = queue.len();

    while keys.len() < size {

        let current_hash = queue.pop_front().unwrap();
        queue.push_back(hash(&format!("{}{}", salt, counter + n)));
        
        counter += 1;

        let key = find_triple(&current_hash);
        if key != ' ' {
            for h in queue.iter() {
                if has_n_consecutive(h, 5, key) {
                    keys.push(key);
                    break
                }
            }
        }

    }

    (keys, counter as i64 - 1)

}

pub fn part2(input: &Vec<String>) -> i64 {

    let salt = &input[0];

    let mut queue = VecDeque::new();

    for i in 0..1000 {
        let hash = hash(&format!("{}{}", salt, i));
        queue.push_back(hash);
    }

    let size = 64;

    let (keys, counter) = find_keys(size, &mut queue, salt);

    return counter;
}