fn get_div(row: &Vec<i64>) -> i64 {
    for i in 0..row.len() {
        for j in i+1..row.len() {
            let (a,b) = (row[i], row[j]);
            if a > b {
                if a % b == 0 {
                    return a / b;
                }
            } else {
                if b % a == 0 {
                    return b / a;
                }
            }
        }
    }
    return 0;
}

pub fn part2(input: &Vec<String>) -> i64 {

    let rows = input.iter().map(|l| l.trim().split_whitespace().collect::<Vec<_>>().iter().map(|x| x.parse::<i64>().unwrap()).collect::<Vec<_>>()).collect::<Vec<_>>();

    let mut sum = 0;
    for row in rows.iter() {
        sum += get_div(row);
    }

    return sum;
}