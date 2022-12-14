/* AoC utilities */
function sum(arr) {
    return arr.reduce((acc, curr) => Number(acc) + Number(curr));
}

function isContainedBy(a, b) {
    // Returns true if 'a' is contained by 'b'
    return b[0] <= a[0] && b[1] >= a[1];
}

function isOverlap(a, b) {
    // Return true if 'a' or 'b' contain any overlapping sections
    return a[0] >= b[0] && a[0] <= b[1] // a[0] contained in 'b'
        || a[1] >= b[0] && a[1] <= b[1] // a[1] contained in 'b'
}

function containsUniqueChars(str) {
    let set = new Set(str.split(''));
    return set.size === str.length;
}

function getAdjacentPositions2D(row, col, width, height) {
    let positions = [];
    let dirs = [];
    // Assume same width for all rows
    if (row > 0) {
        positions.push(new Pos(row-1, col)) // up
        dirs.push(DIR.north)
        //if (col > 0) { positions.push(new Pos(row-1, col-1)) } // up-left
        //if (col < width - 1) { positions.push(new Pos(row-1, col+1)) } // up-right
    }
    if (row < height - 1) {
        positions.push(new Pos(row+1, col)) // down
        dirs.push(DIR.south)
        //if (col > 0) { positions.push(new Pos(row+1, col-1)) } // down-left
        //if (col < width - 1) { positions.push(new Pos(row+1, col+1)) } // down-right
    }
    if (col > 0) {
        positions.push(new Pos(row, col-1)) // left
        dirs.push(DIR.west)
    }
    if (col < width - 1) {
        positions.push(new Pos(row, col+1)) // right
        dirs.push(DIR.east)
    }

    let adj_data = positions.map((pos, i) => {
        return [pos, dirs[i]];
    });

    return adj_data;
}

const DIR = {
    none: null,
    north: 'north',
    east: 'east',
    south: 'south',
    west: 'west',
}

class Pos {
    constructor(row, col) {
        this.row = row;
        this.col = col;
    }

    north(dist = 1) {
        this.row -= dist;
    }

    south(dist = 1) {
        this.row += dist;
    }

    east(dist = 1) {
        this.col += dist;
    }

    west(dist = 1) {
        this.col -= dist;
    }
}

class Node {
    constructor(val) {
        this.val = val;
    }
}

class Edge {
    constructor(node1, node2, weight = 1, dir = DIR.none) {
        this.node1 = node1;
        this.node2 = node2;
        this.weight = weight;
        this.dir = dir;
    }
}

class DGraphNode {
    constructor(id, value, edges = []) {
        this.id = id;
        this.value = value;
        this.edges = edges;
    }

    addEdge(weight, dest_node, dir) {
        this.edges.push(new Edge(this, dest_node, weight, dir));
    }
}

class DGraph {
    constructor(nodes = []) {
        this.nodes = nodes;
        this.edges = [];
        this.nodes.forEach(node => {
            node.edges.forEach(edge => {
                this.edges.push(edge);
            });
        });
    }

    addNodes(nodes) {
        nodes.forEach(node => {
            this.addNode(node);
        })
    }

    addNode(node) {
        this.nodes.push(node);
        node.edges.forEach(edge => {
            this.edges.push(edge);
        });
    }

    // Bellman Ford shortest path algorithm
    shortestPath(source) {
        // Initialize graph
        this.nodes.forEach(node => {
            node.distance = Infinity;
            node.predecessor = null;
        });

        source.distance = 0; // source => source distance is 0

        // Relax edges
        for (let _ = 0; _ < this.nodes.length - 1; _++) {
            this.edges.forEach(edge => {
                let u = edge.node1;
                let v = edge.node2;
                if (u.distance + edge.weight < v.distance) {
                    v.distance = u.distance + edge.weight;
                    v.predecessor = u;
                }
            });
        }

        // Check for negative-weight cycles
        this.edges.forEach(edge => {
            let u = edge.node1;
            let v = edge.node2;
            if (u.distance + edge.weight < v.distance) {
                // Find a negative-weight cycle
                let negative_loop = [v, u];
                for (let _ = 0; _ < this.nodes.length - 1; _++) {
                    u = negative_loop[0];
                    this.edges.forEach(edge => {
                        if (u.distance + edge.weight < v.distance) {
                            negative_loop = [v].concat(negative_loop);
                        }
                    });
                    throw "Graph contains a negative-weight cycle"
                }
            }
        });
    }
}

const LOGS = {
    INFO: 1,
    WARNING: 2,
    CRITICAL: 3,
    DISABLE: 99,
}

let LOG_LEVEL = LOGS.INFO;
function log(level, ...args) {
    if (level >= LOG_LEVEL) {
        let str = "";
        args.forEach(arg => {
            if (typeof arg !== 'string') {
                str += JSON.stringify(arg) + ' ';
            } else {
                str += arg + ' ';
            }
        });
        console.log(str);
    }
}

module.exports = {
    sum,
    isContainedBy,
    isOverlap,
    containsUniqueChars,
    getAdjacentPositions2D,
    DGraphNode,
    DGraph,
    LOG_LEVEL,
    LOGS,
    log,
}
