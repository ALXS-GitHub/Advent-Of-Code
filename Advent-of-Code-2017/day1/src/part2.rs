fn get_sum(numbers: Vec<i32>) -> i32 {
    let n = numbers.len();
    let mut sum = 0;
    for i in 0..numbers.len() {
        if numbers[i] == numbers[(i+n/2) % numbers.len()] {
            sum += numbers[i];
        }
    }
    sum
}

pub fn part2(input: &Vec<String>) -> i64 {

    let numbers = input[0].chars().collect::<Vec<_>>().iter().map(|n| n.to_digit(10).unwrap() as i32).collect::<Vec<i32>>();

    return get_sum(numbers) as i64;
}