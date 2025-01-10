use std::cmp::Ordering;
use std::collections::HashMap;

fn parse_input(input: &Vec<String>) -> Vec<(String, i64, [char; 5])> {

    let mut rooms = Vec::new();

    for line in input {
        let (name, checksum) = line.split_once("[").unwrap();
        let name = name.replace("-", "");
        let (name, num) = name.split_at(name.len() - 3);
        let name = rotate(name, num.parse::<i64>().unwrap() as usize);
        let checksum = &checksum[..5];
        let checksum: [char; 5] = checksum.chars().collect::<Vec<char>>().try_into().unwrap();

        rooms.push((name.to_string(), num.parse::<i64>().unwrap(), checksum));
    }

    rooms
}

fn sort_char_map(map: &HashMap<char, i64>) -> Vec<(char, i64)> {
    let mut vec: Vec<(char, i64)> = map.iter().map(|(&k, &v)| (k,v)).collect();

    vec.sort_by(|a, b| {
        b.1.cmp(&a.1)
            .then_with(|| a.0.cmp(&b.0))
    });

    vec
}

fn get_char_map(name: &String) -> HashMap<char, i64> {
    let chars = name.chars();
    let mut map: HashMap<char, i64> = HashMap::new(); 
    for c in chars {
        *map.entry(c).or_insert(0) += 1;
    }
    map
}

fn is_real(room: &(String, i64, [char; 5])) -> bool {
    let char_map = get_char_map(&room.0);
    let sorted = sort_char_map(&char_map);
    let sorted = &sorted.iter().map(|(c, _)| *c).collect::<Vec<_>>();
    let sorted: &[char; 5] = sorted[..5].try_into().unwrap();

    if *sorted == room.2 {
        return true;
    }
    return false

}

fn rotate(name: &str, rotations: usize) -> String {
    name.chars().map(|c| {
        let rotated = ((c as u64 - b'a' as u64 + rotations as u64) % 26) as u8 + b'a';
        rotated as char
    }).collect()
}

pub fn part2(input: &Vec<String>) -> i64 {
    let rooms = parse_input(input);    

    for room in rooms {
        if room.0 == "northpoleobjectstorage" {
            return room.1
        }
    }

    return 0;
}