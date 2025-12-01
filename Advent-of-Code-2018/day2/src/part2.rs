fn diff(a: &String, b: &String) -> (i64, usize) {

    let mut diff_count = 0;
    let mut last_diff_idx = 0;
    let a = a.chars().collect::<Vec<_>>();
    let b = b.chars().collect::<Vec<_>>();

    for i in 0..a.len() {
        if a[i] != b[i] {
            diff_count += 1;
            last_diff_idx = i;
        }
    }

    (diff_count, last_diff_idx)

}

pub fn part2(input: &Vec<String>) -> String {

    let mut output = String::new();

    for i in 0..input.len() {
        for j in i+1..input.len() {
            let (diff, idx) = diff(&input[i], &input[j]);
            if diff == 1 {
                output = input[i].clone();
                output.remove(idx);
                break
            }
        }
    }

    return output;
}