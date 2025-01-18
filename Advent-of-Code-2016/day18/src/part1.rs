fn compute_next_row(row: &String) -> String {

    let mut new_row = String::new();
    let chars: Vec<char> = std::iter::once('.')
    .chain(row.chars())
    .chain(std::iter::once('.'))
    .collect();

    for w in chars.windows(3) {
        let new_tile = match w {
            ['.', '.', '^'] => '^',
            ['.', '^', '^'] => '^',
            ['^', '^', '.'] => '^',
            ['^', '.', '.'] => '^',
            _ => '.',
        };

        new_row.push(new_tile);
    }

    new_row

}

fn get_nb_safe(row: &String) -> i64 {
    let chars = row.chars();

    let mut count = 0;

    for c in chars {
        if c == '.' {
            count += 1;
        }
    }

    count
}

pub fn part1(input: &Vec<String>) -> i64 {

    let mut row = input[0].clone();
    let num_rows = 40;

    let mut total = 0;

    total += get_nb_safe(&row);

    for _ in 1..num_rows {
        let new_row = compute_next_row(&row);
        total += get_nb_safe(&new_row);
        row = new_row;
    }

    return total;
}