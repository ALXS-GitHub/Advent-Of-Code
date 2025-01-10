fn decompress(input: &String) -> String {
    let mut decompressed = String::new();
    let chars = input.chars().collect::<Vec<char>>();

    let n = input.len();
    let mut i = n;

    while i > 0 {

        let c = chars[n - i];

        if c == '(' {
            let mut buffer = String::new();
            let mut _i = i - 1;

            while chars[n - _i] != ')' {
                buffer.push(chars[n - _i]);
                _i -= 1;
            }
            // after loop _i -> ')'

            let (sub, times) = buffer.split_once("x").unwrap();
            let sub = sub.parse::<i64>().unwrap();
            let times = times.parse::<i64>().unwrap();

            i = _i - 1;

            let mut subsequent = String::new();
            
            for _ in 0..sub {
                subsequent.push(chars[n - i]);
                i -= 1;
            }
            for _ in 0..times {
                decompressed += &subsequent;
            }

        } else {
            decompressed.push(c);
            i -= 1;
        }

    }
    decompressed
}

pub fn part1(input: &Vec<String>) -> i64 {

    let decompressed = decompress(&input[0]);

    return decompressed.len() as i64;
}