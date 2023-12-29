const events = require('events');
const fs = require('fs');
const { setgroups } = require('process');
const readline = require('readline');
const math = require('mathjs');
const utils = require('../utils.js');
const { timeStamp } = require('console');

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

/* Matrix format for costs
    | x | -> ore
    | y | -> clay
    | z | -> obsidian
*/

// Blueprint cost model
class Blueprint {
    constructor(id, ore_cost, clay_cost, obsidian_cost) {
        this.id = id;

    }
}

function parseData(data, part = 1) {
    let parsed_data = {};
    switch (part) {
        case 1: // parse part 1
            return data.map(x => x.split(',').map(y => parseInt(y)));
        case 2: // parse part 2
            return data[0].split('');
    }
}

/* Solve */
function solve1(data) {
    data = parseData(data);
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
