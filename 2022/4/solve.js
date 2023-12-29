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

        let data = [];
        // Parse input for part 1
        if (part === 1) {
            file.on('line', line => {
                line = line.trim();
                let pair = line.split(',')
                    .map(assignment => assignment.split('-')
                    .map(section => Number(section)));

                data.push(pair);
            })
        // Parse input for part 2
        } else if (part === 2) {
            file.on('line', line => {
                line = line.trim();
                let pair = line.split(',')
                    .map(assignment => assignment.split('-')
                    .map(section => Number(section)));

                data.push(pair);
            })
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

function solve1(data) {
    let count = 0;
    data.forEach(pair => {
        if (isContainedBy(pair[0], pair[1]) || isContainedBy(pair[1], pair[0])) {
            count++;
        }
    });

    return count;
}

function solve2(data) {
    let count = 0;
    data.forEach(pair => {
        if (isOverlap(pair[0], pair[1]) || isOverlap(pair[1], pair[0])) {
            count++;
        }
    });

    return count;
}

// Solve
parseFile(process.argv[2], 1).then(data => {
    // Part 1
    console.log(`Part 1: ${solve1(data)}`);

})
parseFile(process.argv[2], 2).then(data => {
    // Part 2
    console.log(`Part 2: ${solve2(data)}`);
})
