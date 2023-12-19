use std::fmt;

use aoc_utils::read_file;

use petgraph::graph::{Node, NodeIndex, DiGraph};
use petgraph::Directed;
use petgraph::algo::astar;


#[derive(Debug, PartialEq, PartialOrd)]
enum DIR {
    NORTH,
    SOUTH,
    EAST,
    WEST,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
struct Pos {
    row: usize,
    col: usize,
}

impl Pos {
    fn new(row: usize, col: usize) -> Pos {
        Pos { row, col }
    }
}


struct CityMap {
    data: Vec<Vec<u32>>,
    width: usize,
    height: usize,
}

impl CityMap {
    fn new(data: Vec<Vec<u32>>) -> CityMap {
        let height = data.len();
        let width = data[0].len();

        CityMap { data, width, height }
    }
}

impl fmt::Display for CityMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for r in &self.data {
            for c in r {
                write!(f, "{c}").unwrap();
            }
            write!(f, "\n").unwrap();
        }

        write!(f, "rows: {}, cols: {}\n", self.width, self.height)
    }
}

fn build_graph(map: &CityMap) -> DiGraph<u32, u32> {
    let pos_to_id = |pos: Pos| -> u32 { (((pos.row) * (map.width)) + pos.col) as u32 };

    let mut edges: Vec<(u32, u32, u32)> = vec![];

    // Add corner edges
    let top_left_id     = pos_to_id(Pos::new(0, 0));
    let top_right_id    = pos_to_id(Pos::new(0, map.width-1));
    let bot_left_id     = pos_to_id(Pos::new(map.height-1, 0));
    let bot_right_id    = pos_to_id(Pos::new(map.height-1, map.width-1));
    edges.extend_from_slice(&[
        // Top left
        (top_left_id, pos_to_id(Pos::new(0, 1)), map.data[0][1]),
        (top_left_id, pos_to_id(Pos::new(1, 0)), map.data[1][0]),

        // Top right
        (top_right_id, pos_to_id(Pos::new(0, map.width-2)), map.data[0][map.width-2]),
        (top_right_id, pos_to_id(Pos::new(1, map.width-1)), map.data[1][map.width-1]),

        // Bottom left
        (bot_left_id, pos_to_id(Pos::new(map.height-2, 0)), map.data[map.height-2][0]),
        (bot_left_id, pos_to_id(Pos::new(map.height-1, 1)), map.data[map.height-1][1]),

        // Bottom right
        (bot_right_id, pos_to_id(Pos::new(map.height-1, map.width-2)), map.data[map.height-1][map.width-2]),
        (bot_right_id, pos_to_id(Pos::new(map.height-2, map.width-1)), map.data[map.height-2][map.width-1]),
    ]);


    // Add remaining graph `edges` for top, bottom, left, right sides of map excluding corners
    // Add first row edges
    for c in 1..map.width-1 {
        let src_node_id = pos_to_id(Pos::new(0, c));
        edges.push(( src_node_id, pos_to_id(Pos::new(0, c-1)), map.data[0][c-1] ));
        edges.push(( src_node_id, pos_to_id(Pos::new(0, c+1)), map.data[0][c+1] ));
        edges.push(( src_node_id, pos_to_id(Pos::new(1, c)), map.data[1][c] ));
    }

    // Add last row edges
    for c in 1..map.width-1 {
        let src_node_id = pos_to_id(Pos::new(map.height-1, c));
        edges.push(( src_node_id, pos_to_id(Pos::new(map.height-1, c-1)), map.data[map.height-1][c-1] ));
        edges.push(( src_node_id, pos_to_id(Pos::new(map.height-1, c+1)), map.data[map.height-1][c+1] ));
        edges.push(( src_node_id, pos_to_id(Pos::new(map.height-2, c)), map.data[map.height-2][c] ));
    }

    // Add first column edges
    for r in 1..map.height-1 {
        let src_node_id = pos_to_id(Pos::new(r, 0));
        edges.push(( src_node_id, pos_to_id(Pos::new(r-1, 0)), map.data[r-1][0] ));
        edges.push(( src_node_id, pos_to_id(Pos::new(r+1, 0)), map.data[r+1][0] ));
        edges.push(( src_node_id, pos_to_id(Pos::new(r, 1)), map.data[r][1] ));
    }

    // Add last column edges
    for r in 1..map.height-1 {
        let src_node_id = pos_to_id(Pos::new(r, map.width-1));
        edges.push(( src_node_id, pos_to_id(Pos::new(r-1, map.width-1)), map.data[r-1][map.width-1] ));
        edges.push(( src_node_id, pos_to_id(Pos::new(r+1, map.width-1)), map.data[r+1][map.width-1] ));
        edges.push(( src_node_id, pos_to_id(Pos::new(r, map.width-2)), map.data[r][map.width-2] ));
    }

    // Add remaining internal graph edges
    for r in 1..map.height-1 {
        for c in 1..map.width-1 {
            let src_node_id = pos_to_id(Pos::new(r, c));
            edges.push(( src_node_id, pos_to_id(Pos::new(r-1, c)), map.data[r-1][c] ));
            edges.push(( src_node_id, pos_to_id(Pos::new(r, c+1)), map.data[r][c+1] ));
            edges.push(( src_node_id, pos_to_id(Pos::new(r+1, c)), map.data[r+1][c] ));
            edges.push(( src_node_id, pos_to_id(Pos::new(r, c-1)), map.data[r][c-1] ));
        }
    }

    DiGraph::<u32, u32>::from_edges(edges)
}


pub fn solve(filepath: &str) -> String {
    let data = read_file(filepath);

    let city_map = CityMap::new(data.split_whitespace()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>())
        .collect()
    );

    let g = build_graph(&city_map);

    let pos_to_id = |pos: Pos| -> u32 { (((pos.row) * (city_map.width)) + pos.col) as u32 };
    let id_to_pos = |pos: Pos| -> u32 { (((pos.row) * (city_map.width)) + pos.col) as u32 };

    let start_id = pos_to_id(Pos::new(0, 0) ) as usize;
    let finish_id = pos_to_id(Pos::new(city_map.height-1, city_map.width-1)) as usize;

    // dbg!(start_id, finish_id);
    let start_node = g.node_indices().find(|i| *i == NodeIndex::new(start_id as usize)).unwrap();
    let finish_node = g.node_indices().find(|i| *i == NodeIndex::new(finish_id as usize)).unwrap();
    let path = astar(&g, start_node, |f| f == finish_node, |_e| 1, |_| 0);

    println!("{}", city_map);
    dbg!(&path);

    for ni in path.unwrap().1 {

    }



    // dbg!(g);

    "p2".to_string()
}
