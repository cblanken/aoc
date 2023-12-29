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
        file.on('line', line => {
            data.push(line);
        })

        file.on('close', _ => {
            resolve(data);
        });
    });
}

class Sand {
    constructor(pos) {
        this.pos = pos;
    }

    isPosEmpty(grid) {
        return grid.inRange(this.pos) && grid.grid[this.pos.y][this.pos.x] === '.'
    }

    step(grid) {
        // Returns true when sand has come to resting point
        grid.grid[this.pos.y][this.pos.x] = '.' // Empty current pos

        // Move down
        this.pos.south();
        if (this.isPosEmpty(grid)) {
            grid.grid[this.pos.y][this.pos.x] = 'o'
            return false;
        }

        // Move down and left
        this.pos.west();
        if (this.isPosEmpty(grid)) {
            grid.grid[this.pos.y][this.pos.x] = 'o'
            return false;
        }

        // Move down and right
        this.pos.east(2);
        if (this.isPosEmpty(grid)) {
            grid.grid[this.pos.y][this.pos.x] = 'o'
            return false;
        }

        // Rest
        this.pos.north();
        this.pos.west();
        if (this.isPosEmpty(grid)) {
            grid.grid[this.pos.y][this.pos.x] = 'o'
            return true;
        }
    }
}

function parseData(data, part = 1) {
    let parsed_data = [];
    switch (part) {
        case 1: // parse part 1
            data.forEach(line => {
                parsed_data.push(line.trim().split('->').map(pos_str => {
                    let pos = pos_str.split(',')
                    return new utils.Pos(parseInt(pos[0]), parseInt(pos[1]));
                }));
            });
            return parsed_data;
        case 2: // parse part 2
            return data;
    }
}

/* Solve */
function solve1(data) {
    data = parseData(data);
    let grid = new utils.Grid(600, 173);
    //let grid = new utils.Grid(600, 11);
    let lines = [];
    data.forEach(line => {
        for (let i = 1; i < line.length; i++) {
            lines.push(new utils.Line(line[i-1], line[i]));
        }
    });

    // Draw rocks
    lines.forEach(line => {
        grid.drawLine(line);
    });

    
    //grid.print(x1=490, x2=505);
    let sand = new Sand(new utils.Pos(500, 0)); // drop new sand
    let sand_cnt = 0;
    while (true) {
        let rested = sand.step(grid);
        if (rested) {
            sand = new Sand(new utils.Pos(500, 0)); // drop new sand
            sand_cnt++;
        }
        if (sand.pos.y > grid.height - 2) {
            console.log("FALLING INTO VOID! AAAAAH!");
            break;
        }
    }
    grid.print(450, 0);
    return sand_cnt;
}

function solve2(data) {
    data = parseData(data);
    //let grid = new utils.Grid(600, 11); // sample1.txt
    let grid = new utils.Grid(1000, 173); // input.txt
    let lines = [];
    data.forEach(line => {
        for (let i = 1; i < line.length; i++) {
            lines.push(new utils.Line(line[i-1], line[i]));
        }
    });

    // Draw rocks
    lines.forEach(line => {
        grid.drawLine(line);
    });
    
    let sand = new Sand(new utils.Pos(500, 0)); // drop new sand
    let cnt = 0;
    let sand_cnt = 0;
    while (true) {
        let rested = sand.step(grid);
        if (rested) {
            sand_cnt++;
            if (sand.pos.y === 0) {
                console.log("SOURCE IS BLOCKED");
                break;
            }
            sand = new Sand(new utils.Pos(500, 0)); // drop new sand
        }
    }
    grid.print(0, 0);
    return sand_cnt;
}

// Part 1
readFile(process.argv[2], 1).then(data => {
    //console.log(`Part 1: ${solve1(data)}`);
})

// Part 2
readFile(process.argv[2], 2).then(data => {
    console.log(`Part 2: ${solve2(data)}`);
})
