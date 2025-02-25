fn get_diff(row: &Vec<i64>) -> i64 {
    let max = row.iter().max().unwrap();
    let min = row.iter().min().unwrap();
    return max - min;
}

pub fn part1(input: &Vec<String>) -> i64 {

    let rows = input.iter().map(|l| l.trim().split_whitespace().collect::<Vec<_>>().iter().map(|x| x.parse::<i64>().unwrap()).collect::<Vec<_>>()).collect::<Vec<_>>();

    let mut sum = 0;
    for row in rows.iter() {
        sum += get_diff(row);
    }

    return sum;
}