use regex::Regex;

fn parse_input(line: &String) -> (i64, i64) {
    let r = Regex::new(r"row (\d+), column (\d+)").unwrap();
    let caps = r.captures(line).unwrap();
    let row = caps.get(1).unwrap().as_str().parse::<i64>().unwrap();
    let col = caps.get(2).unwrap().as_str().parse::<i64>().unwrap();

    (row, col)
}

fn get_next_pos(row: i64, col: i64) -> (i64, i64) {
    if row == 1 {
        return (col + 1, 1);
    } else {
        return (row - 1, col + 1);
    }
}

fn solve(row: i64, col: i64) -> i64 {

    let mut value = 20151125;
    let mul = 252533;
    let div = 33554393;

    let (mut r, mut c) = (1, 1);

    while (r, c) != (row, col) {
        value *= mul;
        value %= div;
        (r, c) = get_next_pos(r, c);
    }

    return value;
}

pub fn part1(input: &Vec<String>) -> i64 {

    let (row, col) = parse_input(&input[0]);

    return solve(row, col);
}