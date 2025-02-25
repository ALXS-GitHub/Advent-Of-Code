use std::collections::HashSet;

struct Virus {
    dirs: Vec<(i32, i32)>,
    dir: i32,
    pos: (i32, i32),
    infection_counter: i32
}

impl Virus {
    fn new() -> Self {
        Self {
            dirs: vec![(0, 1), (1, 0), (0, -1), (-1, 0)],
            dir: 0,
            pos: (0, 0),
            infection_counter: 0
        }
    }

    fn rot_r(&mut self) {
        self.dir += 1 + 4;
        self.dir %= 4;
    }

    fn rot_l(&mut self) {
        self.dir += -1 + 4;
        self.dir %= 4;
    }

    fn forward(&mut self) {
        let d = self.dirs[self.dir as usize];
        self.pos = (self.pos.0 + d.0, self.pos.1 + d.1)
    }

    fn step(&mut self, infected: &mut HashSet<(i32, i32)>) {

        match infected.get(&self.pos) {
            Some(_) => {
                self.rot_r();
                infected.remove(&self.pos);
            },
            None => {
                self.rot_l();
                infected.insert(self.pos);
                self.infection_counter += 1;
            }
        }
        self.forward();
    }
}

fn parse_input(input: &Vec<String>) -> HashSet<(i32, i32)> {
    let input = input.iter().map(|l| l.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    let mut infected = HashSet::new();
    let x_size = input[0].len();
    let y_size = input.len();

    for y in 0..y_size {
        for x in 0..x_size {
            if input[y][x] == '#' {
                infected.insert((x as i32 - x_size as i32 / 2, -(y as i32- y_size as i32 / 2)));
            }
        }
    }
    infected
}

pub fn part1(input: &Vec<String>) -> i64 {

    let mut infected = parse_input(input);
    let mut virus = Virus::new();

    let iter = 10000;
    for _ in 0..iter {
        virus.step(&mut infected);
    }

    let res= virus.infection_counter;

    return res as i64;
}