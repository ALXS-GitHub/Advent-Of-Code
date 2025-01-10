use std::collections::HashMap;

fn parse_input(input: &Vec<String>) -> Vec<Vec<char>> {
    let mut codes = vec![vec![]; input[0].len()];

    for line in input {
        for (i, c) in line.chars().enumerate() {
            codes[i].push(c);
        }
    }

    codes
}

fn get_most_common(col: Vec<char>) -> char {
    let mut map = HashMap::new();

    for c in col {
        *map.entry(c).or_insert(0) += 1;
    }

    let (mut max, mut mc) = (0, ' ');
    for (k, v) in map.iter() {
        if *v >= max {
            max = *v;
            mc = *k;
        }
    }

    return mc;
}

pub fn part1(input: &Vec<String>) -> String {

    let codes = parse_input(&input);

    let mut message = String::new();

    for code in codes {
        message.push(get_most_common(code));
    }

    return message;
}