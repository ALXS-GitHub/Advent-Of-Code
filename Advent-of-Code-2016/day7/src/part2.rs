use std::collections::HashSet;

fn is_valid(ip: &String) -> bool {

    let chars: Vec<char> = ip.chars().collect();
    let mut in_brackets = false;
    let mut supernet = HashSet::new();
    let mut hypernet = HashSet::new();

    for window in chars.windows(3) {

        if window[0] == '[' {
            in_brackets = true;
        } else if window[0] == ']' {
            in_brackets = false;
        }

        if !in_brackets && is_aba(window.try_into().unwrap()) {
            supernet.insert(window);
        } else if in_brackets && is_aba(window.try_into().unwrap()) {
            hypernet.insert(window);
        }
    }

    return has_match(&supernet, &hypernet);

}

fn has_match(supernet: &HashSet<&[char]>, hypernet: &HashSet<&[char]>) -> bool {
    for aba in supernet {
        for bab in hypernet {
            if matches(aba, bab) {
                return true;
            }
        }
    }
    return false
}

fn matches(aba: &[char], bab: &[char]) -> bool {
    return aba[1] == bab[0] && aba[0] == bab[1]
}

fn is_aba(chars: &[char; 3]) -> bool {
    return chars[0] == chars[2] && chars[0] != chars[1]
}

pub fn part2(input: &Vec<String>) -> i64 {

    let mut counter = 0;
    for ip in input {
        if is_valid(ip) {
            counter += 1;
        }
    }

    return counter;
}