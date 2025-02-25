use std::collections::HashSet;

fn distribute_remainder(banks: &mut Vec<i64>, current_idx: &mut usize, remainder: &mut i64) {
    while *remainder > 0 {
        *current_idx += 1;
        *current_idx %= banks.len();
        banks[*current_idx] += 1;
        *remainder -= 1;
    }
}

fn find_cycle(banks: &mut Vec<i64>) -> i64 {
    let mut seen = HashSet::new();
    
    let mut counter = 0;
    
    while let None = seen.get(banks) {
        counter += 1;
        seen.insert(banks.clone());
        let max = *banks.iter().max().unwrap();
        let mut max_idx = banks.iter().position(|&x| x == max).unwrap();
        banks[max_idx] = 0;
        let d = max / banks.len() as i64;
        let mut r = max % banks.len() as i64;
        if d != 0 {
            for bank in banks.into_iter() {
                *bank += d;
            }
        }
        distribute_remainder(banks, &mut max_idx, &mut r);
    }

    counter
}

pub fn part1(input: &Vec<String>) -> i64 {

    let mut banks = input[0].split_whitespace().collect::<Vec<_>>().iter().map(|x| x.parse::<i64>().unwrap()).collect::<Vec<_>>();

    let res = find_cycle(&mut banks);

    return res;
}