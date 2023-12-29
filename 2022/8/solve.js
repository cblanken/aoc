const events = require('events');
const fs = require('fs');
const readline = require('readline');

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
                row = line.split('').map(num => Number(num));
                data.push(row);
            })
        // Parse input for part 2
        } else if (part === 2) {
        }

        file.on('close', _ => {
            resolve(data);
        });
    });
}

/* Utilities */
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

const DIR = {
    north: 'north',
    south: 'south',
    east: 'east',
    west: 'west',
}

function getMaxTreeHeightInDir(dir, x, y, x_max, y_max, forest) {
    let max_height = 0;
    switch (dir) {
        case DIR.north:
            for (let row = y - 1; row >= 0; row--) {
                max_height = Math.max(max_height, forest[row][x]);
            }
            break;
        case DIR.south:
            for (let row = y + 1; row <= y_max; row++) {
                max_height = Math.max(max_height, forest[row][x]);
            }
            break;
        case DIR.east:
            for (let col = x + 1; col <= x_max; col++) {
                max_height = Math.max(max_height, forest[y][col]);
            }
            break;
        case DIR.west:
            for (let col = x - 1; col >= 0; col--) {
                max_height = Math.max(max_height, forest[y][col]);
            }
            break;
    }

    //console.log('-- MAX HEIGHT:', max_height, "DIR:", dir);

    return max_height;
}

function isTreeVisible(x, y, x_max, y_max, forest) {
    // Assume location is visible by default from each direction
    // Mark each direction false if larger tree found between
    // location and border
    let val = forest[y][x];
    let height = forest[y][x];

    let north = getMaxTreeHeightInDir(DIR.north, x, y, x_max, y_max, forest) < val;
    let south = getMaxTreeHeightInDir(DIR.south, x, y, x_max, y_max, forest) < val;
    let east = getMaxTreeHeightInDir(DIR.east, x, y, x_max, y_max, forest) < val;
    let west = getMaxTreeHeightInDir(DIR.west, x, y, x_max, y_max, forest) < val;


    //console.log("===========================");
    //console.log("X:", x, "Y:", y, "Val:", val);
    //console.log("north:", north);
    //console.log("south:", south);
    //console.log("east:", east);
    //console.log("west:", west);

    return north || south || east || west ? 1 : 0;
}

function getTreeVisibilityMatrix(forest) {
    max_height_matrix = [...forest];
    let x_max = forest[0].length - 1;
    let y_max = forest.length - 1;
    vis_matrix = Array.from(Array(y_max + 1), () => new Array(x_max + 1));

    // Set left and right edges to visible
    for (let y = 0; y <= y_max; y++) {
        vis_matrix[y][0] = 1;
        vis_matrix[y][x_max] = 1;
    }

    // Set top and bottom edges to visible
    for (let x = 0; x <= x_max; x++) {
        vis_matrix[0][x] = 1;
        vis_matrix[y_max][x] = 1;
    }

    let count = (x_max + 1) * 2 + (y_max - 1) * 2;

    // Populate remaining interior max height matrix
    for (let x = 1; x < x_max; x++) {
        for (let y = 1; y < y_max; y++) {
            vis_matrix[y][x] = isTreeVisible(x, y, x_max, y_max, forest);
            if (vis_matrix[y][x] === 1) {
                count++;
            }
        }
    }

    return [vis_matrix, count];
}

function getScenicScore(x, y, x_max, y_max, forest) {
    let scores = [];
    let val = forest[y][x];
    // Look north
    for (let row = y - 1; row >= 0; row--) {
        if (forest[row][x] >= val || row == 0) {
            scores.push(Math.abs(y - row))
            break;
        }
    }
    // Look south
    for (let row = y + 1; row <= y_max; row++) {
        if (forest[row][x] >= val || row == y_max) {
            scores.push(Math.abs(y - row))
            break;
        }
    }
    // Look east
    for (let col = x + 1; col <= x_max; col++) {
        if (forest[y][col] >= val || col == x_max) {
            scores.push(Math.abs(x - col))
            break;
        }
    }
    // Look west
    for (let col = x - 1; col >= 0; col--) {
        if (forest[y][col] >= val || col == 0) {
            scores.push(Math.abs(x - col))
            break;
        }
    }

    return scores.reduce((acc, score) => acc * score);
}

function getScenicScoreMatrix(forest) {
    max_height_matrix = [...forest];
    let x_max = forest[0].length - 1;
    let y_max = forest.length - 1;
    scenic_matrix = Array.from(Array(y_max + 1), () => new Array(x_max + 1));
    max_scenic_score = 0;

    // Set left and right edges to visible
    for (let y = 0; y <= y_max; y++) {
        scenic_matrix[y][0] = 0;
        scenic_matrix[y][x_max] = 0;
    }

    // Set top and bottom edges to visible
    for (let x = 0; x <= x_max; x++) {
        scenic_matrix[0][x] = 0;
        scenic_matrix[y_max][x] = 0;
    }

    // Populate remaining interior max height matrix
    for (let x = 1; x < x_max; x++) {
        for (let y = 1; y < y_max; y++) {
            scenic_matrix[y][x] = getScenicScore(x, y, x_max, y_max, forest);
            max_scenic_score = Math.max(max_scenic_score, scenic_matrix[y][x]);
        }
    }

    return [scenic_matrix, max_scenic_score];
}

/* Solve */
function solve1(data) {
    let mat = getTreeVisibilityMatrix(data);
    return mat[1]
}

function solve2(data) {
    let mat = getScenicScoreMatrix(data);
    return mat[1];
}

// Part 1
parseFile(process.argv[2], 1).then(data => {
    console.log(`Part 1: ${solve1(data)}`);
})

// Part 2
parseFile(process.argv[2], 1).then(data => {
    console.log(`Part 2: ${solve2(data)}`);
})
