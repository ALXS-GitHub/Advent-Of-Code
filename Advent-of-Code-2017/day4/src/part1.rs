fn is_valid(passphrase: &Vec<&str>) -> bool {
    for i in 0..passphrase.len() {
        for j in (i+1)..passphrase.len() {
            if passphrase[i] == passphrase[j] {
                return false;
            }
        }
    }
    return true
}

pub fn part1(input: &Vec<String>) -> i64 {

    let passphrases = input.iter().map(|l| l.split_whitespace().collect::<Vec<_>>()).collect::<Vec<_>>();

    let mut counter = 0;

    for passphrase in passphrases.iter() {
        if is_valid(passphrase) {
            counter += 1;
        }
    }

    return counter;
}