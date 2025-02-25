fn execute(jumps: &mut Vec<i64>) -> i64 {
    let mut counter = 0;
    let mut cursor: usize = 0;

    while cursor < jumps.len() {
        counter += 1;
        let v = jumps[cursor];
        if v >= 3 {
            jumps[cursor] -= 1;
        } else {
            jumps[cursor] += 1;
        }
        cursor = (cursor as i64 + v) as usize;
    } 

    counter
}

pub fn part2(input: &Vec<String>) -> i64 {

    let mut jumps = input.iter().map(|l| l.parse::<i64>().unwrap()).collect::<Vec<_>>();

    let res = execute(&mut jumps);

    return res;
}