/* AoC utilities */
function sum(arr) {
    return arr.reduce((acc, curr) => Number(acc) + Number(curr));
}

function mult(arr) {
    return arr.reduce((acc, curr) => Number(acc) * Number(curr));
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

function padStr(str, len) {
    if (len < str.length) {
        return str;
    } else {
        return ' '.repeat(len - str.length) + str;
    }
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
    constructor(x, y) {
        this.x = x;
        this.y = y;
    }

    north(dist = 1) {
        this.y -= dist;
    }

    south(dist = 1) {
        this.y += dist;
    }

    east(dist = 1) {
        this.x += dist;
    }

    west(dist = 1) {
        this.x -= dist;
    }
}

class Line {
    constructor(pos1, pos2) {
        this.pos1 = pos1;
        this.pos2 = pos2;
    }
}

class Grid {
    constructor(width, height) {
        this.width = width;
        this.height = height;
        this.grid = Array.from(Array(height), () => new Array(width))
        this.grid.forEach(row => {
            row.fill('.');
        });
    }

    inRange(pos) {
        return pos.x >= 0 && pos.x < this.width && pos.y >= 0 && pos.y < this.height
    }

    drawLine(line, char = "█") { // non-inclusive on upper bound
        if (line.pos1.x === line.pos2.x) { // vertical line
            let inc = line.pos1.y - line.pos2.y < 0 ? 1 : -1;
            for (let y = line.pos1.y; y !== line.pos2.y +inc; y += inc) {
                if (y < 0 || y > this.grid.length - 1) { break; }
                this.grid[y][line.pos1.x] = char;
            }
        } else if (line.pos1.y === line.pos2.y) { // horizontal line
            let inc = line.pos1.x - line.pos2.x < 0 ? 1 : -1;
            for (let x = line.pos1.x; x !== line.pos2.x + inc; x += inc) {
                if (x < 0 || x > this.grid[0].length - 1) { break; }
                this.grid[line.pos1.y][x] = char;
            }
        } else { // sloped line
            // TODO
        }
    }

    toString(x1=0, y1=0, x2=this.width, y2=this.height) {
        let str = "";
        for (let y = y1; y < y2; y++) {
            str += padStr(`${y}`, this.grid.length.toString().length + 2) + ` ${this.grid[y].slice(x1, x2).join('')}\n`;
        }
        return str;
    }

    print(x1=0, y1=0, x2=this.grid[0].length, y2=this.grid.length) {
        console.log(this.toString(x1, y1, x2, y2));
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

let LOG_LEVEL = LOGS.DISABLE;
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
    DGraph,
    DGraphNode,
    Grid,
    Line,
    LOGS,
    LOG_LEVEL,
    Pos,
    containsUniqueChars,
    getAdjacentPositions2D,
    isContainedBy,
    isOverlap,
    log,
    padStr,
    mult,
    sum,
}
