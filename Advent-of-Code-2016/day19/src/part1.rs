#[derive(Debug, Clone)]
struct Elve {
    id: usize,
    presents: usize,
}

fn get_winner(elves: &mut Vec<Elve>) -> usize {

    let n = elves.len();
    while elves.len() > 1 {
        let mut remaining_elves: Vec<Elve> = Vec::new();
        for i in 0..elves.len() {
            if elves[i].presents == 0 {
                continue
            }

            let mut new_elve = elves[i].clone();

            let mut j = i + 1;
            if j >= elves.len() {
                j = 0;
                new_elve.presents += remaining_elves[j].presents;
                remaining_elves = remaining_elves[1..].into();
            } else {
                new_elve.presents += elves[j].presents;
            }

            elves[j].presents = 0;
            remaining_elves.push(new_elve);
        } 

        *elves = remaining_elves;
    }

    assert_eq!(elves[0].presents, n);
    elves[0].id

}

pub fn part1(input: &Vec<String>) -> i64 {

    let num_elves = input[0].parse::<usize>().unwrap();

    let mut elves: Vec<Elve> = (1..=num_elves)
        .map(|i| Elve {
            id: i,
            presents: 1,
        })
        .collect();

    let res = get_winner(&mut elves);

    return res as i64;
}