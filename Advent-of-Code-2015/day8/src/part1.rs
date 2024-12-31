fn unescape(s: String) -> String {
    let mut result = String::new();
    let chars: Vec<char> = s.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == '\\' && i + 1 < chars.len() {
            match chars[i + 1] {
                'x' if i + 3 < chars.len() => {
                    let hex = &s[i+2..i+4];
                    if let Ok(byte) = u8::from_str_radix(hex, 16) {
                        result.push(byte as char);
                        i += 4;
                        continue;
                    }
                },
                '"' => {
                    result.push('"');
                    i += 2;
                    continue;
                },
                '\\' => {
                    result.push('\\');
                    i += 2;
                    continue;
                },
                _ => {}
            }
        }
        result.push(chars[i]);
        i+=1;

    }
    result
}

pub fn part1(input: &Vec<String>) -> i64 {

    let mut total = 0;

    for line in input {
        let line_trim = line[1..line.len() - 1].to_string();
        let value: String = unescape(line_trim.clone());
        // println!("{} - {:?}", line, value.chars());
        let value_chars: Vec<char> = value.chars().collect();
        // println!("{} - {}", line.len(), value_chars.len());
        total += line.len() - value_chars.len();
    }

    return total as i64;
}