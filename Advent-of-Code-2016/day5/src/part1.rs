fn md5(input: &String) -> String {
    let digest = md5::compute(input);
    return format!("{:x}", digest);
}

pub fn part1(input: &Vec<String>) -> String {

    let door_id = &input[0];
    let mut password = String::new();
    let mut counter = 0;

    while password.len() < 8 {
        let current = format!("{}{}", &door_id, counter);
        let hash = md5::compute(&current);
        if hash.0[0] == 0 && hash.0[1] == 0 && hash.0[2] <= 0x0F {
            let hash_hex = format!("{:x}", hash);
            if let Some(c) = hash_hex.chars().nth(5) {
                password.push(c);
            }
        }
        counter += 1;
    }

    return password;
}