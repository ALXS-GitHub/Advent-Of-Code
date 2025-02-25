fn add_tuple(a: (i64, i64), b: (i64, i64)) -> (i64, i64) {
    return (a.0 + b.0, a.1 + b.1)
}

fn is_corner(square_size: i64, pos: (i64, i64)) -> bool {
    let offset = square_size / 2;
    return pos.0.abs() == offset && pos.1.abs() == offset
}

fn is_bottom_corner(square_size: i64, pos: (i64, i64)) -> bool {
    let offset = square_size / 2;
    return pos.0 == offset && pos.1 == -offset
}

fn find_index_coords(index: i64) -> (i64, i64) {

    let directions = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];
    let mut dir_idx = 0;
    let mut pos = (0, 0);
    let mut square_size = 1;
    let mut counter = 1;
    
    while counter < index {
        // first check the bottom corner
        if is_bottom_corner(square_size, pos) {
            square_size += 2;
            pos = add_tuple(pos, directions[dir_idx]);
            dir_idx = (dir_idx + 1) % directions.len();
        } else if is_corner(square_size, pos) {
            dir_idx = (dir_idx + 1) % directions.len();
            pos = add_tuple(pos, directions[dir_idx]);
        } else {
            pos = add_tuple(pos, directions[dir_idx]);
        }
        counter += 1;
    }

    pos
}

pub fn part1(input: &Vec<String>) -> i64 {

    let index = input[0].parse::<i64>().unwrap();

    let res = find_index_coords(index);

    return res.0.abs() + res.1.abs();
}