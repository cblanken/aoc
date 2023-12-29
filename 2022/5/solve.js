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

        let data = {
            crate_stacks: [],
            move_instructions: [],
        };
        // Parse input for part 1
        if (part === 1) {
            let is_stacks = true;
            file.on('line', line => {
                if (line.length === 0) {
                    is_stacks = false;
                    return;
                }

                if (is_stacks) {
                    data.crate_stacks.push(line);
                } else {
                    data.move_instructions.push(line);
                }
            })
        // Parse input for part 2
        } else if (part === 2) {
        }

        file.on('close', _ => {
            resolve(data);
        });
    });
}

STACK_INDEXES = [
    0, 1, 5, 9, 13, 17, 21, 25, 29, 33, 37,
]

ALPHA_UPPER = [
    'A','B','C','D','E','F','G','H','I','J','K','L','M',
    'N','O','P','Q','R','S','T','U','V','W','X','Y','Z'
]

class CrateStacks {
    constructor(stack_strings) {
        this.stack_count = Math.ceil(stack_strings[stack_strings.length-1].length / 4);
        this.stacks = {};
        for (let i = 1; i <= this.stack_count; i++) {
            this.stacks[i] = [];
        }


        stack_strings.splice(stack_strings.length - 1, 1);
        stack_strings.forEach(row => {
            for (let i = 1; i <= this.stack_count; i++) {
                if (ALPHA_UPPER.includes(row[STACK_INDEXES[i]])) {
                    this.stacks[i].unshift(row[STACK_INDEXES[i]]);
                }
            }
        });
    }

    getStackIndex(i) {
        if (i <= 0) {
            return 0;
        } else {
            return 2 + 4 * (i - 1);
        }
    }

    applyMovements(movements, maintain_stack_order = false) {
        if (!maintain_stack_order) {
            movements.forEach(m => {
                for (let i = 0; i < m.count; i++) {
                    let crate = this.stacks[m.from].pop();
                    this.stacks[m.to].push(crate);
                }
            });
        } else { // maintain stack order for part 2
            movements.forEach(m => {
                let from_stack = this.stacks[m.from];

                // Remove stack group from "from" stack
                let stack_group = from_stack.splice(from_stack.length - m.count, m.count);

                // Add stack group to "to" stack
                this.stacks[m.to] = this.stacks[m.to].concat(stack_group);

                //console.log("mov:", m)
                //console.log("stack group:", stack_group)
                //console.log(this.stacks);
            });
        }

    }
}

class Movements {
    constructor(movement_strings) {
        this.movements = movement_strings.map(movement => {
            let words = movement.split(' ');
            return {
                count: words[1],
                from: words[3],
                to: words[5],
            }
        });
    }
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
    crate_stacks = new CrateStacks(data.crate_stacks);
    movements = new Movements(data.move_instructions);

    console.log(crate_stacks)
    crate_stacks.applyMovements(movements.movements);

    let ans = [];
    for (const key of Object.keys(crate_stacks.stacks)) {
        let stack = crate_stacks.stacks[key];
        ans.push(stack[stack.length - 1]);
    }

    return ans.join('');
}

function solve2(data) {
    crate_stacks = new CrateStacks(data.crate_stacks);
    movements = new Movements(data.move_instructions);

    console.log(crate_stacks)
    crate_stacks.applyMovements(movements.movements, maintain_stack_order = true);

    let ans = [];
    for (const key of Object.keys(crate_stacks.stacks)) {
        let stack = crate_stacks.stacks[key];
        ans.push(stack[stack.length - 1]);
    }

    return ans.join('');
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
