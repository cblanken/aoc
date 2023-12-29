const events = require('events');
const fs = require('fs');
const readline = require('readline');

/* Read data */
async function parseFile(filename, part) {
    return new Promise( (resolve, reject) => {
        const file = readline.createInterface({
            input: fs.createReadStream(filename),
            output: process.stdout,
            terminal: false
        });


        let data = [];
        let cmd = "";
        let arg = "";
        // Parse input for part 1
        if (part === 1) {
            file.on('line', line => {
                let arr = line.trim().split(' ');
                data.push([arr[0], Number(arr[1])]);
            })
        // Parse input for part 2
        } else if (part === 2) {
        }

        file.on('close', _ => {
            resolve(data);
        });
    });
}

/* Utilities */
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

const DIR = {
    U: 'up',
    D: 'down',
    L: 'left',
    R: 'right',
}

class Knot {
    constructor(x, y, sym) {
        this.x = x;
        this.y = y;
        this.sym = sym;
    }

    mov(x, y) {
        this.x = x;
        this.y = y;
    }

    up(value) {
        this.y = this.y - value;
    }

    down(value) {
        this.y = this.y + value;
    }

    left(value) {
        this.x = this.x - value;
    }

    right(value) {
        this.x = this.x + value;
    }
}

class RopeBoard {
    constructor(size = 10, start) {
        this.size = size;
        this.board = Array.from(Array(size), () => new Array(size));
        this.board.forEach(row => {
            row.fill('.');
        });
        this.start = start === undefined ? new Knot(Math.trunc(size / 2), Math.trunc(size / 2)) : start;
        this.board[this.start.y][this.start.x] = 's';
        this.head = new Knot(this.start.x, this.start.y, 'H');
        this.knots = [
            new Knot(this.start.x, this.start.y, '1'),
            new Knot(this.start.x, this.start.y, '2'),
            new Knot(this.start.x, this.start.y, '3'),
            new Knot(this.start.x, this.start.y, '4'),
            new Knot(this.start.x, this.start.y, '5'),
            new Knot(this.start.x, this.start.y, '6'),
            new Knot(this.start.x, this.start.y, '7'),
            new Knot(this.start.x, this.start.y, '8'),
            new Knot(this.start.x, this.start.y, '9'),
        ];
        this.tail = this.knots[this.knots.length - 1];
    }

    updateTailChain() {
        let head = this.head;
        this.knots.forEach(knot => {
            this.setEmptySymbol(knot);
            this.tailFollow(head, knot);
            head = knot;
        });

        this.setVisitedSymbol(this.tail);
    }

    // Check current position of `head` and update `tail` accordingly
    tailFollow(head, tail) {
        if (head.x !== tail.x && head.y !== tail.y) { // diag
            // up-left
            if (head.x <= tail.x - 2 && head.y <= tail.y - 1 ||
                head.x <= tail.x - 1 && head.y <= tail.y - 2) {
                tail.up(1);
                tail.left(1);
            // up-right
            } else if (
                head.x >= tail.x + 2 && head.y <= tail.y - 1 ||
                head.x >= tail.x - 1 && head.y <= tail.y - 2) {
                tail.up(1);
                tail.right(1);
            // down-left
            } else if (
                head.x <= tail.x - 2 && head.y >= tail.y + 1 ||
                head.x <= tail.x - 1 && head.y >= tail.y + 2) {
                tail.down(1);
                tail.left(1);
            // down-right
            } else if (
                head.x >= tail.x + 2 && head.y >= tail.y + 1 ||
                head.x >= tail.x + 1 && head.y >= tail.y + 2) {
                tail.down(1);
                tail.right(1);
            }
        } else if (head.y <= tail.y - 2) { // up
            tail.up(1);
        } else if (head.y >= tail.y + 2) { // down
            tail.down(1);
        } else if (head.x <= tail.x - 2) { // left
            tail.left(1);
        } else if (head.x >= tail.x + 2) { // right
            tail.right(1);
        }

        this.setSymbol(tail, tail.sym);
    }

    setSymbol(pos, sym) {
        if (this.board[pos.y][pos.x] !== 's' && this.board[pos.y][pos.x] != '#') {
            this.board[pos.y][pos.x] = sym;
        }
    }

    setHeadSymbol(pos) {
        this.setSymbol(pos, 'H');
    }

    setTailSymbol(pos) {
        this.setSymbol(pos, 'T');
    }

    setVisitedSymbol(pos) {
        this.setSymbol(pos, '#');
    }

    setEmptySymbol(pos) {
        this.setSymbol(pos, '.');
    }

    moveHead(dir, val) {
        switch (DIR[dir]) {
            case DIR.U:
                for (let i = 0; i < val; i++) {
                    this.setEmptySymbol(this.head);
                    this.head.up(1);
                    this.setHeadSymbol(this.head);
                    this.updateTailChain();
                }
                break;
            case DIR.D:
                for (let i = 0; i < val; i++) {
                    this.setEmptySymbol(this.head);
                    this.head.down(1);
                    this.setHeadSymbol(this.head);
                    this.updateTailChain();
                }
                break;
            case DIR.L:
                for (let i = 0; i < val; i++) {
                    this.setEmptySymbol(this.head);
                    this.head.left(1);
                    this.setHeadSymbol(this.head);
                    this.updateTailChain();
                }
                break;
            case DIR.R:
                for (let i = 0; i < val; i++) {
                    this.setEmptySymbol(this.head);
                    this.head.right(1);
                    this.setHeadSymbol(this.head);
                    this.updateTailChain();
                }
                break;
        }
    }

    countVisited() {
        let count = 0;
        this.board.reduce((acc,row) => {
            count += row.reduce((acc, cell) => {
                if (cell === 's' || cell === '#' || cell === 'T') {
                    acc++;
                }

                return acc;
            }, 0)
        });

        return count;
    }
}

RopeBoard.prototype.toString = function() {
    let str = '\n';
    this.board.forEach(row => {
        let line = row.join('');
        str = str.concat(line, '\n');
    });
    str = str.concat('\n');

    return str;
}

/* Solve */
function solve1(data) {
    let rboard = new RopeBoard(5000);
    //console.log(rboard.toString());
    data.forEach(input => {
        rboard.moveHead(input[0], input[1]);
    });

    console.log(rboard.toString());
    return rboard.countVisited();
}

function solve2(data) {
}

// Part 1
parseFile(process.argv[2], 1).then(data => {
    console.log(`Part 1: ${solve1(data)}`);
})

// Part 2
parseFile(process.argv[2], 1).then(data => {
    console.log(`Part 2: ${solve2(data)}`);
})
