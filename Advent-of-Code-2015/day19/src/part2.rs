use regex::Regex;

fn parse_input(input: &Vec<String>) -> (Vec<(String, String)>, String) {

    let mut second_part = false;
    let mut molecule = String::new();
    let mut replacements = Vec::new();

    for line in input {
        if line == "" {
            second_part = true;
        }

        if !second_part {
            let parts = line.split(" => ").collect::<Vec<&str>>();
            replacements.push((parts[0].to_string(), parts[1].to_string()));
        } else {
            molecule = line.clone();
        }
    }

    (replacements, molecule)
}

fn reverse_replace(replacements: &Vec<(String, String)>, molecule: &String) -> (String, i64){
    let mut new_molecule = molecule.clone();
    let mut count = 0;

    for (pattern, replacement) in replacements.iter() {
        let replacement = &format!(r"{}", replacement);
        let r = Regex::new(replacement).unwrap();
        loop {
            let clone_mol = new_molecule.clone();
            let caps = r.find_iter(&clone_mol).map(|x| (x.as_str(), x.start(), x.end())).collect::<Vec<_>>();
            if caps.is_empty() { break }
            let cap = caps[0];
            new_molecule.replace_range(cap.1..cap.2, pattern);
            count += 1;
        }
    }

    return (new_molecule, count)
}

pub fn part2(input: &Vec<String>) -> i64 {
    let (replacements, molecule) = parse_input(input);

    let mut prev_len = molecule.len();
    let (mut res, mut count) = reverse_replace(&replacements, &molecule);
    let mut new_len = res.len();
    let mut c = 0;
    while prev_len != new_len {
        (res, c) = reverse_replace(&replacements, &res);
        count += c;
        prev_len = new_len;
        new_len = res.len();
    }

    return count;
}