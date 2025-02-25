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

fn get_weight(tree: &HashMap<String, Node>, node_name: String) -> i64 {

    let node = tree.get(&node_name).unwrap();

    if node.child.is_empty() {
        return node.weight;
    } else {
        let mut sum = node.weight;
        for child in node.child.iter() {
            sum += get_weight(tree, child.clone());
        }
        return sum;
    }

}

fn find_unique(vec:&Vec<(i64, String)>) -> Option<(i64, String)> {
    vec.iter()
        .find(|&x| vec.iter().filter(|&y| y.0 == x.0).count() == 1)
        .cloned()
}

fn find_wrong(tree: &HashMap<String, Node>, root_name: String, depth: i64) -> i64 {
    let root = tree.get(&root_name).unwrap();

    let mut cw = vec![];

    for child in root.child.iter() {
        let w = get_weight(&tree, child.clone());
        cw.push((w, child.clone()));
    }

    let wrong_root = find_unique(&cw);

    if let Some(wr) = wrong_root {
        if depth == 0 {
            let common_weight = cw.iter()
            .find(|&&(w, _)| w != wr.0)
            .map(|&(w, _)| w)
            .unwrap();
        
            let difference = common_weight - wr.0;

            return find_wrong(tree, wr.1, depth + 1) + difference;
        }

        return find_wrong(tree, wr.1, depth + 1)

    } else {
        return root.weight;
    }

}

pub fn part2(input: &Vec<String>) -> i64 {

    let tree = parse_input(input);

    let root_name = find_root(&tree);

    let res = find_wrong(&tree, root_name, 0);

    return res;
}