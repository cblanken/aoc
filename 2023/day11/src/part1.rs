use std::fmt;
use petgraph::graph::{NodeIndex, Graph};
use petgraph::algo::astar;
use combinations::Combinations;

use aoc_utils::{
    read_file,
};

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


struct GalaxyMap {
    gmap: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

impl GalaxyMap {
    fn new(gmap: Vec<Vec<char>>) -> GalaxyMap {
        let height = gmap.len();
        let width = gmap[0].len();

        GalaxyMap { gmap, width, height }
    }

    fn get_empty_rows(&self) -> Vec<usize> {
        let mut empty_rows: Vec<usize> = vec![];
        for (r, row) in self.gmap.clone().into_iter().enumerate() {
            if row.into_iter().all(|cell| cell == '.') {
                empty_rows.push(r as usize);
            }
        }

        empty_rows
    }

    fn get_empty_cols(&self) -> Vec<usize> {
        let m = self.gmap.clone();
        let mut empty_cols: Vec<usize> = vec![];
        for c in 0..m[0].len() {
            let mut is_empty = true;
            for r in 0..m.len() {
                if m[r][c] != '.' {
                    is_empty = false;
                    break;
                }
            }

            if is_empty {
                empty_cols.push(c);
            }
        }

        empty_cols
    }

    fn get_galaxies(&self) -> Vec<Pos> {
        let mut galaxies: Vec<Pos> = vec![];
        for r in 0..self.gmap.len() {
            for c in 0..self.gmap[0].len() {
                if self.gmap[r][c] == '#' {
                    galaxies.push(Pos { row: r, col: c })
                }
            }
        }

        galaxies
    }

    pub fn expand_map(&mut self) {
        let empty_cols = self.get_empty_cols();

        // Expand columns
        for col_i in empty_cols.into_iter().rev() {
            for row_i in 0..self.gmap.len() {
                self.gmap[row_i].insert(col_i + 1, '.');
            }
        }

        // Expand rows
        let empty_rows = self.get_empty_rows();
        let empty_row: Vec<char> = ".".repeat(self.gmap[0].len()).chars().collect();

        for row_i in empty_rows.into_iter().rev() {
            self.gmap.insert(row_i + 1, empty_row.clone());
        }

        self.height = self.gmap.len();
        self.width = self.gmap[0].len();

        assert!(self.height > 2);
        assert!(self.width > 2);
    }
}

impl fmt::Display for GalaxyMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for r in &self.gmap {
            println!("{}", r.iter().collect::<String>());
        }

        write!(f, "rows: {}, cols: {}\n", self.width, self.height)
    }
}

