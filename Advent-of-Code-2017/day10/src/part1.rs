fn hash(lengths: Vec<usize>) -> Vec<i64> {

    let mut current_pos = 0;
    let mut skip = 0;
    let mut list = vec![0; 256];
    for i in 0..list.len() {
        list[i] = i as i64;
    }

    for l in lengths {
        let split_off = (current_pos + l) % list.len();
        let mut sub = if split_off < current_pos {
            let mut sub = list[current_pos..list.len()].to_vec();
            sub.extend(list[..split_off].to_vec());
            sub
        } else {
            let sub = list[current_pos..split_off].to_vec();
            sub
        };

        sub.reverse();

        if split_off < current_pos {
            let end_len = list.len() - current_pos;
            list[current_pos..].copy_from_slice(&sub[..end_len]);
            list[..split_off].copy_from_slice(&sub[end_len..]);
        } else {
            list[current_pos..split_off].copy_from_slice(&sub);
        }

        current_pos = (current_pos + l + skip) % list.len();
        skip += 1;

    }

    list
}

pub fn part1(input: &Vec<String>) -> i64 {

    let lengths = input[0].clone().split(",").collect::<Vec<_>>().iter().map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>();

    let res = hash(lengths);

    return res[0] * res[1];
}