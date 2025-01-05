use std::{collections::HashSet, f64::MIN_POSITIVE};

fn parse_input(input: &Vec<String>) -> Vec<i64> {
    let mut packages = vec![];

    for line in input {
        packages.push(line.parse::<i64>().unwrap());
    }

    return packages;
}

fn find_unique_subsets(available_packages: &[i64], target_size: i64) -> Vec<Vec<i64>> {
    let mut results = Vec::new();
    let mut current_subset = Vec::new();

    backtrack(&available_packages, target_size, &mut current_subset, 0, &mut results);

    results
}

fn backtrack( packages: &[i64], remaining: i64, current_subset: &mut Vec<i64>, start_index: usize, results: &mut Vec<Vec<i64>>) {
    if remaining == 0 {
        results.push(current_subset.clone());
        return;
    }

    for i in start_index..packages.len() {
        if i > start_index && packages[i] == packages[i - 1] {
            continue;
        }

        if packages[i] > remaining {
            break;
        }

        current_subset.push(packages[i]);
        backtrack(packages, remaining - packages[i], current_subset, i + 1, results);
        current_subset.pop(); 
    }
}

fn get_quantum_entanglement(arrangement: Vec<i64>) -> i64 {
    return arrangement.into_iter().product();
}

pub fn part1(input: &Vec<String>) -> i64 {

    let packages = &parse_input(input);
    let groups = 3;

    let size = packages.into_iter().sum::<i64>() / groups;

    let arrangements = find_unique_subsets(packages, size);
    
    let min_len = arrangements.iter().map(|x| x.len()).min().unwrap();
    let min_arrangements = arrangements.into_iter().filter(|x| x.len() == min_len).collect::<Vec<_>>();

    let min_entanglement = min_arrangements.iter().map(|x| get_quantum_entanglement(x.to_vec())).min().unwrap();
    return min_entanglement;
}