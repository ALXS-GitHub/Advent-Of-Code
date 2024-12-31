fn get_next_num(num: Vec<char>) -> Vec<char> {

    let mut counter = 0;
    let mut previous = num[0];
    let mut res = vec![];


    for c in num {
        if c == previous {
            counter += 1;
        } else {
            res.extend(format!("{}", counter).chars());
            res.push(previous);
            counter = 1;
        }
        previous = c;
    }

    res.extend(format!("{}", counter).chars());
    res.push(previous);

    res
}

pub fn part1(input: &Vec<String>) -> i64 {

    let mut num: Vec<char> = input[0].chars().collect::<Vec<char>>();
    let times = 40;

    for _ in 0..times {
        num = get_next_num(num);
    }


    return num.len() as i64;
}