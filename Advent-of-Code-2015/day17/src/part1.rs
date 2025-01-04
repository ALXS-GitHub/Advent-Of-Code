fn parse_input(input: &Vec<String>) -> Vec<i64> {
    let containers = input.iter().map(|l| l.parse::<i64>().unwrap()).collect();
    containers
}

fn dp(quantity: i64, containers: Vec<i64>) -> i64 {
    let mut dp = vec![0; quantity as usize + 1];

    dp[0] = 1;

    for container in containers.iter() {
        for i in (*container as usize..=quantity as usize).rev() {
            dp[i] += dp[i - *container as usize];
        }
    }

    dp[dp.len() - 1]

}

pub fn part1(input: &Vec<String>) -> i64 {

    let containers = parse_input(input);
    let quantity = 150;

    let res = dp(quantity, containers);

    return res;
}