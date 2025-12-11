use petgraph::graph::{DiGraph, NodeIndex};
use std::collections::HashMap;

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
    visited: &mut HashMap<NodeIndex, i64>,
) -> i64 {
    if current_node == end_node {
        return 1;
    }
    if let Some(&count) = visited.get(&current_node) {
        return count;
    }
    
    let mut total_paths = 0;
    for neighbor in graph.neighbors(current_node) {
        total_paths += count_paths(graph, neighbor, end_node, visited);
    }
    
    visited.insert(current_node, total_paths);
    return total_paths;
}

pub fn part1(input: &Vec<String>) -> i64 {
    let (graph, node_indices) = parse_input(input);
    let start_node = node_indices.get("you").unwrap();
    let end_node = node_indices.get("out").unwrap();
    let mut visited: HashMap<NodeIndex, i64> = HashMap::new();
    let total_paths = count_paths(&graph, *start_node, *end_node, &mut visited);
    return total_paths;
}