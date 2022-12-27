const events = require('events');
const fs = require('fs');
const { setgroups } = require('process');
const readline = require('readline');
const math = require('mathjs');
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

function parseData(data, part = 1) {
    let parsed_data = {};
    switch (part) {
        case 1: // parse part 1
            return data.map(x => x.split(',').map(y => parseInt(y)));
        case 2: // parse part 2
            return data[0].split('');
    }
}

function getNeighbors(matrix, x, y, z) {
    let neighbors = [];
    let size = matrix.size();
    for(let i = math.max(0, x-1); i <= math.min(size[0]-1, x+1); i++) {
        for(let j = math.max(0, y-1); j <= math.min(size[1]-1, y+1); j++) {
            for(let k = math.max(0, z-1); k <= math.min(size[2]-1, z+1); k++) {
                if (i === x && j === y && k === z) { continue; }
                neighbors.push([i, j, k]);
            }
        }
    }
    return neighbors;
}

function getCubicNeighbors(matrix, x, y, z) {
    let neighbors = [];
    let size = matrix.size();
    for(let i = math.max(0, x-1); i <= math.min(size[0]-1, x+1); i++) {
        for(let j = math.max(0, y-1); j <= math.min(size[1]-1, y+1); j++) {
            for(let k = math.max(0, z-1); k <= math.min(size[2]-1, z+1); k++) {
                if (i === x && j === y && k === z) { continue; }
                if ((i === x && j === y) || (i === x && k === z) || (j === y && k === z)) {
                    neighbors.push([i, j, k]);
                }
            }
        }
    }
    return neighbors;
}

function addCube(sa, adjacentCubeCnt) {
    // Returns the surface area after adding new cube
    return sa ;
}

/* Solve */
function solve1(data) {
    data = parseData(data);
    let max_dim = data.reduce((acc, pos) => math.max(...pos, acc), 0);
    let cubes = math.zeros(max_dim, max_dim, max_dim);
    let surface_areas = math.zeros(max_dim, max_dim, max_dim);

    let sa_total = 0;
    data.forEach(pos => {
        
        let neighbors = getCubicNeighbors(cubes, pos[0], pos[1], pos[2]);
        let sa = 6;
        neighbors.forEach(n => {
            if (cubes.get([n[0], n[1], n[2]]) === 1) {
                sa--;
                sa_total--;
            }
        });

        //console.log("Adding", sa)
        sa_total += sa;
        cubes.set([pos[0], pos[1], pos[2]], 1); // mark each cube in matrix
    });

    return sa_total;
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
