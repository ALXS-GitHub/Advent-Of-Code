use serde_json::Value;

fn get_sum(value: &Value) -> i64 {
    match value {
        Value::Number(n) => n.as_i64().unwrap_or(0),
        Value::Array(arr) => arr.iter().map(get_sum).sum(),
        Value::Object(map) =>  {
            if map.values().any(|v| v == "red") {
                0
            } else {
                map.values().map(get_sum).sum()
            }
        },
        _ => 0,
    }
}



pub fn part2(input: &Vec<String>) -> i64 {
    let json: Value = serde_json::from_str(&format!("{}", input[0].clone())).unwrap();
    return get_sum(&json)
}