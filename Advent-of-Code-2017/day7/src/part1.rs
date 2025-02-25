use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Node {
    parent: Option<String>,
    weight: i64,
    child: Vec<String>
}

fn parse_input(input: &Vec<String>) -> HashMap<String, Node> {
    let mut tree = HashMap::new();
    let r = Regex::new(r"(\w+) \((\d+)\)( -> [\w,\s]+)?").unwrap();

    for line in input {
        let caps = r.captures(line).unwrap();
        let name = caps.get(1).unwrap().as_str().to_string();
        let weight = caps.get(2).unwrap().as_str().parse::<i64>().unwrap();
        let mut child = vec![];
        if let Some(c) = caps.get(3) {
            let c = c.as_str();
            let c = c.chars().collect::<Vec<_>>()[4..].iter().collect::<String>();
            let names = c.split(", ").collect::<Vec<_>>().iter().map(|x| x.to_string()).collect::<Vec<_>>();
            child.extend(names);
        }

        let current_node = tree.entry(name.clone()).or_insert(Node {
            parent: None,
            weight: 0,
            child: vec![]
        });
        current_node.weight = weight;
        current_node.child = child.clone();

        for c in child {
            let c_node = tree.entry(c).or_insert(Node {
                parent: None,
                weight: 0,
                child: vec![]
            });
            c_node.parent = Some(name.clone())
        }

    }

    tree
}

fn find_root(tree: &HashMap<String, Node>) -> String {
    for (k, v) in tree.iter() {
        if v.parent == None {
            return k.clone()
        }
    }
    return "".to_string();
}

pub fn part1(input: &Vec<String>) -> String {

    let tree = parse_input(input);

    let res = find_root(&tree);

    return res;
}