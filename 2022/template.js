const events = require('events');
const fs = require('fs');
const readline = require('readline');
const utils = require('../utils.js');

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
                data.push(arr);
            })
        // Parse input for part 2
        } else if (part === 2) {
        }

        file.on('close', _ => {
            resolve(data);
        });
    });
}


/* Solve */
function solve1(data) {

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
