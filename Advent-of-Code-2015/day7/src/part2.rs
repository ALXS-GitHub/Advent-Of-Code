use std::collections::HashMap;

#[derive(Debug, Clone)]
enum Value {
    VAL(u16),
    VAR(String)
}

#[derive(Debug, Clone)]
enum Operation {
    AND(Value, Value),
    OR(Value, Value),
    RSHIFT(Value, Value),
    LSHIFT(Value, Value),
    NOT(Value),
    VALUE(Value)
}

fn get_val_or_var(s: &str) -> Value {
    if s.chars().all(|c| c.is_digit(10)) { 
        return Value::VAL(s.parse::<u16>().unwrap())
    } else {
        return Value::VAR(s.to_string())
    }
}

fn parse_input(input: &Vec<String>) -> HashMap<String, Operation> {

    let mut links_map = HashMap::new();

    for line in input {
        let parts = line.split(" -> ").collect::<Vec<&str>>();
        let out = parts[1];
        let op_parts = parts[0].split(" ").collect::<Vec<&str>>();

        let op: Operation = if op_parts.len() == 1 {
            let o = get_val_or_var(op_parts[0]);
            Operation::VALUE(o)
        } else if op_parts.len() == 2 {
            assert_eq!("NOT", op_parts[0]);
            let o = get_val_or_var(op_parts[1]);
            Operation::NOT(o)
        } else if op_parts.len() == 3 {
            let op1 = get_val_or_var(op_parts[0]);
            let op2 = get_val_or_var(op_parts[2]);
            let operator = op_parts[1];
            match operator {
                "LSHIFT" => Operation::LSHIFT(op1, op2),
                "RSHIFT" => Operation::RSHIFT(op1, op2),
                "AND" => Operation::AND(op1, op2),
                "OR" => Operation::OR(op1, op2),
                _ => panic!("Unknown Operator")
            }
        } else {
            panic!("Unknown len")
        };

        links_map.insert(out.to_string(), op);

    }

    return links_map

}

fn get_key_value(key: Value, operations_map: &HashMap<String, Operation>, values_map: &mut HashMap<String,u16>) -> u16 {

    if let Value::VAL(v) = key {
        return v;
    } else if let Value::VAR(k) = key {
        
        if let Some(val) = values_map.get(&k) {
            return *val;
        }
        
        let res = match operations_map.get(&k).unwrap() {
            Operation::VALUE(v) => match v {
                Value::VAL(_v) => _v,
                Value::VAR(_v) => &get_key_value(v.clone(), operations_map, values_map)
            },
            Operation::AND(o1, o2) =>&(get_key_value(o1.clone(), operations_map, values_map) & get_key_value(o2.clone(), operations_map, values_map)),
            Operation::OR(o1, o2) =>&(get_key_value(o1.clone(), operations_map, values_map) | get_key_value(o2.clone(), operations_map, values_map)),
            Operation::RSHIFT(o1, o2) =>&(get_key_value(o1.clone(), operations_map, values_map) >> get_key_value(o2.clone(), operations_map, values_map)),
            Operation::LSHIFT(o1, o2) =>&(get_key_value(o1.clone(), operations_map, values_map) << get_key_value(o2.clone(), operations_map, values_map)),
            Operation::NOT(v) => &(!get_key_value(v.clone(), operations_map, values_map))
        };
        
        values_map.insert(k, res.clone());

        return res.clone();
    }
    
    return 0;
}

pub fn part2(input: &Vec<String>) -> i64 {
    let operations_map = parse_input(input);
    
    let mut values_map = HashMap::new();
    
    let a_val = Value::VAR(String::from("a"));

    let res = get_key_value(a_val.clone(), &operations_map, &mut values_map);
    let mut values_map = HashMap::new();
    values_map.insert(String::from("b"), res);
    let res = get_key_value(a_val, &operations_map, &mut values_map);
    
    return res as i64;
}