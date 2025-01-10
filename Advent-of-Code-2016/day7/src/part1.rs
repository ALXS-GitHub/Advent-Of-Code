fn is_valid(ip: &String) -> bool {

    let chars: Vec<char> = ip.chars().collect();
    let mut abba_counter = 0;
    let mut in_brackets = false;

    for window in chars.windows(4) {

        if window[0] == '[' {
            in_brackets = true;
        } else if window[0] == ']' {
            in_brackets = false;
        }

        if !in_brackets && is_abba(window.try_into().unwrap()) {
            abba_counter += 1;
        } else if in_brackets && is_abba(window.try_into().unwrap()) {
            return false;
        }
    }

    return abba_counter >= 1;

}

fn is_abba(chars: &[char; 4]) -> bool {
    return chars[0] == chars[3] && chars[1] == chars[2] && chars[0] != chars[1]
}

pub fn part1(input: &Vec<String>) -> i64 {

    let mut counter = 0;
    for ip in input {
        if is_valid(ip) {
            counter += 1;
        }
    }

    return counter;
}