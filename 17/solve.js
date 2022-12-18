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

const ROCKS = [
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
            return data[0];
        case 2: // parse part 2
            return data[0];
    }
}

class Rock {
    constructor(str, pos, grid) {
        this.str = str;
        this.height = str.split('\n').length;
        this.width = str.split('\n')[0].length;
        this.grid = grid;
        this.pos = pos;
        this.pixels = [];
        str.split('\n').forEach((row, y) => {
            row.split('').forEach((char, x) => {
                if (char === '#') {
                    this.pixels.push(new utils.Pos(x, y));
                }
            });
        });
        this.step_count = 1;
        this.is_landed = false;
    }

    #move(dir) {
        this.pos[dir]();

        //switch(dir) {
        //    case utils.DIR.north:
        //        this.top_left.north();
        //        this.top_right.north();
        //        this.bottom_left.north();
        //        this.bottom_right.north();
        //        break;
        //    case utils.DIR.south:
        //        this.top_left.south();
        //        this.top_right.south();
        //        this.bottom_left.south();
        //        this.bottom_right.south();
        //        break;
        //    case utils.DIR.east:
        //        this.top_left.east();
        //        this.top_right.east();
        //        this.bottom_left.east();
        //        this.bottom_right.east();
        //        break;
        //    case utils.DIR.west:
        //        this.top_left.west();
        //        this.top_right.west();
        //        this.bottom_left.west();
        //        this.bottom_right.west();
        //        break;
        //}
    }

    reset(pos) {
        this.step_count = 1;
        this.pos = pos;
        this.is_landed = false;
    }

    fall() {
        // TODO: check if all pixels are in range or have intersected with existing rocks
        if (this.pos.y - this.grid.floor >= this.height) {
            this.#move(utils.DIR.north);
        } else {
            this.is_landed = true;
        }
    }
    
    #jet_push(jet_char, grid) {
        // TODO: check if all pixels are in range or have intersected with existing rocks
        switch (jet_char) {
            case '>':
                this.#move(utils.DIR.east);
                break;
            case '<':
                this.#move(utils.DIR.west);
                break;
        }
    }

    #rest() {
        this.grid.floor += this.height;
    }

    draw() {
        this.pixels.forEach(pixel => {
            let pos = new utils.Pos(this.pos.x + pixel.x, this.pos.y - pixel.y)
            console.log('Draw pos:', pos)
            this.grid.drawChar(pos, '#')
        });
    }

    step() {
        console.log(`Step ${this.step_count}`);
        this.step_count++;
        // Rock comes to rest
        if (this.is_landed) {
            this.#rest()
            return false;
        }

        // Rock falls or is pushed by jet
        if (this.step_count % 2 === 0) {
            console.log('  Rock is falling!')
            this.fall();
        } else {
            console.log('  Rock being pushed by jet!')
            this.#jet_push();
        }

        return true;
    }

}

/* Solve */
function solve1(data) {
    data = parseData(data);
    let height = 28;
    let width = 7;
    let grid = new utils.Grid(width, height)
    grid.floor = grid.y_offset;

    let drop_pos = new utils.Pos(2, 0)


    rocks = ROCKS.map(str => new Rock(str, new utils.Pos(2, str.split('\n').length+3), grid))
    
    for (let i = 0; i < 6; i++) {
        console.log(rocks[i])
        while(rocks[i].step()) {
            console.log('>>> ROCK POS: ', rocks[i].pos)
        }
        rocks[i].draw(grid);
        rocks[i].reset(new utils.Pos(2, grid.floor + rocks[i].height + 3))

        grid.printBottomUp();
        console.log(' '.repeat(height.toString().length + 3) + '+' + '-'.repeat(width) + '+');
    }

    // NOTE: using a grid.floor WON'T WORK! You have to check whether one of the current rock's
    // pixels is sitting on top of another rock (or the floor) since the rocks won't alwys cover
    // the entire floor

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
