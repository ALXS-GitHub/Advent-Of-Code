pub struct Rotation {
    pub direction: char,
    pub degrees: i64,
}

pub fn read_input(input: &Vec<String>) -> Vec<Rotation> {
    let mut rotations: Vec<Rotation> = Vec::new();
    for line in input {
        let direction = line.chars().next().unwrap();
        let degrees: i64 = line[1..].parse().unwrap();
        rotations.push(Rotation {
            direction,
            degrees,
        });
    }
    rotations
}   

pub fn part1(input: &Vec<String>) -> i64 {

    let mut dial = 50;
    let mut counter = 0;

    let rotations = read_input(input);
    for rotation in rotations {
        match rotation.direction {
            'L' => {
                dial -= rotation.degrees;
                dial = (dial % 100 + 100) % 100;
            }
            'R' => {
                dial += rotation.degrees;
                dial = (dial % 100 + 100) % 100;
            }
            _ => {}
        }
        if dial == 0 {
            counter += 1;
        }
    }

    return counter;
}