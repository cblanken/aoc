const events = require('events');
const fs = require('fs');
const readline = require('readline');
const utils = require('../utils.js');

async function readFile(filename, part) {
    return new Promise( (resolve, reject) => {
        const file = readline.createInterface({
            input: fs.createReadStream(filename),
            output: process.stdout,
            terminal: false
        });

        let data = [];
        file.on('line', line => {
            data.push(line);
        })

        file.on('close', _ => {
            resolve(data);
        });
    });
}


class Valve extends utils.DGraphNode {
    constructor(id, flow, tunnels, is_closed) {
        tunnels = tunnels === undefined ? [] : tunnels;
        super(id, flow, tunnels);
        this.is_closed = is_closed === undefined ? true : is_closed;
    }
}

function parseData(data, part = 1) {
    let parsed_data = {};
    switch (part) {
        case 1: // parse part 1
            // Create all nodes
            data.forEach(line => {
                let split = line.split(';');
                let id = split[0].split(' ')[1];
                let flow = split[0].split(' ')[4].split('=')[1];
                let edges_str = eval('["' + split[1].split(' ').slice(5).join('') + '"]');

                parsed_data[id] = new Valve(id, flow);
                parsed_data[id].edge_ids = edges_str
            });
            
            // Add node edges
            for (let key in parsed_data) {
                let valve = parsed_data[key];
                valve.edge_ids.forEach(id => {
                    valve.addEdge(1, parsed_data[id])
                })
            }

            return parsed_data;
        case 2: // parse part 2
            return data;
    }
}

/* Solve */
function solve1(data) {
    //console.log(data)
    data = parseData(data);
    for (let key in data) {
        let valve = data[key]
        console.log(valve.id, valve.edges)
    };
}

function solve2(data) {
}

// Part 1
readFile(process.argv[2], 1).then(data => {
    console.log(`Part 1: ${solve1(data)}`);
})

// Part 2
readFile(process.argv[2], 2).then(data => {
    //console.log(`Part 2: ${solve2(data)}`);
})
