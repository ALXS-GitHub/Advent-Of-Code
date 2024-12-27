use regex::Regex;

fn turn_on(b: bool) -> bool {
    true
}

fn turn_off(b: bool) -> bool {
    false
}

fn toggle(b: bool) -> bool {
    !b
}

fn apply<F>(f: F, grid: &mut Vec<Vec<bool>>, coords1: (usize, usize), coords2: (usize, usize))
where F: Fn(bool) -> bool
{

    for x in coords1.0..=coords2.0 {
        for y in coords1.1..=coords2.1 {
            grid[y][x] = f(grid[y][x])
        }
    }

}

fn execute_line(line: String, grid: &mut Vec<Vec<bool>>) {
    let r = Regex::new(r"^([\w\s]+) (\d+),(\d+) through (\d+),(\d+)").unwrap();
    let caps = r.captures(&line).unwrap();
    let action = caps.get(1).unwrap().as_str();
    let coords1 = (caps.get(2).unwrap().as_str().parse::<usize>().unwrap(), caps.get(3).unwrap().as_str().parse::<usize>().unwrap());
    let coords2 = (caps.get(4).unwrap().as_str().parse::<usize>().unwrap(), caps.get(5).unwrap().as_str().parse::<usize>().unwrap());

    let f = match action {
        "turn on" => turn_on,
        "turn off" => turn_off,
        "toggle" => toggle,
        _ => panic!("Unknown action: {}", action)
    };

    apply(f, grid, coords1, coords2)
}

fn count_lights(grid: Vec<Vec<bool>>) -> i64 {
    grid.iter()
        .flatten()
        .filter(|&&cell| cell)
        .count() as i64
}

pub fn part1(input: &Vec<String>) -> i64 {

    let mut grid = vec![vec![false; 1000]; 1000];

    for s in input.iter() {
        execute_line(s.to_string(), &mut grid);
    }

    return count_lights(grid);
}