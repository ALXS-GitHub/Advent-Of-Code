use std::collections::HashMap;

fn distribute_remainder(banks: &mut Vec<i64>, current_idx: &mut usize, remainder: &mut i64) {
    while *remainder > 0 {
        *current_idx += 1;
        *current_idx %= banks.len();
        banks[*current_idx] += 1;
        *remainder -= 1;
    }
}

fn find_cycle(banks: &mut Vec<i64>) -> i64 {
    let mut seen = HashMap::new();
    
    let mut counter = 0;
    
    while let None = seen.get(banks) {
        seen.insert(banks.clone(), counter);
        counter += 1;
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

    if let Some(b) = seen.get(banks) {
        return counter - *b;
    } else {
        0
    }

}

pub fn part2(input: &Vec<String>) -> i64 {

    let mut banks = input[0].split_whitespace().collect::<Vec<_>>().iter().map(|x| x.parse::<i64>().unwrap()).collect::<Vec<_>>();

    let res = find_cycle(&mut banks);

    return res;
}