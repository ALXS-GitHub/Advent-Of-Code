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

pub fn part2(input: &Vec<String>) -> i64 {
    let mut dial: i64 = 50;
    let mut counter: i64 = 0;
    let offset: i64 = 1_000_000_000; 

    let rotations = read_input(input);
    for rotation in rotations {
        let degrees = rotation.degrees;
        
        match rotation.direction {
            'L' => {
                let start_pos = dial;
                let end_pos = dial - degrees;

                let zeros_passed = ((start_pos - 1 + offset) / 100) - ((end_pos - 1 + offset) / 100);

                counter += zeros_passed;
                dial = ((end_pos % 100) + 100) % 100;
            }
            'R' => {
                let start_pos = dial;
                let end_pos = dial + degrees;
                
                let zeros_passed = (end_pos / 100) - (start_pos / 100);

                counter += zeros_passed;
                dial = ((end_pos % 100) + 100) % 100;
            }
            _ => {}
        }   
    }
    return counter;
}