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

class Pos {
    constructor(x, y) {
        this.x = x;
        this.y = y;
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
        this.start = start === undefined ? new Pos(Math.trunc(size / 2), Math.trunc(size / 2)) : start;
        this.board[this.start.y][this.start.x] = 's';
        this.head = new Pos(this.start.x, this.start.y);
        this.tail = new Pos(this.start.x, this.start.y);
    }

    // Check current position of this.head and update this.tail accordingly
    tailFollow() {
        if (this.head.x !== this.tail.x && this.head.y !== this.tail.y) { // diag
            // up-left
            if (this.head.x <= this.tail.x - 2 && this.head.y <= this.tail.y - 1 ||
                this.head.x <= this.tail.x - 1 && this.head.y <= this.tail.y - 2) {
                this.tail.up(1);
                this.tail.left(1);
            // up-right
            } else if (
                this.head.x >= this.tail.x + 2 && this.head.y <= this.tail.y - 1 ||
                this.head.x >= this.tail.x - 1 && this.head.y <= this.tail.y - 2) {
                this.tail.up(1);
                this.tail.right(1);
            // down-left
            } else if (
                this.head.x <= this.tail.x - 2 && this.head.y >= this.tail.y + 1 ||
                this.head.x <= this.tail.x - 1 && this.head.y >= this.tail.y + 2) {
                this.tail.down(1);
                this.tail.left(1);
            // down-right
            } else if (
                this.head.x >= this.tail.x + 2 && this.head.y >= this.tail.y + 1 ||
                this.head.x >= this.tail.x + 1 && this.head.y >= this.tail.y + 2) {
                this.tail.down(1);
                this.tail.right(1);
            }
        } else if (this.head.y <= this.tail.y - 2) { // up
            this.tail.up(1);
        } else if (this.head.y >= this.tail.y + 2) { // down
            this.tail.down(1);
        } else if (this.head.x <= this.tail.x - 2) { // left
            this.tail.left(1);
        } else if (this.head.x >= this.tail.x + 2) { // right
            this.tail.right(1);
        }
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
                    this.setVisitedSymbol(this.tail);
                    this.tailFollow();
                    this.setTailSymbol(this.tail);
                }
                break;
            case DIR.D:
                for (let i = 0; i < val; i++) {
                    this.setEmptySymbol(this.head);
                    this.head.down(1);
                    this.setHeadSymbol(this.head);
                    this.setVisitedSymbol(this.tail);
                    this.tailFollow();
                    this.setTailSymbol(this.tail);
                }
                break;
            case DIR.L:
                for (let i = 0; i < val; i++) {
                    this.setEmptySymbol(this.head);
                    this.head.left(1);
                    this.setHeadSymbol(this.head);
                    this.setVisitedSymbol(this.tail);
                    this.tailFollow();
                    this.setTailSymbol(this.tail);
                }
                break;
            case DIR.R:
                for (let i = 0; i < val; i++) {
                    this.setEmptySymbol(this.head);
                    this.head.right(1);
                    this.setHeadSymbol(this.head);
                    this.setVisitedSymbol(this.tail);
                    this.tailFollow();
                    this.setTailSymbol(this.tail);
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
