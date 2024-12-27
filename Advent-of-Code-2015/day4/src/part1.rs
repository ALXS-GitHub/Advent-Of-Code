fn md5(input: String) -> String {
    let digest = md5::compute(input);
    return format!("{:x}", digest);
}

fn concat(input: String, num: i64) -> String {
    return format!("{}{}", input, num);
}

fn has_5_leading_0(hash: String) -> bool {
    if hash.len() < 5 {
        return false;
    }

    if hash[0..5] == *"00000" {
        return true;
    }

    false
}

pub fn part1(input: &Vec<String>) -> i64 {

    let input = &input[0];

    let mut res = 0;

    loop {
        res += 1;

        let string = concat(input.to_string(), res);
        let hash = md5(string);
        if has_5_leading_0(hash) {
            break;
        }

    }

    return res;
}