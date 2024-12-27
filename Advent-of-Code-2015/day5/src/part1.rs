fn is_nice(s: String) -> bool {

    let forbidden = vec!["ab", "cd", "pq", "xy"];
    let vowels = vec!['a', 'e', 'i', 'o', 'u'];
    let mut vowels_count = 0;
    let mut doubles_count = 0;

    for i in 0..s.len() - 1 {
        let c = s.chars().nth(i).unwrap();
        let next_c = s.chars().nth(i+1).unwrap();
        if vowels.contains(&c) {
            vowels_count += 1;
        }

        if c == next_c {
            doubles_count += 1;
        }

        if forbidden.contains(&format!("{}{}", c, next_c).as_str()) {
            return false;
        }
    }

    let last_c = s.chars().nth(s.len() - 1).unwrap();
    if vowels.contains(&last_c) {
        vowels_count += 1;
    }

    if doubles_count >= 1 && vowels_count >= 3 {
        return true;
    }

    false
}

pub fn part1(input: &Vec<String>) -> i64 {

    let mut total = 0;

    for s in input.iter().cloned() {
        if is_nice(s) {
            total += 1;
        }
    }

    return total;
}