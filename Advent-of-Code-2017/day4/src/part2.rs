fn is_valid(passphrase: &Vec<&str>) -> bool {
    for i in 0..passphrase.len() {
        for j in (i+1)..passphrase.len() {
            if is_anagram(passphrase[i], passphrase[j]) {
                return false;
            }
        }
    }
    return true
}

fn is_anagram(w1: &str, w2: &str) -> bool {
    let mut w1_chars: Vec<char> = w1.chars().collect();
    let mut w2_chars: Vec<char> = w2.chars().collect();

    w1_chars.sort();
    w2_chars.sort();

    return w1_chars == w2_chars;
}

pub fn part2(input: &Vec<String>) -> i64 {

    let passphrases = input.iter().map(|l| l.split_whitespace().collect::<Vec<_>>()).collect::<Vec<_>>();

    let mut counter = 0;

    for passphrase in passphrases.iter() {
        if is_valid(passphrase) {
            counter += 1;
        }
    }

    return counter;
}