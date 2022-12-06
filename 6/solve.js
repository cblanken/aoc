const events = require('events');
const fs = require('fs');
const readline = require('readline');

// Read data
async function parseFile(filename, part) {
    return new Promise( (resolve, reject) => {
        const file = readline.createInterface({
            input: fs.createReadStream(filename),
            output: process.stdout,
            terminal: false
        });

        // Parse input for part 1
        if (part === 1) {
            file.on('line', line => {
                data = line;
            })
        // Parse input for part 2
        } else if (part === 2) {
        }

        file.on('close', _ => {
            resolve(data);
        });
    });
}

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

function findStartOfPacket(data, window_size) {
    for (let i = 0; i < data.length - window_size; i++) {
        let slice = data.slice(i, i+window_size);
        if (containsUniqueChars(slice)) {
            return i + window_size;
        }
    }

    return data.length;
}

function solve1(data) {
    let packet_start = findStartOfPacket(data, 4);
    return packet_start
}

function solve2(data) {
    let packet_start = findStartOfPacket(data, 14);
    return packet_start
}

// Solve
parseFile(process.argv[2], 1).then(data => {
    // Part 1
    console.log(`Part 1: ${solve1(data)}`);

})
parseFile(process.argv[2], 1).then(data => {
    // Part 2
    console.log(`Part 2: ${solve2(data)}`);
})
