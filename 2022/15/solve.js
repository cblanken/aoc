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

function parseData(data, part = 1) {
    let parsed_data = [];
    switch (part) {
        case 1: // parse part 1
            data.forEach(line => {
                let split = line.split(':');
                let sensor_str = split[0];
                let s_x = sensor_str.split(',')[0].split('=')[1];
                let s_y = sensor_str.split(',')[1].split('=')[1];
                let beacon_str = split[1];
                let b_x = beacon_str.split(',')[0].split('=')[1];
                let b_y = beacon_str.split(',')[1].split('=')[1];

                let sensor_pos = new utils.Pos(parseInt(s_x), parseInt(s_y));
                let beacon_pos = new utils.Pos(parseInt(b_x), parseInt(b_y));
                parsed_data.push(new Sensor(sensor_pos, beacon_pos));
            });
            return parsed_data;
        case 2: // parse part 2
            return data;
    }
}

class Sensor {
    constructor(pos, closest_beacon) {
        this.pos = pos;
        this.closest_beacon = closest_beacon;
        this.manhattan_distance = this.pos.getManhattanDistance(this.closest_beacon);
    }
}

/* Solve */
function solve1(data) {
    data = parseData(data);
    console.log(data)
    
    // Get max width and height form data
    let max_x = data.reduce((acc, sensor) => Math.max(Math.max(acc, sensor.pos.x), sensor.closest_beacon.x), 0) 
    let max_y = data.reduce((acc, sensor) => Math.max(Math.max(acc, sensor.pos.y), sensor.closest_beacon.y), 0) 
    let min_x = data.reduce((acc, sensor) => Math.min(Math.min(acc, sensor.pos.x), sensor.closest_beacon.x), 0) 
    let min_y = data.reduce((acc, sensor) => Math.min(Math.min(acc, sensor.pos.y), sensor.closest_beacon.y), 0) 
    console.log(`Max x: ${max_x}; Max y: ${max_y}; Min x: ${min_x}; Min y: ${min_y}`);

    // sample.txt
    //let grid = new utils.Grid(max_x - min_x + 10, 5, min_x - 5, 8) // sample.txt
    //let row = 10; // sample.txt
    
    // input.txt
    let row = 2000000;
    let grid = new utils.Grid(max_x - min_x, 5, min_x - 1, 1999998) // input.txt
    
    data.forEach(sensor => {
        grid.drawChar(sensor.pos, 'S');
        grid.drawChar(sensor.closest_beacon, 'B');
    });

    // Mark positions that can't contain a beacon in `row`
    grid.getRow(row).forEach((col, x) => {
        x += grid.x_offset;
        let pos = new utils.Pos(x, row);
        data.forEach(sensor => {
            if (sensor.manhattan_distance >= sensor.pos.getManhattanDistance(pos) && grid.getValue(pos) !== 'B') {
                grid.drawChar(pos, '#');
            }
        });
    });
    
    grid.print()

    // Count positions that can't contain a beacon
    let no_beacon_cnt = grid.getRow(row).reduce((acc, cell) => {
        if (cell === '#') { acc++; }
        return acc
    }, 0);

    return no_beacon_cnt;
}

function solve2(data) {
    data = parseData(data);
    
    // Get max width and height form data
    let max_x = data.reduce((acc, sensor) => Math.max(Math.max(acc, sensor.pos.x), sensor.closest_beacon.x), 0) 
    let max_y = data.reduce((acc, sensor) => Math.max(Math.max(acc, sensor.pos.y), sensor.closest_beacon.y), 0) 
    let min_x = data.reduce((acc, sensor) => Math.min(Math.min(acc, sensor.pos.x), sensor.closest_beacon.x), 0) 
    let min_y = data.reduce((acc, sensor) => Math.min(Math.min(acc, sensor.pos.y), sensor.closest_beacon.y), 0) 
    console.log(`Max x: ${max_x}; Max y: ${max_y}; Min x: ${min_x}; Min y: ${min_y}`);

    // sample.txt
    let grid = new utils.Grid(20, 20, 0, 0) // sample.txt
    
    // input.txt
    //let grid = new utils.Grid(4000000, 5, -5, -5) // input.txt
    
    data.forEach(sensor => {
        grid.drawChar(sensor.pos, 'S');
        grid.drawChar(sensor.closest_beacon, 'B');
    });

    // Mark positions that can't contain a beacon in `row`
    grid.grid.forEach((row, r) => {
        row.forEach((col, x) => {
            x += grid.x_offset;
            let pos = new utils.Pos(x, r);
            data.forEach(sensor => {
                if (sensor.manhattan_distance >= sensor.pos.getManhattanDistance(pos) && grid.getValue(pos) !== 'B') {
                    grid.drawChar(pos, '#');
                }
            });
        });
    });
    
    grid.print()

    // Find distress beacon
    let distress_beacon;
    grid.grid.every((row, r) => {
        return row.every((col, c) => {
            distress_beacon = new utils.Pos(c, r);
            if (grid.getValue(distress_beacon) === '.') {
                console.log('FOUND DISTRESS BEACON', distress_beacon)
                return false;
            }

            return true;
        });
    });

    let tuning_frequency = distress_beacon.x * 4000000 + distress_beacon.y;
    return tuning_frequency;
}

// Part 1
readFile(process.argv[2], 1).then(data => {
    //console.log(`Part 1: ${solve1(data)}`);
})

// Part 2
readFile(process.argv[2], 2).then(data => {
    console.log(`Part 2: ${solve2(data)}`);
})
