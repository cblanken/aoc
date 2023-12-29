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
        if (part === 1) {
            file.on('line', line => {
                line = line.trim();
                let half = Math.trunc(line.length / 2);
                data.push([line.slice(0, half), line.slice(half, line.length)]);
            })
        } else if (part === 2) {
            let group = [];
            file.on('line', line => {
                group.push(line)
                if (group.length === 3) {
                    data.push([...group]);
                    group.length = 0;
                }
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

const PRIORITIES = {
    'a': 1,     'b': 2,     'c': 3,     'd': 4,     'e': 5,
    'f': 6,     'g': 7,     'h': 8,     'i': 9,     'j': 10,
    'k': 11,    'l': 12,    'm': 13,    'n': 14,    'o': 15,
    'p': 16,    'q': 17,    'r': 18,    's': 19,    't': 20,
    'u': 21,    'v': 22,    'w': 23,    'x': 24,    'y': 25,
    'z': 26,    'A': 27,    'B': 28,    'C': 29,    'D': 30,
    'E': 31,    'F': 32,    'G': 33,    'H': 34,    'I': 35,
    'J': 36,    'K': 37,    'L': 38,    'M': 39,    'N': 40,
    'O': 41,    'P': 42,    'Q': 43,    'R': 44,    'S': 45,
    'T': 46,    'U': 47,    'V': 48,    'W': 49,    'X': 50,
    'Y': 51,    'Z': 52,
}

function findCommonItems(compartment1, compartment2) {
    compartment1 = compartment1.split('');
    compartment2 = compartment2.split('');
    return compartment1.filter(item => compartment2.includes(item));
}

function findCommonItems3(e1, e2, e3) {
    e1 = Array.from(new Set(e1.split('')));
    e2 = Array.from(new Set(e2.split('')));
    e3 = Array.from(new Set(e3.split('')));
    let e123 = e1.filter(item => e2.includes(item) && e3.includes(item));
    return e123;
}

function solve1(data) {
    let priority_sum = 0;
    data.forEach(rucksack => {
        priority_sum += PRIORITIES[findCommonItems(rucksack[0], rucksack[1])[0]];
    })

    return priority_sum;
}

function solve2(data) {
    let priority_sum = 0;
    data.forEach(group => {
         priority_sum += PRIORITIES[findCommonItems3(group[0], group[1], group[2])];
    });
    return priority_sum;
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
