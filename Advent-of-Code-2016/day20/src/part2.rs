use std::cmp::{min, max};

fn get_max_ranges(ranges: Vec<(u32, u32)>) -> Vec<(u32, u32)> {

    let mut new_ranges = Vec::new();

    for r in ranges.iter() {

        let mut inserted = false;

        for i in 0..new_ranges.len() {
            let nr: (u32, u32) = new_ranges[i];
            if r.0 >= nr.0 && r.0 <= nr.1 {
                new_ranges[i] = (nr.0, nr.1.max(r.1));
                inserted = true;
                break
            }
            if r.1 <= nr.1 && r.1 >= nr.0 {
                new_ranges[i] = (nr.0.min(r.0), nr.1);
                inserted = true;
                break
            }
            if r.0 <= nr.0 && r.1 >= nr.1 {
                new_ranges[i] = (r.0, r.1);
                inserted = true;
                break
            }
        }

        if !inserted {
            new_ranges.push(*r);
        }
    }

    new_ranges

}

fn count_valid(ranges: Vec<(u32, u32)>) -> u32 {
    let mut counter = 0;

    for i in 0..ranges.len() - 1{
        counter += ranges[i+1].0 - ranges[i].1 - 1;
    }

    counter
}

pub fn part2(input: &Vec<String>) -> i64 {

    let ranges = input.iter().map(|l| {
        let l = l.split_once("-").unwrap();
        let (min, max) = (l.0.parse::<u32>().unwrap(), l.1.parse::<u32>().unwrap());
        (min, max)
    }).collect::<Vec<_>>();

    let mut new_ranges = get_max_ranges(ranges);
    new_ranges.sort();
    // doing a second pass to merge ranges
    let mut new_ranges = get_max_ranges(new_ranges);
    new_ranges.sort();

    return count_valid(new_ranges) as i64;
}