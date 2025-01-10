use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref KEYPAD: HashMap<(i64, i64), char> = {
        let mut m = HashMap::new();
        m.insert((0, 0), '1');
        m.insert((0, 1), '2');
        m.insert((0, 2), '3');
        m.insert((1, 0), '4');
        m.insert((1, 1), '5');
        m.insert((1, 2), '6');
        m.insert((2, 0), '7');
        m.insert((2, 1), '8');
        m.insert((2, 2), '9');
        return m;
    };
}

fn solve(input: &Vec<String>) -> i64 {

    let mut res = String::new();

    let mut pos = (1, 1);

    for line in input {
        let chars = line.chars().collect::<Vec<char>>();
        for c in chars {
            let v = match c {
                'R' => (0, 1),
                'L' => (0, -1),
                'D' => (1, 0),
                'U' => (-1, 0),
                _ => panic!("unknown character")
            };

            let new = (pos.0 + v.0, pos.1 + v.1);
            if 0 <= new.0 && new.0 <= 2 && 0 <= new.1 && new.1 <= 2 {
                pos = new;
            }

        }

        res.push(*KEYPAD.get(&pos).unwrap());
    }

    return res.parse::<i64>().unwrap();

}

pub fn part1(input: &Vec<String>) -> i64 {
    return solve(input);
}