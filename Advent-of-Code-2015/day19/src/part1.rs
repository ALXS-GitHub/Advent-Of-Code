use std::collections::HashSet;
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

fn find_nb_replacements(replacements: &Vec<(String, String)>, molecule: &String) -> i64 {

    let mut set: HashSet<String> = HashSet::new();

    for (pattern, replacement) in replacements.iter() {
        let pattern = &format!(r"{}", pattern);
        let r = Regex::new(pattern).unwrap();
        let caps = r.find_iter(&molecule).map(|x| (x.as_str(), x.start(), x.end())).collect::<Vec<_>>();
        for cap in caps {
            let mut new_molecule = molecule.clone();
            new_molecule.replace_range(cap.1..cap.2, replacement);
            set.insert(new_molecule);
        }
    }

    return set.len() as i64;

}

pub fn part1(input: &Vec<String>) -> i64 {

    let (replacements, molecule) = parse_input(input);

    let res = find_nb_replacements(&replacements, &molecule);

    return res;
}