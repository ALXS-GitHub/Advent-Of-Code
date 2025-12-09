pub struct Pos {
    pub x: i64,
    pub y: i64,
}

pub fn parse_input(input: &Vec<String>) -> Vec<Pos> {
    let mut positions: Vec<Pos> = Vec::new();

    for line in input {
        let coords: Vec<&str> = line.split(',').collect();
        let x: i64 = coords[0].trim().parse().unwrap();
        let y: i64 = coords[1].trim().parse().unwrap();
        positions.push(Pos { x, y });
    }

    return positions;
}

pub fn area(p1: &Pos, p2: &Pos) -> i64 {
    let width = (p1.x - p2.x).abs() + 1;
    let height = (p1.y - p2.y).abs() + 1;
    return width * height;
}

pub fn part1(input: &Vec<String>) -> i64 {

    let positions = parse_input(input);

    let mut max = 0;
    for i in 0..positions.len() {
        let p1 = &positions[i];
        for j in (i + 1)..positions.len() {
            let p2 = &positions[j];
            let a = area(p1, p2);
            if a > max {
                max = a;
            }
        }
    }

    return max;
}