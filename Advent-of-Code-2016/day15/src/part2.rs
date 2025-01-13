use regex::Regex;

#[derive(Debug, Clone)]
struct Disc {
    id: usize,
    positions: usize,
    pos: usize,
}

fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    let (mut q, mut a, mut b) = (1, a, b);
    let (mut x0, mut x1, mut y0, mut y1) = (1, 0, 0, 1);

    while b > 0 {
        (q, a, b) = ((a / b), b, a % b);
        (x0, x1) = (x1, x0 - q * x1);
        (y0, y1) = (y1, y0 - q * y1);
    }

    return (q, x0, y0)
}

fn least_pos_equivalent(res: i64, n: i64) -> i64 {
    (res % n + n) % n
}

fn crt(discs: &Vec<Disc>) -> i64 {
    let mut res = 0;
    let n: usize = discs.iter().map(|d| d.positions).collect::<Vec<usize>>().iter().product();

    for d in discs.iter() {
        // t + disc.id + d.pos ≡ 0 mod [d.positions]
        // t ≡ - (disc.id + d.pos) mod [d.positions]
        let a_i = - (d.id as i64 + d.pos as i64) % d.positions as i64;
        let a_i = least_pos_equivalent(a_i, d.positions as i64);
        let m = n / d.positions;
        let (_, _, si) = extended_gcd(d.positions as i64, m as i64);
        res += a_i * si * m as i64;
    }

    return least_pos_equivalent(res, n as i64);
}


fn parse_input(input: &Vec<String>) -> Vec<Disc> {

    let mut discs = Vec::new();
    let r = Regex::new(r"Disc #(\d+) has (\d+) positions; at time=0, it is at position (\d+).").unwrap();

    for line in input {
        let caps = r.captures(line).unwrap();
        discs.push(Disc {
            id: caps.get(1).unwrap().as_str().parse::<usize>().unwrap(),
            positions : caps.get(2).unwrap().as_str().parse::<usize>().unwrap(),
            pos: caps.get(3).unwrap().as_str().parse::<usize>().unwrap(),
        });
    }

    discs
}

pub fn part2(input: &Vec<String>) -> i64 {

    let mut discs = parse_input(input);
    // * note that all discs cycles are prime numbers -> we can use the CRT
    discs.push(Disc {
        id: discs.len() + 1,
        positions: 11,
        pos: 0,
    });

    let res = crt(&discs);

    return res;
}