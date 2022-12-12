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
        this.worry = BigInt(worry);
    }
}

Item.prototype.toString = function() {
    return this.worry;
}


class Monkey {
    constructor(id, items, op, test, worry_reduction) {
        this.id = id;
        this.items = items;
        this.op = op; // Get next worry value
        this.test = test; // Get next monkey to throw to
        this.inspect_count = 0;
        this.worry_reduction = worry_reduction === undefined ? true : worry_reduction;
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
    console.log(`    Item thrown to: Monkey ${next_monkey.id}`);
    this.pass_to(next_monkey);
}

Monkey.prototype.turn = function(monkeys) {
    console.log("ITEMS: ", this.items);
    while (this.items.length > 0) {
        let item = this.items[0];
        this.inspect(item);
        if (this.worry_reduction) {
            item.worry = item.worry / 3n; // no rounding necessary as BigInt
        }
        this.exec_test(item, monkeys);
    }
    this.items.forEach(item => {
    });
}

function calcMonkeyBusiness(monkeys) {
    let most_active = monkeys.sort((a, b) => b.inspect_count - a.inspect_count).slice(0, 2);
    return most_active[0].inspect_count * most_active[1].inspect_count;
}

function parseData(data, part) {
    part = part === undefined ? 1 : part;
    parsed_data = [];
    data.forEach(group => {
        let id = Number(group[0][group[0].length - 2]);
        let items = group[1].split(':')[1].split(',').map(num => {
            return new Item(BigInt(num.trim()));
        });

        let eval_arr = group[2].split('=')[1].trim().split(' ');
        eval_arr = eval_arr.map(expr_ele => {
            //console.log('expr', expr_ele);
            if (!isNaN(parseInt(expr_ele))) {
                //console.log("n BigInt", parseInt(expr_ele, 10));
                expr_ele += 'n'; // make BigInt literal
                //console.log("new BigInt", expr_ele);
            }

            return expr_ele;
        });

        let div_by = BigInt(group[3].split(' ').pop());
        let eval_str = eval_arr.join(' ');
        console.log('eval_str:', eval_str)
        const operation = (old) => {
            return eval(eval_str);
        }

        const test = (worry) => {
            // Return next monkey to throw to
            if (worry % div_by === 0n) {
                return Number(group[4].split(' ').pop());
            } else {
                return Number(group[5].split(' ').pop());
            }
        }

        if (part === 1) {
            parsed_data.push(new Monkey(id, items, operation, test))
        } else if (part === 2) {
            parsed_data.push(new Monkey(id, items, operation, test, worry_reduction=false))
        }
    });

    return parsed_data;
}


/* Solve */
function solve1(data) {
    let monkeys = parseData(data, 1)
    for (let round = 0; round < 20; round++) {
        monkeys.forEach(monkey => {
            console.log(`Monkey ${monkey.id}:`)
            monkey.turn(monkeys)
        });
    }
    monkeys.forEach(m => console.log('Monkey', m.id, 'inspected', m.inspect_count, 'items'));

    return calcMonkeyBusiness(monkeys);
}

function solve2(data) {
    let monkeys = parseData(data, 2)
    for (let round = 0; round < 1000; round++) {
        monkeys.forEach(monkey => {
            console.log(`Monkey ${monkey.id}:`)
            monkey.turn(monkeys)
        });
    }

    monkeys.forEach(m => console.log('Monkey', m.id, 'inspected', m.inspect_count, 'items'));

    return calcMonkeyBusiness(monkeys);
}

// Part 1
readFile(process.argv[2], 1).then(data => {
    console.log(`Part 1: ${solve1(data)}`);
})

// Part 2
readFile(process.argv[2], 1).then(data => {
    //console.log(`Part 2: ${solve2(data)}`);
})
