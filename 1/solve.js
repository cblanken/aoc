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
        let current_elf = [];
        file.on('line', line => {
            if (line.length === 0) {
                data.push([...current_elf]);
                current_elf.length = 0;
            } else {
                current_elf.push(line);
            }
        })

        file.on('close', _ => {
            resolve(data);
        });
    });
}

function sum(arr) {
    return arr.reduce((acc, curr) => Number(acc) + Number(curr));
}

// Solve
parseFile(process.argv[2]).then(data => {
    //console.log(data)
    total_cals_per_elf = data.map(cals => Number(sum(cals)));
    console.log(total_cals_per_elf)
    console.log(Math.max(...total_cals_per_elf));
})
