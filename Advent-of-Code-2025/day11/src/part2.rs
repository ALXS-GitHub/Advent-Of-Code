use petgraph::graph::{DiGraph, NodeIndex};
use std::collections::{HashMap, HashSet};

pub fn parse_input(input: &Vec<String>) -> (DiGraph<String, ()>, HashMap<String, NodeIndex>) {
    let mut graph = DiGraph::new();
    let mut node_indices: HashMap<String, NodeIndex> = HashMap::new();
    
    for line in input {
        let parts: Vec<&str> = line.split(": ").collect();
        let node_name = parts[0].to_string();
        
        if !node_indices.contains_key(&node_name) {
            let idx = graph.add_node(node_name.clone());
            node_indices.insert(node_name.clone(), idx);
        }
        
        if parts.len() > 1 {
            let targets: Vec<&str> = parts[1].split_whitespace().collect();
            for target in targets {
                let target_name = target.to_string();
                if !node_indices.contains_key(&target_name) {
                    let idx = graph.add_node(target_name.clone());
                    node_indices.insert(target_name, idx);
                }
            }
        }
    }
    
    for line in input {
        let parts: Vec<&str> = line.split(": ").collect();
        let source_name = parts[0];
        let source_idx = *node_indices.get(source_name).unwrap();
        
        if parts.len() > 1 {
            let targets: Vec<&str> = parts[1].split_whitespace().collect();
            for target in targets {
                let target_idx = *node_indices.get(target).unwrap();
                graph.add_edge(source_idx, target_idx, ());
            }
        }
    }
    
    return (graph, node_indices);
}

fn count_paths(
    graph: &DiGraph<String, ()>,
    current_node: NodeIndex,
    end_node: NodeIndex,
    visited: &mut HashMap<(NodeIndex, Vec<NodeIndex>), i64>,
    constraint_nodes: &Vec<&NodeIndex>,
    visited_constraints: &mut HashSet<NodeIndex>,
    visited_path: &mut HashSet<NodeIndex>,
) -> i64 {
    if current_node == end_node {
        return if visited_constraints.len() == constraint_nodes.len() { 1 } else { 0 };
    }
    
    let mut sorted_constraints: Vec<NodeIndex> = visited_constraints.iter().copied().collect();
    sorted_constraints.sort();
    let key = (current_node, sorted_constraints);
    
    if let Some(&count) = visited.get(&key) {
        return count;
    }

    visited_path.insert(current_node);
    
    let mut total_paths = 0;
    for neighbor in graph.neighbors(current_node) {
        if !visited_path.contains(&neighbor) {
            if constraint_nodes.contains(&&neighbor) {
                let already_visited = visited_constraints.contains(&neighbor);
                if !already_visited {
                    visited_constraints.insert(neighbor);
                }
            }
            
            total_paths += count_paths(graph, neighbor, end_node, visited, constraint_nodes, visited_constraints, visited_path);
            
            visited_constraints.remove(&neighbor);
        }
    }
    
    visited_path.remove(&current_node);
    visited.insert(key, total_paths);
    return total_paths;
}


pub fn part2(input: &Vec<String>) -> i64 {
    let (graph, node_indices) = parse_input(input);
    let start_node = node_indices.get("svr").unwrap();
    let end_node = node_indices.get("out").unwrap();
    let mut constraint_nodes: Vec<&NodeIndex> = vec![];
    for node in vec!["dac", "fft"] {
        if let Some(idx) = node_indices.get(node) {
            constraint_nodes.push(idx);
        }
    }
    let mut visited: HashMap<(NodeIndex, Vec<NodeIndex>), i64> = HashMap::new();
    let mut visited_constraints: HashSet<NodeIndex> = HashSet::new();
    let mut visited_path: HashSet<NodeIndex> = HashSet::new();
    let total_paths = count_paths(&graph, *start_node, *end_node, &mut visited, &constraint_nodes, &mut visited_constraints, &mut visited_path);
    
    return total_paths;
}