use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref KEYPAD: HashMap<(i64, i64), char> = {
        let mut m = HashMap::new();
        m.insert((0, 0), '7');
        m.insert((0, -1), '6');
        m.insert((0, -2), '5');
        m.insert((-2, 0), '1');
        m.insert((-1, -1), '2');
        m.insert((-1, 0), '3');
        m.insert((-1, 1), '4');
        m.insert((0, 1), '8');
        m.insert((0, 2), '9');
        m.insert((1, -1), 'A');
        m.insert((1, 0), 'B');
        m.insert((1, 1), 'C');
        m.insert((2, 0), 'D');
        return m;
    };
}

fn solve(input: &Vec<String>) -> String {

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

            let new: (i64, i64) = (pos.0 + v.0, pos.1 + v.1);
            if 0 <= new.0.abs() + new.1.abs() && new.0.abs() + new.1.abs() <= 2 {
                pos = new;
            }

        }

        res.push(*KEYPAD.get(&pos).unwrap());
    }

    return res;

}

pub fn part2(input: &Vec<String>) -> String {
    return solve(input);
}