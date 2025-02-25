fn next(a: &mut i64, b: &mut i64) {
    *a *= 16807;
    *b *= 48271;
    *a %= 2147483647;
    *b %= 2147483647; 
}

fn judge(a: i64, b: i64) -> bool {

    let mut a = a;
    let mut b = b;
    
    for _ in 0..16 {
        let ra = a % 2;
        let rb = b % 2;
        if ra != rb {
            return false
        }
        a >>= 1;
        b >>= 1;
    }

    true
}

pub fn part1(input: &Vec<String>) -> i64 {

    let mut a = input[0].chars().collect::<String>()[input[0].len() - 3..input[0].len()].parse::<i64>().unwrap();
    let mut b = input[1].chars().collect::<String>()[input[1].len() - 3..input[1].len()].parse::<i64>().unwrap();

    let rounds = 40000000;
    let mut counter = 0;


    for _ in 0..rounds {
        next(&mut a, &mut b);
        if judge(a, b) {
            counter += 1;
        }
    }

    return counter;
}