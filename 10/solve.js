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
                let arr = line.trim().split(' ');
                let [name, ...args] = [...arr];
                let cmd = new Command(CMD[name], args);
                data.push(cmd);
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

const CMD = {
    noop: 'noop',
    addx: 'addx',
}

const CYCLES = {
    noop: 1,
    addx: 2,
}

class Command {
    constructor(name, args) {
        this.name = name;
        this.cycles = CYCLES[name];
        this.args = args === undefined ? [] : args;
    }
}

class CPU {
    constructor() {
        this.instructions = [];
        this.cycle = 1;
        this.registers = {
            X: 1,
        };
        this.signal_strengths = [];
        this.crt = Array.from(Array(6), () => new Array(40));
        this.crt.forEach(row => {
            row.fill('.');
        });
    }

    get_signal_strength() {
        return this.cycle * this.registers.X;
    }

    tick() {
        // Check each cycle for signal sample
        if ((this.cycle - 20) % 40 === 0) {
            console.log("x", this.registers);
            this.signal_strengths.push([this.cycle, this.get_signal_strength()]);
        }
        this.cycle++;
    }

    exec(cmd) {
        // Execute command
        switch (cmd.name) {
            case CMD.noop:
                this.tick();
                break;
            case CMD.addx:
                for (let c = 0; c < cmd.cycles; c++) {
                    this.tick();
                }
                this.registers.X += Number(cmd.args[0]);
                break;
        }
    }
}

/* Solve */
function solve1(data) {
    console.log(data)
    let cpu = new CPU();
    data.forEach(cmd => {
        cpu.exec(cmd);
    });

    return cpu.signal_strengths.reduce((acc, signal) => acc += signal[1], 0);
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
