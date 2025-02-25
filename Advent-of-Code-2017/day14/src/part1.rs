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

pub fn part1(input: &Vec<String>) -> i64 {
    let res = count_used(&input[0]);
    return res;
}