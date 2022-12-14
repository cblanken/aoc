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

function getPrefix(depth) {
    return ' '.repeat(depth);
}

class Packet {
    constructor(left, right) {
        this.left = left;
        this.right = right;
    }

    cmp(left = this.left, right = this.right, depth = 0) {
        utils.log(utils.LOGS.INFO, getPrefix(depth) + `${depth} - Compare`, left, 'vs', right)
        if (typeof left === 'number' && typeof right === 'number') {
            if (left === right) {
                return undefined;
            } else if (left < right) {
                utils.log(utils.LOGS.INFO, getPrefix(depth+1) + '- Left side is smaller, so inputs are in the right order')
                return true;
            } else {
                utils.log(utils.LOGS.INFO, getPrefix(depth+1) + '- Right side is smaller, so inputs are not in the right order')
                return false;
            }
        } else if (Array.isArray(left) && Array.isArray(right)) {
            for (let i = 0; i < left.length; i++) {
                if (right[i] === undefined) {
                    utils.log(utils.LOGS.INFO, getPrefix(depth+1) + '- Right side ran out of items, so inputs are not in the right order')
                    return false;
                } else {
                    let cmp = this.cmp(left[i], right[i], depth+1);
                    if (cmp !== undefined) { return cmp };
                }
            }
            utils.log(utils.LOGS.INFO, getPrefix(depth+1) + '- Left side ran out of items, so inputs are in the right order')
            return true;
        } else if (Array.isArray(right) && typeof left === 'number') { // left is integer
            utils.log(utils.LOGS.INFO, getPrefix(depth+1) + `- Mixed types; convert left to [${left}] and retry comparison`)
            return this.cmp([left], right, depth+1);
        } else if (Array.isArray(left) && typeof right === 'number') { // right is integer
            utils.log(utils.LOGS.INFO, getPrefix(depth+1) + `- Mixed types; convert right to [${right}] and retry comparison`)
            return this.cmp(left, [right], depth+1);
        }
    }
}
 
function cmpPackets(left, right) {
    if (left === right) { return 0; }
    if (typeof left === 'number' && typeof right === 'number') {
        if (left === right) {
            return 0;
        } else if (left < right) {
            return -1;
        } else {
            return 1;
        }
    } else if (Array.isArray(left) && Array.isArray(right)) {
        for (let i = 0; i < left.length; i++) {
            if (right[i] === undefined) {
                return 1;
            } else {
                let cmp = sortPackets(left[i], right[i], depth+1);
                if (cmp !== 0) { return cmp };
            }
        }
        return -1;
    } else if (Array.isArray(right) && typeof left === 'number') { // left is integer
        return sortPackets([left], right, depth+1);
    } else if (Array.isArray(left) && typeof right === 'number') { // right is integer
        return sortPackets(left, [right], depth+1);
    }

}

function parseData(data, part = 1) {
    let parsed_data = [];
    console.log(data)
    while(data.length > 0) {
        let pair = data.splice(0, 2);
        parsed_data.push(new Packet(eval(pair[0]), eval(pair[1])));
        data.shift();
    }
    return parsed_data;
}


/* Solve */
function solve1(data) {
    data = parseData(data);
    console.log(data)
    let sum = 0;
    let cmp;
    data.forEach((packet, p) => {
        utils.log(utils.LOGS.INFO, `\n== Pair ${p+1} ==`)
        let cmp = packet.cmp();
        sum += cmp ? p+1 : 0;
    });

    return sum;
}

function solve2(data) {
    console.log('before', data);
    let sorted = data.sort(cmpPackets);
    console.log('after', sorted);
}

// Part 1
readFile(process.argv[2], 1).then(data => {
    console.log(`Part 1: ${solve1(data)}`);
})

// Part 2
readFile(process.argv[2], 1).then(data => {
    //console.log(`Part 2: ${solve2(data)}`);
})
