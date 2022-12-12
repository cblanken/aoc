const events = require('events');
const fs = require('fs');
const readline = require('readline');
const utils = require('../utils.js');

/* Read data */
async function readFile(filename, part) {
    return new Promise( (resolve, reject) => {
        const file = readline.createInterface({
            input: fs.createReadStream(filename),
            output: process.stdout,
            terminal: false
        });

        let data = [];
        monkey_strings = [];
        file.on('line', line => {
            if (monkey_strings.length === 6) {
                data.push([...monkey_strings]);
                monkey_strings = [];
            } else if (line !== '') {
                monkey_strings.push(line.trim());
            } 

        })

        file.on('close', _ => {
            resolve(data);
        });
    });
}


class Item {
    constructor(worry) {
        this.worry = Number(worry);
    }
}

Item.prototype.toString = function() {
    return this.worry;
}


class Monkey {
    constructor(id, items, op, test) {
        this.id = id;
        this.items = items;
        this.op = op; // Get next worry value
        this.test = test; // Get next monkey to throw to
        this.inspect_count = 0;
    }
}

Monkey.prototype.push = function(item) {
    this.items.push(item);
}

Monkey.prototype.pass_to = function(monkey) {
    monkey.push(this.items.shift());
}

Monkey.prototype.inspect = function(item) {
    console.log(`  Inspecting item:`, item);
    let new_worry = this.op(item.worry);
    item.worry = new_worry;
    console.log(`    Item updated worry = ${item.worry}`);
    this.inspect_count++;
}

Monkey.prototype.exec_test = function(item, monkeys) {
    console.log(`    Testing item:`, item);
    let next_monkey_id = this.test(item.worry)
    let next_monkey = monkeys.find(m => m.id === next_monkey_id);
    console.log(`    Item thrown to: ${next_monkey.id}`);
    this.pass_to(next_monkey);
}

Monkey.prototype.turn = function(monkeys) {
    console.log("ITEMS: ", this.items);
    while (this.items.length > 0) {
        let item = this.items[0];
        this.inspect(item);
        item.worry = Math.trunc(item.worry / 3);
        this.exec_test(item, monkeys);
    }
    this.items.forEach(item => {
    });
}

function calcMonkeyBusiness(monkeys) {
    let most_active = monkeys.sort((a, b) => b.inspect_count - a.inspect_count).slice(0, 2);
    return most_active[0].inspect_count * most_active[1].inspect_count;
}

function parseData(data) {
    parsed_data = [];
    data.forEach(group => {
        let id = Number(group[0][group[0].length - 2]);
        let items = group[1].split(':')[1].split(',').map(num => {
            return new Item(Number(num.trim()));
        });
        const operation = (old) => {
            return eval(group[2].split('=')[1].trim());
        }
        const test = (worry) => {
            // Return next monkey to throw to
            let div_by = group[3].split(' ').pop();
            if (worry % div_by === 0) {
                return Number(group[4].split(' ').pop());
            } else {
                return Number(group[5].split(' ').pop());
            }
        }

        parsed_data.push(new Monkey(id, items, operation, test))
    });

    return parsed_data;
}


/* Solve */
function solve1(data) {
    let monkeys = parseData(data)
    for (let round = 0; round < 20; round++) {
        monkeys.forEach(monkey => {
            console.log(`Monkey ${monkey.id}:`)
            monkey.turn(monkeys)
        });
    }

    //monkeys.forEach(m => console.log(m.id, m.items, m.inspect_count));
    
    return calcMonkeyBusiness(monkeys);
}

function solve2(data) {

}

// Part 1
readFile(process.argv[2], 1).then(data => {
    console.log(`Part 1: ${solve1(data)}`);
})

// Part 2
readFile(process.argv[2], 1).then(data => {
    console.log(`Part 2: ${solve2(data)}`);
})
