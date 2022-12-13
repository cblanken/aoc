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
    // Assume same width for all rows
    if (row > 0) {
        positions.push(new Pos(row-1, col)) // up
        //if (col > 0) { positions.push(new Pos(row-1, col-1)) } // up-left
        //if (col < width - 1) { positions.push(new Pos(row-1, col+1)) } // up-right
    }
    if (row < height - 1) {
        positions.push(new Pos(row+1, col)) // down
        //if (col > 0) { positions.push(new Pos(row+1, col-1)) } // down-left
        //if (col < width - 1) { positions.push(new Pos(row+1, col+1)) } // down-right
    }
    if (col > 0) {
        positions.push(new Pos(row, col-1)) // left
    }
    if (col < width - 1) {
        positions.push(new Pos(row, col+1)) // right
    }

    return positions;
}

class Pos {
    constructor(row, col) {
        this.row = row;
        this.col = col;
    }
}

class Node {
    constructor(val) {
        this.val = val;
    }
}

class Edge {
    constructor(node1, node2, weight = 0) {
        this.node1 = node1;
        this.node2 = node2;
        this.weight = weight;
    }
}

class DGraphNode {
    constructor(value, edges = []) {
        this.value = value;
        this.edges = edges;
    }

    addEdge(weight, dest_node) {
        this.edges.push(new Edge(this, dest_node, weight));
    }
}

class DGraph {
    constructor(root, nodes = []) {
        this.root = root;
        this.nodes = nodes;
    }

    addNodes(nodes) {
        nodes.forEach(node => {
            this.addNode(node);
        })
    }

    addNode(node) {
        this.nodes.push(node);
    }

    shortestPath(start, end) {
        // Implementation of Bellman Ford shortest path algorithm
    }
}

const LOG_LEVELS = {
    DISABLE: 0,
    CRITICAL: 1,
    WARNING: 2,
    INFO: 3,
}

function log(level, ...args) {
}

module.exports = {
    sum,
    isContainedBy,
    isOverlap,
    containsUniqueChars,
    getAdjacentPositions2D,
    DGraphNode,
    DGraph,
}
