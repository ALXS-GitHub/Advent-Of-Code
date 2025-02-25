fn complete_circular_buffer(steps: i32, times: i32) -> i32 {
    // let mut buffer = vec![0];
    let mut buffer_len = 1;
    let mut current_pos = 0;
    let mut pos_0 = 0;
    let mut after_0 = 0;

    for t in 1..=times {
        current_pos = (current_pos + steps as usize) % buffer_len;
        if current_pos == pos_0 {
            after_0 = t;
        }
        if current_pos < pos_0 {
            pos_0 += 1;
        }
        current_pos += 1;
        buffer_len += 1;
    }

    return after_0;
}

pub fn part2(input: &Vec<String>) -> i32 {

    let steps = input[0].parse::<i32>().unwrap();
    let times = 50000000;

    let res = complete_circular_buffer(steps, times);

    return res;
}