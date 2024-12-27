pub fn part2(input: &Vec<String>) -> i64 {
    let line = input[0].clone();

    let mut floor = 0;
    for (i,c) in line.chars().enumerate() {
        if c == '(' {
            floor += 1;
        } else {
            floor -= 1;
        }
        if floor == -1 {
            return i as i64 +1;
        }
    }

    return 0;
}