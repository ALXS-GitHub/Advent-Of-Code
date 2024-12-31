use regex::Regex;

fn get_sum(line: String) -> i64 {

    let r = Regex::new(r"-?\d+").unwrap();
    let mut sum = 0;

    for num in  r.find_iter(&line) {
        let n = num.as_str().parse::<i64>().unwrap();
        sum += n;
    }

    return sum;

}

pub fn part1(input: &Vec<String>) -> i64 {
    return get_sum(input[0].clone());
}