fn build_graph(map: &GalaxyMap) -> Graph<u32, u32> {
    let pos_to_id = |pos: Pos| -> u32 { (((pos.row) * (map.width)) + pos.col) as u32 };

    let mut edges: Vec<(u32, u32)> = vec![];

    // Add corner edges
    let top_left_id     = pos_to_id(Pos::new(0, 0));
    let top_right_id    = pos_to_id(Pos::new(0, map.width-1));
    let bot_left_id     = pos_to_id(Pos::new(map.height-1, 0));
    let bot_right_id    = pos_to_id(Pos::new(map.height-1, map.width-1));
    edges.extend_from_slice(&[
        // Top left
        (top_left_id, pos_to_id(Pos::new(0, 1))),
        (top_left_id, pos_to_id(Pos::new(1, 0))),

        // Top right
        (top_right_id, pos_to_id(Pos::new(0, map.width-2))),
        (top_right_id, pos_to_id(Pos::new(1, map.width-1))),

        // Bottom left
        (bot_left_id, pos_to_id(Pos::new(map.height-2, 0))),
        (bot_left_id, pos_to_id(Pos::new(map.height-1, 1))),

        // Bottom right
        (bot_right_id, pos_to_id(Pos::new(map.height-1, map.width-2))),
        (bot_right_id, pos_to_id(Pos::new(map.height-2, map.width-1))),
    ]);


    // Add remaining graph `edges` for top, bottom, left, right sides of map excluding corners

    // Add first row edges
    for c in 1..map.width-1 {
        let src_node_id = pos_to_id(Pos::new(0, c));
        edges.push(( src_node_id, pos_to_id(Pos::new(0, c-1)) ));
        edges.push(( src_node_id, pos_to_id(Pos::new(0, c+1)) ));
        edges.push(( src_node_id, pos_to_id(Pos::new(1, c)) ));
    }

    // Add last row edges
    for c in 1..map.width-1 {
        let src_node_id = pos_to_id(Pos::new(map.height-1, c));
        edges.push(( src_node_id, pos_to_id(Pos::new(map.height-1, c-1)) ));
        edges.push(( src_node_id, pos_to_id(Pos::new(map.height-1, c+1)) ));
        edges.push(( src_node_id, pos_to_id(Pos::new(map.height-2, c)) ));
    }

    // Add first column edges
    for r in 1..map.height-1 {
        let src_node_id = pos_to_id(Pos::new(r, 0));
        edges.push(( src_node_id, pos_to_id(Pos::new(r-1, 0)) ));
        edges.push(( src_node_id, pos_to_id(Pos::new(r+1, 0)) ));
        edges.push(( src_node_id, pos_to_id(Pos::new(r, 1)) ));
    }

    // Add last column edges
    for r in 1..map.height-1 {
        let src_node_id = pos_to_id(Pos::new(r, map.width-1));
        edges.push(( src_node_id, pos_to_id(Pos::new(r-1, map.width-1)) ));
        edges.push(( src_node_id, pos_to_id(Pos::new(r+1, map.width-1)) ));
        edges.push(( src_node_id, pos_to_id(Pos::new(r, map.width-2)) ));
    }

    // Add remaining internal graph edges
    for r in 1..map.height-1 {
        for c in 1..map.width-1 {
            let src_node_id = pos_to_id(Pos::new(r, c));
            edges.push(( src_node_id, pos_to_id(Pos::new(r-1, c)) ));
            edges.push(( src_node_id, pos_to_id(Pos::new(r, c+1)) ));
            edges.push(( src_node_id, pos_to_id(Pos::new(r+1, c)) ));
            edges.push(( src_node_id, pos_to_id(Pos::new(r, c-1)) ));
        }
    }

    Graph::<u32, u32>::from_edges(edges)
}


pub fn solve(filepath: &str) -> String {
    let data = read_file(filepath);

    let mut galaxy_map = GalaxyMap::new(data.split_whitespace().map(|l| l.chars().collect::<Vec<char>>()).collect());
    galaxy_map.expand_map();
    println!("{}", galaxy_map);

    let galaxy_locs = galaxy_map.get_galaxies();

    // Mark galaxies with IDs
    // for i in 0..galaxy_locs.len() {
    //     let loc = galaxy_locs[i];
    //     galaxy_map.gmap[loc.row][loc.col] = i.to_string().chars().collect::<Vec<char>>()[0];
    // }

    // Model galaxy_map into graph 
    let g = build_graph(&galaxy_map);


    // Run A* over graph for each galaxy pair
    let pos_to_id = |pos: Pos| -> u32 { (((pos.row) * (galaxy_map.width)) + pos.col) as u32 };
    let mut sum: u64 = 0;
    let combs: Vec<Vec<Pos>> = Combinations::new(galaxy_locs.clone(), 2).collect();
    for c in combs {
        let start_id = pos_to_id(c[0]) as usize;
        let finish_id = pos_to_id(c[1]) as usize;

        // dbg!(start_id, finish_id);
        let start_node = g.node_indices().find(|i| *i == NodeIndex::new(start_id as usize)).unwrap();
        let finish_node = g.node_indices().find(|i| *i == NodeIndex::new(finish_id as usize)).unwrap();

        let path = astar(&g, start_node, |f| f == finish_node, |_e| 1, |_| 0);
        println!("{:?} → {:?} [{start_id:?},{finish_id:?}], {}", c[0], c[1], path.clone().unwrap().1.len());
        sum += (path.unwrap().1.len() - 1) as u64;
    }

    //  TODO: use Manhattan distance calc from part 2 instead of expensive A*

    sum.to_string()
}
