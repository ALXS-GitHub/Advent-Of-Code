use std::collections::HashMap;

fn count_2_3_repeats(string: &String) -> (bool, bool) {

    let chars = string.chars();
    let mut counts = HashMap::new();

    for c in chars {
        *counts.entry(c).or_insert(0) += 1;
    }

    let mut two = false;
    let mut three = false;

    for (_, value) in counts {

        if value == 2 {
            two = true;
        } else if value == 3 {
            three = true;
        }

        if two && three { break }
    }

    return (two, three)

}

pub fn part1(input: &Vec<String>) -> i64 {

    let mut two = 0;
    let mut three = 0;

    for string in input.iter() {
        let (b2, b3) = count_2_3_repeats(string);
        if b2 { two += 1}
        if b3 { three += 1}
    }

    return two * three;
}