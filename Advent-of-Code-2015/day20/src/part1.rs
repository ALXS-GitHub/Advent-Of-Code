fn prime_factors(mut n: i64) -> Vec<(i64, usize)> {
    let mut factors = Vec::new();

    let mut count = 0;
    while n % 2 == 0 {
        count += 1;
        n /= 2;
    }
    if count > 0 {
        factors.push((2, count));
    }

    let mut i = 3;
    while i * i <= n {
        let mut count = 0;
        while n % i == 0 {
            count += 1;
            n /= i;
        }
        if count > 0 {
            factors.push((i, count));
        }
        i += 2;
    }

    if n > 2 {
        factors.push((n, 1));
    }

    factors
}

fn get_divisors(n: i64) -> Vec<i64> {
    let factors = prime_factors(n);
    let mut divisors = vec![1];

    for &(prime, exponent) in &factors {
        let len = divisors.len();
        let mut current_power = 1;
        for _ in 0..exponent {
            current_power *= prime;
            for j in 0..len {
                let new_divisor = divisors[j] * current_power;
                divisors.push(new_divisor);
            }
        }
    }

    divisors.sort_unstable();
    divisors
}

fn sum_of_divisors(n: i64) -> i64 {
    let divisors = get_divisors(n);

    let sum_presents: i64 = divisors
        .into_iter()
        .map(|d| 10 * d)
        .sum();

    sum_presents
}

fn solve(n: i64) -> i64 {

    let mut current = 0;
    let mut current_sum = 0;

    while current_sum < n {
        current += 1;
        let sum_div = sum_of_divisors(current);
        current_sum = sum_div;
    }

    current

}

pub fn part1(input: &Vec<String>) -> i64 {

    let n = input[0].parse::<i64>().unwrap();

    return solve(n);
}