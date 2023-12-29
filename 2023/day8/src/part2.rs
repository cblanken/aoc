use std::collections::HashMap;
use petgraph::graph::{NodeIndex, Graph};

use aoc_utils::{
    read_file,
};

pub fn solve(filepath: &str) -> String {
    let data = read_file(filepath);

    let lines: Vec<&str> = data.lines().collect();
    let directions = lines[0];

    // Add graph nodes
    let mut graph = Graph::<String, _>::new();
    for line in &lines[2..] {
        let split: Vec<&str> = line.split('=').collect();
        let source = split[0].trim();

        graph.add_node(source.to_string());

    }

    let mut edges: Vec<(String, String)> = vec![];
    for line in &lines[2..] {
        let split: Vec<&str> = line.split('=').collect();
        let source = split[0].trim();
        let destinations: Vec<String> = split[1].split(',').map(|d| d.trim().replace("(", "").replace(")", "")).collect();

        println!("{:?} - {:?}", source, destinations);
        for d in destinations {
            edges.push((source.to_string(), d));
        }
    }

    // Add graph edges
    let mut lr_index = 0;
    for e in edges {
        let src_node = graph.node_indices().find(|i| graph[*i] == e.0).unwrap();
        let dest_node = graph.node_indices().find(|i| graph[*i] == e.1).unwrap();
        graph.add_edge(src_node, dest_node, "LR".chars().collect::<Vec<char>>()[lr_index]);
        lr_index = (lr_index + 1) % 2;
    }

    let mut active_nodes: Vec<NodeIndex> = graph.node_indices()
        .filter(|i| graph[*i].chars().rev().collect::<Vec<char>>()[0] == 'A')
        .collect();

    let active_node_cnt = active_nodes.len();
    let mut dir_index = 0;
    let mut step_cnt: u128 = 0;
    loop {
        for i in 0..active_node_cnt {
            let mut edges = graph.neighbors(active_nodes[i]).detach();
            while let Some(edge) = edges.next_edge(&graph) {
                let edge_endpoints = graph.edge_endpoints(edge).unwrap();
                let weight = graph.edge_weight(edge).unwrap();
                let dir = &directions.chars().collect::<Vec<char>>()[dir_index];
                // dbg!(graph.node_weight(active_nodes[i]), weight, dir, edge_endpoints);
                if weight == dir {
                    let target_node_index = edge_endpoints.1;
                    active_nodes[i] = target_node_index;
                    break;
                }
            }
        }

        step_cnt += 1;
        if step_cnt % 50000 == 0 {
            println!("Step count: {step_cnt}");
        }
        // if active_nodes.clone().into_iter().all(|i| graph[i].chars().rev().collect::<Vec<char>>()[0] == 'Z') {
        if active_nodes.iter().all(|i| graph[*i].chars().nth(2).unwrap() == 'Z') {
            println!("ALL NODES IN FINAL POSITIONS");
            dbg!(&active_nodes);
            break;
        }
        dir_index = (dir_index + 1) % directions.len();
    }

    // for n in node_map_dup.values() {
    //     println!("{:?} - {:?}", n.name, n.nodes)
    // }

    step_cnt.to_string()
}
