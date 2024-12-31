fn increment(password: String) -> String {

    let mut chars: Vec<char> = password.chars().collect::<Vec<char>>();
    let mut i = chars.len();

    while i > 0 {
        i -= 1;

        if chars[i] == 'z' {
            chars[i] = 'a';
        } else {
            chars[i] = ((chars[i] as u8) + 1) as char;
            break;
        }

    }

    chars.iter().collect()
}

fn has_sequence(password: String) -> bool {

    let chars = password.chars().collect::<Vec<char>>();

    for i in 0..password.len() - 2 {
        let c = chars[i];
        let c1 = chars[i+1];
        let c2 = chars[i+2];

        if c1 == (c as u8 + 1) as char && c2 == (c as u8 + 2) as char {
            return true;
        }
    }

    return false
}

fn has_other_pair(password: String, forbidden: char) -> bool {

    let mut chars = password.chars().collect::<Vec<char>>();

    for i in 0..password.len() - 1 {
        let c = chars[i];

        if c == forbidden {
            continue
        }

        let c1 = chars[i+1];

        if c == c1 {
            return true;
        }
    }

    return false
}

fn has_double_pair(password: String) -> bool {

    let mut chars = password.chars().collect::<Vec<char>>();

    for i in 0..password.len() - 1 {
        
        let c = chars[i];
        let c1 = chars[i+1];

        if c == c1 {
            if has_other_pair(password.clone(), c) {
                return true;
            }
        }
    }
    return false
}

fn is_valid(password: String) -> bool {

    if password.contains('i') || password.contains('l') || password.contains('o') {
        return false;
    }

    let res = has_double_pair(password.clone()) && has_sequence(password);

    return res

}

pub fn part1(input: &Vec<String>) -> String {

    let mut password = input[0].clone();

    while !is_valid(password.clone()) {
        password = increment(password);
    }

    return password;
}