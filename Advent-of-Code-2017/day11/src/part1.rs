fn mv(pos: &(i64, i64), dir: &str) -> (i64, i64) {
    match dir {
        "s" => {
            return (pos.0, pos.1 + 1)
        },
        "n" => {
            return (pos.0, pos.1 - 1)
        },
        "sw" => {
            if pos.0.abs() % 2 == 0 {
                return (pos.0 - 1, pos.1)
            } else {
                return (pos.0 - 1, pos.1 + 1)
            }
        },
        "se" => {
            if pos.0.abs() % 2 == 0 {
                return (pos.0 + 1, pos.1)
            } else {
                return (pos.0 + 1, pos.1 + 1)
            }
        },
        "nw" => {
            if pos.0.abs() % 2 == 0 {
                return (pos.0 - 1, pos.1 - 1)
            } else {
                return (pos.0 - 1, pos.1)
            }
        },
        "ne" => {
            if pos.0.abs() % 2 == 0 {
                return (pos.0 + 1, pos.1 - 1)
            } else {
                return (pos.0 + 1, pos.1)
            }
        },
        _ => panic!("Unknown")
    }
}

fn offset_to_cube(offset: &(i64, i64)) -> (i64, i64, i64) {
    let x = offset.0;
    let z = offset.1 - (offset.0 + (offset.0.abs() % 2)) / 2;
    let y = -x - z;
    (x, y, z)
}

fn get_distance(start: &(i64, i64), end: &(i64, i64)) -> i64 {
    let start_cube = offset_to_cube(start);
    let end_cube = offset_to_cube(end);
    let dx = (end_cube.0 - start_cube.0).abs();
    let dy = (end_cube.1 - start_cube.1).abs();
    let dz = (end_cube.2 - start_cube.2).abs();
    std::cmp::max(dx, std::cmp::max(dy, dz))
}

pub fn part1(input: &Vec<String>) -> i64 {

    let moves = input[0].split(",").collect::<Vec<_>>();

    let mut end_pos = (0, 0);
    for m in moves {
        end_pos = mv(&end_pos, m);
    }

    let dist = get_distance(&(0, 0), &end_pos);

    return dist;
}