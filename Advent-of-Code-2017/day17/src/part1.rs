fn complete_circular_buffer(steps: i64, times: i64) -> i64 {
    let mut buffer = vec![0];
    let mut current_pos = 0;

    for t in 1..=times {
        current_pos = (current_pos + steps as usize + 1)  % buffer.len();
        buffer.insert(current_pos, t);
    }

    let res_pos = (current_pos + 1) % buffer.len();

    return buffer[res_pos];
}

pub fn part1(input: &Vec<String>) -> i64 {

    let steps = input[0].parse::<i64>().unwrap();
    let times = 2017;

    let res = complete_circular_buffer(steps, times);

    return res;
}