fn parse_input(input: &Vec<String>) -> Vec<i64> {
    let containers = input.iter().map(|l| l.parse::<i64>().unwrap()).collect();
    containers
}

#[derive(Debug, Clone)]
struct DPState {
    ways: i64,
    min_containers: usize,
}

fn dp_with_min_containers(quantity: i64, containers: Vec<i64>) -> i64 {
    let mut dp: Vec<Option<DPState>> = vec![None; (quantity + 1) as usize + 1];

    dp[0] = Some(DPState {
        ways: 1,
        min_containers: 0,
    });

    for container in containers {
        let container = container as usize;
        for i in (container..=quantity as usize).rev() {
            if let Some(ref prev_state) = dp[i - container] {
                match dp[i] {
                    Some(ref current_state) => {
                        if prev_state.min_containers + 1 < current_state.min_containers {
                            dp[i] = Some(DPState {
                                ways: prev_state.ways,
                                min_containers: prev_state.min_containers + 1,
                            });
                        } else if prev_state.min_containers + 1 == current_state.min_containers {
                            dp[i].as_mut().unwrap().ways += prev_state.ways;
                        }
                    }
                    None => {
                        dp[i] = Some(DPState {
                            ways: prev_state.ways,
                            min_containers: prev_state.min_containers + 1,
                        });
                    }
                }
            }
        }
    }

    let total_ways = dp[quantity as usize].as_ref().map_or(0, |state| state.ways);

    total_ways
}


pub fn part2(input: &Vec<String>) -> i64 {
    let containers = parse_input(input);
    let quantity = 150;

    let res = dp_with_min_containers(quantity, containers.clone());

    return res;
}