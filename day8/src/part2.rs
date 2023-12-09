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

    dbg!(&graph);


    let mut steps = 0;
    let mut dir_index = 0;
    // let mut curr_node = graph.node_indices().find(|i| graph[*i] == "AAA").unwrap();

    let mut active_nodes: Vec<NodeIndex> = graph.node_indices()
        .filter(|i| graph[*i].chars().rev().collect::<Vec<char>>()[0] == 'A')
        .collect();

    let node_cnt = active_nodes.clone().len();
    let mut finished_nodes: Vec<NodeIndex> = vec![];

    while finished_nodes.len() < node_cnt {
        for curr_node in active_nodes {
            let mut edges = graph.neighbors(curr_node).detach();
            while let Some(edge) = edges.next_edge(&graph) {
                let weight = graph.edge_weight(edge).unwrap();
                let dir = &directions.chars().collect::<Vec<char>>()[dir_index];
                dbg!(graph.node_weight(curr_node), weight, dir);
                if weight == dir {
                    curr_node = graph.edge_endpoints(edge).unwrap().1;
                    break;
                }
            }
        }
    }

    // while graph.node_weight(curr_node).unwrap() != "ZZZ" {
    //     let mut edges = graph.neighbors(curr_node).detach();
    //     while let Some(edge) = edges.next_edge(&graph) {
    //         let weight = graph.edge_weight(edge).unwrap();
    //         let dir = &directions.chars().collect::<Vec<char>>()[dir_index];
    //         dbg!(graph.node_weight(curr_node), weight, dir);
    //         if weight == dir {
    //             curr_node = graph.edge_endpoints(edge).unwrap().1;
    //             break;
    //         }
    //     }
    //     dir_index = (dir_index + 1) % directions.len();
    //     steps += 1;
    // }



    // for n in node_map_dup.values() {
    //     println!("{:?} - {:?}", n.name, n.nodes)
    // }

    steps.to_string()
}
