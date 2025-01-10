pub fn part2(input: &Vec<String>) -> String {
    let door_id = &input[0];
    let mut password = vec![' '; 8];
    let mut counter = 0;

    while password.iter().any(|&c| c == ' ') {
        let current = format!("{}{}", &door_id, counter);
        let hash = md5::compute(&current);
        if hash.0[0] == 0 && hash.0[1] == 0 && hash.0[2] <= 0x0F {
            let hash_hex = format!("{:x}", hash);
            if let (Some(pos), Some(c)) = (hash_hex.chars().nth(5), hash_hex.chars().nth(6)) {
                let pos = pos.to_string().parse::<usize>().unwrap_or(8);
                if 0 <= pos && pos <= 7 && password[pos] == ' ' {
                    password[pos] = c;
                }
            }
        }
        counter += 1;
    }

    return password.iter().collect::<String>();
}