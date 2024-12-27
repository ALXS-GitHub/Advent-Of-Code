use std::collections::HashMap;

fn get_direction(c: char) -> (i64, i64) {
    match c {
        'v' => return (0, -1),
        '<' => return (-1, 0),
        '^' => return (0, 1),
        '>' => return (1, 0),
        _ => panic!("Unknown direction")
    }
}

fn moves(houses: &mut HashMap<(i64, i64), i64>, line: String) {

    let mut position = (0, 0);

    for c in line.chars() {
        let direction = get_direction(c);
        position = (position.0 + direction.0, position.1 + direction.1);

        if let Some(value) = houses.get_mut(&position) {
            *value += 1;
        } else {
            houses.insert(position, 1);
        }

    }

}

pub fn part1(input: &Vec<String>) -> i64 {

    let line = input[0].clone();
    let mut houses: HashMap<(i64, i64), i64> = HashMap::new();
    houses.insert((0, 0), 1);

    moves(&mut houses, line);

    return houses.len() as i64;
}