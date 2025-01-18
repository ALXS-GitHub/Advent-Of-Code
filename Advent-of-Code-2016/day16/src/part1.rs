fn dragon_curve(data: &mut String, length: usize) {

    while data.len() < length {
        let data_copy = data.clone();
        let new_data: String = data_copy
        .chars()
        .rev()
        .map(|c| match c {
            '0' => '1',
            '1' => '0',
            _ => panic!("Unexpected character"),
        })
        .collect();
        data.push_str(&"0");
        data.push_str(&new_data);
    }
}

fn checksum(data: &mut String) {
    while data.len() % 2 == 0 {
        let chars = data.chars().collect::<Vec<_>>();
        let mut new_checksum = String::new();

        for pair in chars.chunks(2) {
            if pair[0] == pair[1] {
                new_checksum.push('1');
            } else {
                new_checksum.push('0');
            }
        }

        *data = new_checksum;
    }
}

pub fn part1(input: &Vec<String>) -> String {

    let length: usize = 272;
    let mut data = input[0].clone();
    // println!("{}", data.len());

    dragon_curve(&mut data, length);

    data = data[..length].to_string();

    // println!("{}", data.len());

    checksum(&mut data);

    // println!("{}", data.len());

    return data;
}