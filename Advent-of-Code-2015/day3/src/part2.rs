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
    let mut robot_position = (0, 0);

    for (i, c) in line.chars().enumerate() {
        let direction = get_direction(c);
        
        let current = if i % 2 == 0 {
            position = (position.0 + direction.0, position.1 + direction.1);
            position
        } else {
            robot_position = (robot_position.0 + direction.0, robot_position.1 + direction.1);
            robot_position
        };

        if let Some(value) = houses.get_mut(&current) {
            *value += 1;
        } else {
            houses.insert(current, 1);
        }

    }

}

pub fn part2(input: &Vec<String>) -> i64 {
    let line = input[0].clone();
    let mut houses: HashMap<(i64, i64), i64> = HashMap::new();
    houses.insert((0, 0), 1);

    moves(&mut houses, line);

    return houses.len() as i64;
}