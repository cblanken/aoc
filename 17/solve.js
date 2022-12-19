const events = require('events');
const fs = require('fs');
const { setgroups } = require('process');
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

const FALLING_ROCKS = [
`@@@@`,
`.@.
@@@
.@.`,
`..@
..@
@@@`,
`@
@
@
@`,
`@@
@@`,
]

let ROCKS = [
`####`,
`.#.
###
.#.`,
`..#
..#
###`,
`#
#
#
#`,
`##
##`,
]

function parseData(data, part = 1) {
    let parsed_data = {};
    switch (part) {
        case 1: // parse part 1
            return data[0].split('');
        case 2: // parse part 2
            return data[0].split('');
    }
}

class Rock {
    #pos; #step_count; #is_landed;
    constructor(str, pos, grid) {
        this.str = str;
        this.height = str.split('\n').length;
        this.width = str.split('\n')[0].length;
        this.grid = grid;
        this.#pos = pos;
        this.pixels = [];
        str.split('\n').forEach((row, y) => {
            row.split('').forEach((char, x) => {
                if (char === '#') {
                    this.pixels.push(new utils.Pos(x, y));
                }
            });
        });
        this.#step_count = 0;
        this.#is_landed = false;
    }

    get pos() {
        return this.#pos;
    }

    set pos(value) {
        this.#pos = value;
    }

    get step_count() {
        return this.#step_count;
    }

    #move(dir) {
        this.pos[dir]();
    }

    reset() {
        this.#step_count = 0;
        this.#is_landed = false;
    }

    #fall() {
        // Check for overlap if rock falls 1 unit down
        let is_overlap = this.pixels.some(pixel => {
            let pos = new utils.Pos(this.pos.x + pixel.x, this.pos.y - pixel.y - 1);
            //console.log('grid val', pos.x, pos.y, this.grid.getValue(pos), pixel)
            return this.grid.getValue(pos) === '#';
        });
        if (is_overlap) {
            this.#is_landed = true;
            console.log('  Rock falls 1 unit causing it to land')
        } else {
            this.#move(utils.DIR.north);
            console.log('  Rock falls 1 unit!')
        }
    }

    #rest() {
        this.grid.floor = this.pos.y > this.grid.floor ? this.pos.y : this.grid.floor;
        this.draw();
        this.reset();
    }
    
    #jet_push(jet_char) {
        let is_overlap;
        switch (jet_char) {
            case '>':
                is_overlap = this.pixels.some(pixel => this.pos.x + pixel.x + 1 >= this.grid.x_offset + this.grid.width ||
                     this.grid.grid[this.pos.y - pixel.y][this.pos.x + pixel.x + 1] === '#')
                if (!is_overlap) {
                    this.#move(utils.DIR.east);
                    console.log('  Rock being pushed right by jet!')
                } else {
                    console.log('  Rock being pushed right by jet but nothing happens!')
                }
                break;
            case '<':
                is_overlap = this.pixels.some(pixel => this.pos.x + pixel.x - 1 < this.grid.x_offset ||
                     this.grid.grid[this.pos.y - pixel.y][this.pos.x + pixel.x - 1] === '#')
                if (!is_overlap) {
                    console.log('  Rock being pushed left by jet!')
                    this.#move(utils.DIR.west);
                } else {
                    console.log('  Rock being pushed left by jet but nothing happens!')
                }
                break;
        }
    }

    draw() {
        this.pixels.forEach(pixel => {
            let pos = new utils.Pos(this.pos.x + pixel.x, this.pos.y - pixel.y)
            console.log('Draw pos:', pos)
            this.grid.drawChar(pos, '#')
        });
    }

    step(jet_char) {
        console.log(`Step ${this.#step_count}, Pos:`, this.#pos);

        this.#jet_push(jet_char);
        this.#fall();

        this.#step_count += 1;

        // Rock comes to rest
        if (this.#is_landed) {
            this.#rest()
            return false;
        }
        return true;
    }
}

/* Solve */
function solve1(data) {
    data = parseData(data);
    let height = 6050;
    let width = 7;
    let grid = new utils.Grid(width, height)
    
    // Set floor
    grid.floor = 0;
    for(let i = 0; i < grid.grid[0].length; i++) {
        grid.drawChar(new utils.Pos(i, 0), '#')
    }

    let drop_pos = new utils.Pos(2, 0)

    let rocks = ROCKS.map(str => new Rock(str, new utils.Pos(2, str.split('\n').length+3), grid))

    // Iterate over all jet stream input
    rock_index = 0;
    let jet_str;
    let jet_index = 0;
    let rock_count = 0;
    let max_rock_count = 2021;
    while(rock_count < max_rock_count) {
        // Loop through rocks
        while (true) {
            // Update rock spawn position
            rocks[rock_index].pos = new utils.Pos(2, grid.floor + rocks[rock_index].height + 3);

            // Step rock until landed
            while (true) {
                //console.log("&&& CURR step count", rocks.length, rock_index, rocks[rock_index].step_count)
                jet_str = data[jet_index];
                jet_index = (jet_index + 1) % data.length;
                if (!rocks[rock_index].step(jet_str)) { break; }
            }
            
            // Update index to next rock
            rock_index = (rock_index + 1) % rocks.length

            rock_count++;
            if (rock_count > max_rock_count)  { break; }
        }
    }
    grid.printBottomUp();

    return grid.floor;
}

function solve2(data) {
}

// Part 1
readFile(process.argv[2], 1).then(data => {
    console.log(`Part 1: ${solve1(data)}`);
})

// Part 2
readFile(process.argv[2], 2).then(data => {
    //console.log(`Part 2: ${solve2(data)}`);
})
