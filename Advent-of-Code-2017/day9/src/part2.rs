fn find_garbage(group: &Vec<char>, start_index: usize) -> (usize, i64) {
    let mut i = start_index;
    let mut counter = 0;

    while i < group.len() {
        if group[i] == '!' {
            i += 1;
            counter -= 1;
        }
        else if group[i] == '>' {
            return (i, counter as i64)
        }
        i += 1;
        counter += 1;
    }

    return (i, counter as i64)
}

fn find_group(group: &Vec<char>, start_index: usize, depth: i64) -> (i64, usize) {

    let mut i = start_index;
    let mut score = 0;

    while i < group.len() {

        
        if group[i] == '{' {
            let (s, e) = find_group(group, i + 1, depth + 1);
            score += s;
            i = e;
        }
        else if group[i] == '}' {
            return (score, i);
        }
        else if group[i] == '<' {
            let (e, s) = find_garbage(group, i + 1);
            score += s;
            i = e;
        }

        i += 1;
    }

    
    return (score, group.len());
}

pub fn part2(input: &Vec<String>) -> i64 {

    let group = input[0].clone().chars().collect::<Vec<_>>();

    let (res, _) = find_group(&group, 0, 0);

    return res;
}