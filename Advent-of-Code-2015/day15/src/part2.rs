use regex::Regex;
use itertools::Itertools;

fn parse_input(input: &Vec<String>) -> Vec<Vec<i64>> {
    let mut matrix = vec![];
    let r = Regex::new(r"-?\d").unwrap();

    for line in input {
        let caps: Vec<i64> = r.find_iter(&line).map(|m| m.as_str().parse::<i64>().unwrap()).collect();
        assert_eq!(caps.len(), 5);
        matrix.push(caps);
    }

    matrix
}

fn get_score(ingredients: &Vec<Vec<i64>>, quantities: &Vec<i64>) -> i64 {

    let mut scores = vec![0; 4];
    let mut cals = 0;
    for i in 0..ingredients.len() {
        for j in 0..ingredients[0].len() - 1 {
            scores[j] += ingredients[i][j] * quantities[i];
        }
        cals += ingredients[i][4] * quantities[i];
    }

    if cals != 500 {
        return 0;
    }

    for i in 0..scores.len() {
        if scores[i] < 0 {
            scores[i] = 0;
        }
    }

    return scores.iter().product();
}

fn get_highest_score(ingredients: &Vec<Vec<i64>>, layer: i64, quantities: Vec<i64>) -> i64 {

    let n = quantities.len();
    let remainder = 100 - quantities.iter().sum::<i64>();
    let mut quantities = quantities.clone();

    if layer == 0 {
        quantities[n - 1] = remainder;
        return get_score(ingredients, &quantities);
    }

    let mut max = 0;
    for i in 0..remainder {
        quantities[n - 1 - layer as usize] = i;
        let current = get_highest_score(ingredients, layer - 1, quantities.clone());
        if current > max {
            max = current;
        }
    }

    return max;

}

pub fn part2(input: &Vec<String>) -> i64 {

    let ingredients = parse_input(input);
    let num_ingredients = ingredients.len();

    let res = get_highest_score(&ingredients, num_ingredients as i64 - 1, vec![0; num_ingredients]);

    return res;
}