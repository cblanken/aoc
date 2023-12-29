const events = require('events');
const fs = require('fs');
const readline = require('readline');

// Read data
async function parseFile(filename) {
    return new Promise( (resolve, reject) => {
        const file = readline.createInterface({
            input: fs.createReadStream(filename),
            output: process.stdout,
            terminal: false
        });

        let data = [];
        file.on('line', line => {
            data.push(line.trim().split(' '));
        })

        file.on('close', _ => {
            resolve(data);
        });
    });
}

function sum(arr) {
    return arr.reduce((acc, curr) => Number(acc) + Number(curr));
}

const RPC_SCORE = {
    X: 1,   // rock
    Y: 2,   // paper
    Z: 3,   // scissors
}

const RPC_WINS = {
    X: 'C', // rock beats scissors
    Y: 'A', // paper beats rock
    Z: 'B', // scissors beats paper
}

const RPC_DRAW = {
    X: 'A', // rock
    Y: 'B', // paper
    Z: 'C', // scissors
}

const RPC_LOSE = {
    X: 'B', // rock loses to paper
    Y: 'C', // paper loses to scissors
    Z: 'A', // scissors loses to rock
}

const RPC_SELECT = { // for part 2
    X: RPC_LOSE,
    Y: RPC_DRAW,
    Z: RPC_WINS,
}

function getGameStatus(left, right) {
    if (RPC_WINS[left] === right) {
        return { stat: "win", score: 6 };
    } else if (RPC_DRAW[left] === right) {
        return { stat: "draw", score: 3 };
    } else {
        return { stat: "loss", score: 0 };
    }
}

function getFinalScore1(arr) {
    return arr.reduce((score, curr) => {
        let [colA, colB] = curr;
        score += Number(getGameStatus(colB, colA).score);
        score += Number(RPC_SCORE[colB]);
        return score;
    }, 0)
}

function getFinalScore2(arr) {
    return arr.reduce((score, curr) => {
        let [colA, colB] = curr;
        let selection_obj = RPC_SELECT[colB]
        let selection = Object.keys(selection_obj).find(key => selection_obj[key] === colA);
        score += Number(getGameStatus(selection, colA).score);
        score += Number(RPC_SCORE[selection]);
        return score;
    }, 0)
}

// Solve
parseFile(process.argv[2]).then(data => {
    // Part 1
    console.log(`Part 1: ${getFinalScore1(data)}`);

    // Part 2
    console.log(`Part 2: ${getFinalScore2(data)}`);
})
