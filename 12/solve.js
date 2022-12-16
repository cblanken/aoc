const events = require('events');
const fs = require('fs');
const readline = require('readline');
const utils = require('../utils.js');

async function readFile(filename, part) {
    return new Promise( (resolve, reject) => {
        const file = readline.createInterface({
            input: fs.createReadStream(filename),
            output: process.stdout,
            terminal: false
        });

        let data = [];
        let line_cnt = 0;
        file.on('line', line => {
            data.push([...line.trim().split('').map((letter, i) => {
                return new utils.DGraphNode(letter, ELEVATION[letter], [], new utils.Pos(i, line_cnt))
            })]);
            line_cnt++;
        })

        file.on('close', _ => {
            resolve(data);
        });
    });
}

function parseData(data, part = 1) {
    data.forEach((row, r) => {
        row.forEach((node, c) => {
            if (node.id === 'E') { return } // Don't add egress edges to End node
            let dir, adj;
            adjs = utils.getAdjacentPositions2D(r, c, data[0].length, data.length);
            adjs.forEach(adj => {
                let pos = adj[0];
                let dir = adj[1];
                let dest_node = data[pos.y][pos.x]
                if (dest_node.id === 'S') { return } // Don't add ingress edges to Start node
                let height_diff = dest_node.value - node.value;
                if (height_diff > 1) { return } // Don't add edge with higher elevation difference than 1
                node.addEdge(1, dest_node, dir)  // Goal is to reach E in as few steps (edges)
                                                        // as possible so each edge weight is equal
            });
        });
    });

    return data;
}

const ELEVATION = {
    a: 1,
    b: 2,
    c: 3,
    d: 4,
    e: 5,
    f: 6,
    g: 7,
    h: 8,
    i: 9,
    j: 10,
    k: 11,
    l: 12,
    m: 13,
    n: 14,
    o: 15,
    p: 16,
    q: 17,
    r: 18,
    s: 19,
    t: 20,
    u: 21,
    v: 22,
    w: 23,
    x: 24,
    y: 25,
    z: 26,
    S: 1,   // elevation = a
    E: 26,  // elevation = z
}


/* Solve */
function solve1(data) {
    let parsed_data = parseData(data, 1);
    let nodes = parsed_data.flat()
    let start_node = nodes.filter(n => n.id === 'S')[0];
    
    let graph = new utils.DGraph(nodes);
    graph.shortestPath(start_node);

    let board = []
    parsed_data.forEach(row => {
        let r = [];
        row.forEach(_ => {
            r.push('⋅');
        });

        board.push(r);
    });

    // Construct path
    let end_node = nodes.filter(n => n.id === 'E')[0];
    board[end_node.pos.y][end_node.pos.x] = 'E';

    let curr_node = end_node;
    while (curr_node.predecessor) {
        curr_node.predecessor.edges.forEach(edge => {
            if (edge.node2 === curr_node) {
                let pos = edge.node1.pos;
                board[pos.y][pos.x] = utils.DIR_SYM[edge.dir][0];
            }
        });

        curr_node = curr_node.predecessor;
    }

    // Print board
    board.forEach(row => {
        console.log(row.join(''))
    })

    return end_node.distance
}

function solve2(data) {
}

// Part 1
readFile(process.argv[2], 1).then(data => {
    console.log(`Part 1: ${solve1(data)}`);
})

// Part 2
readFile(process.argv[2], 1).then(data => {
    //console.log(`Part 2: ${solve2(data)}`);
})
