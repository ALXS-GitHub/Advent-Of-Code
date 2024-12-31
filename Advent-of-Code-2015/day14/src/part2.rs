use std::collections::HashMap;
use regex::Regex;

#[derive(Debug, Clone)]
struct Reindeer {
    name: String,
    speed: i64,
    fly_time: i64,
    rest_time: i64,
    remaining_fly_time: i64,
    remaining_rest_time: i64,
    flying: bool,
    distance: i64,
    points: i64,
}

fn parse_input(input: &Vec<String>) -> Vec<Reindeer> {

    let mut reindeers = vec![];
    let r = Regex::new(r"^(\w+) [\w\s]+ (\d+) [\w\s/]+ (\d+) [\w\s/,]+ (\d+)").unwrap();

    for line in input.iter() {
        let caps = r.captures(line).unwrap();
        let name = caps.get(1).unwrap().as_str().to_string();
        let speed =  caps.get(2).unwrap().as_str().parse::<i64>().unwrap();
        let fly_time =  caps.get(3).unwrap().as_str().parse::<i64>().unwrap();
        let rest_time = caps.get(4).unwrap().as_str().parse::<i64>().unwrap();
        let reindeer = Reindeer {
            name,
            speed,
            fly_time,
            rest_time,
            remaining_fly_time: fly_time,
            remaining_rest_time: 0,
            flying: true,
            distance: 0,
            points: 0
        };
        reindeers.push(reindeer);
    }

    return reindeers;
}

fn next_step(reindeer: &mut Reindeer) {
    if reindeer.flying {
        reindeer.remaining_fly_time -= 1;
        reindeer.distance += reindeer.speed;
        if reindeer.remaining_fly_time == 0 {
            reindeer.flying = false;
            reindeer.remaining_rest_time = reindeer.rest_time;
        }
    } else {
        reindeer.remaining_rest_time -= 1;
        if reindeer.remaining_rest_time == 0 {
            reindeer.flying = true;
            reindeer.remaining_fly_time = reindeer.fly_time;
        }
    }
}

fn simulate(reindeers: &mut Vec<Reindeer>, max_time: i64) -> Vec<Reindeer> {

    for time in 0..max_time {
        for reindeer in reindeers.into_iter() {
            next_step(reindeer);
        }
        distribute_points(reindeers);
    }

    return reindeers.to_vec();
}

fn get_longest_distance(reindeers: Vec<Reindeer>) -> i64 {

    let mut max = 0;

    for reindeer in reindeers {
        if reindeer.distance > max {
            max = reindeer.distance;
        }
    }
    max
}

fn distribute_points(reindeers: &mut Vec<Reindeer>) {
    let max = get_longest_distance(reindeers.to_vec());

    for reindeer in reindeers {
        if reindeer.distance == max {
            reindeer.points += 1;
        }
    }

}

fn get_most_points(reindeers: Vec<Reindeer>) -> i64 {
    let mut max = 0;

    for reindeer in reindeers {
        if reindeer.points > max {
            max = reindeer.points;
        }
    }
    max
}


pub fn part2(input: &Vec<String>) -> i64 {
    let mut reindeers = parse_input(input);

    let max_time = 2503;
    simulate(&mut reindeers, max_time);

    return get_most_points(reindeers);
}