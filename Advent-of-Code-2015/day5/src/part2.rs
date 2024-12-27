fn is_nice(s: String) -> bool {

    let mut doubles_count = 0;
    let mut found_pattern = false;

    for i in 0..s.len() - 2 {
        let c = s.chars().nth(i).unwrap();
        let next_c = s.chars().nth(i+1).unwrap();
        let next_next_c = s.chars().nth(i+2).unwrap();

        if c == next_next_c {
            doubles_count += 1;
        }

        let reduced_s1 = s[0..i].to_string();
        let reduced_s2 = s[i+2..s.len()].to_string();
        let pattern = format!("{}{}", c, next_c);
        if reduced_s1.contains(&pattern) || reduced_s2.contains(&pattern) {
            found_pattern = true;
        }

    }

    if doubles_count >= 1 && found_pattern {
        return true;
    }

    false
}


pub fn part2(input: &Vec<String>) -> i64 {
    let mut total = 0;

    for s in input.iter().cloned() {
        if is_nice(s) {
            total += 1;
        }
    }

    return total;
}