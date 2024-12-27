pub fn part1(input: &Vec<String>) -> i64 {
    
    let line = input[0].clone();

    let mut floor = 0;
    for c in line.chars() {
        if c == '(' {
            floor += 1;
        } else {
            floor -= 1;
        }
    }

    return floor;

}