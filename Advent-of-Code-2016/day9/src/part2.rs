fn decompressed_size(input: &str, depth: i64) -> usize {
    let chars: Vec<char> = input.chars().collect();

    let n = chars.len();
    let mut i = 0;
    let mut size = 0;

    while i < n {
        if chars[i] == '(' {

            let mut buffer = String::new();
            let mut _i = i + 1;

            while chars[_i] != ')' {
                buffer.push(chars[_i]);
                _i += 1;
            }
            // after loop _i -> ')'

            let (sub, times) = buffer.split_once('x').unwrap();
            let sub = sub.parse::<usize>().unwrap();
            let times = times.parse::<usize>().unwrap();

            let subsequent = &input[_i + 1.._i + 1 + sub];
            size += decompressed_size(subsequent, depth + 1) * times;

            i = _i + 1 + sub;
        } else {
            size += 1;
            i += 1;
        }
    }

    size
}

pub fn part2(input: &Vec<String>) -> i64 {

    let decompressed = decompressed_size(&input[0], 0);

    return decompressed as i64;
}