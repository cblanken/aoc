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
                split = line.split(' ');
                if (split[0] === '$') {
                    cmd = split[1];
                    arg = split[2];
                    data.push({cmd: cmd, arg: arg, stdout:[]});
                } else {
                    data[data.length-1].stdout.push(line.split(' '));
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


class Shell {
    constructor(cwd) {
        // Directory object of the current directory
        this.cwd = cwd;
    }

    getRoot() {
        while (this.cwd.parent_dir !== null) {
            this.cwd = this.cwd.parent_dir;
        }

        return this.cwd;
    }
}

Shell.prototype.cd = function(dir) {
    // Assume Directory exists
    if (dir === '..') {
        this.cwd = this.cwd.parent_dir;
    } else {
        // Update cwd
        if (this.cwd?.children[dir]) {
            this.cwd = this.cwd.children[dir] 
        } else {
            this.cwd = new Directory(this.cwd, dir)
        }
    }
}

Shell.prototype.ls = function(contents) {
    // Use ls to populate files for Directories
    contents.forEach(file => {
        let name = file[1]
        if (file[0] === 'dir') {
            this.cwd.addDir(name);
        } else { // regular file
            this.cwd.addFile(name, file[0]);
        }
    });
}

const FILETYPE = {
    file: 'file',
    dir: 'dir',
}

class Directory {
    constructor(parent_dir, name) {
        this.parent_dir = parent_dir;
        this.name = name;
        this.children = {}; 
        this.size = 0;
    }

    addDir(name) {
        this.children[name] = new Directory(this, name)
    }

    addFile(name, size) {
        this.children[name] = new File(this, name, size);
    }

    getSize() {
        this.size = 0;
        for (const key of Object.keys(this.children)) {
            let child = this.children[key];
            if (child.constructor.name === 'Directory') {
                this.size += child.getSize();
            } else if (child.constructor.name === 'File') {
                this.size += Number(child.size);
            }
        }

        return this.size;
    }

    getAllDirs() {
        let dirs = [];
        for (const key of Object.keys(this.children)) {
            let child = this.children[key];
            if (child.constructor.name === 'Directory') {
                dirs.push(child);
                dirs = dirs.concat(child.getAllDirs());
            }
        }

        return dirs;
    }
}

class File {
    constructor(parent_dir, name, size) {
        this.parent_dir = parent_dir;
        this.name = name;
        this.size = Number(size);
    }
}


/* Solve */
function solve1(data) {
    let root = new Directory(null, '/');
    let shell = new Shell(root);
    data.forEach(cmd => {
        if (cmd.cmd === 'cd') {
            shell.cd(cmd.arg);
        } else if (cmd.cmd === 'ls') {
            shell.ls(cmd.stdout);
        }
    });

    root.getSize();
    let dirs = root.getAllDirs()
    dirs = dirs.filter(dir => dir.size <= 100000);
    return dirs.reduce((acc, dir) => { return acc + dir.size }, 0);
}

function solve2(data) {
    let root = new Directory(null, '/');
    let shell = new Shell(root);
    data.forEach(cmd => {
        if (cmd.cmd === 'cd') {
            shell.cd(cmd.arg);
        } else if (cmd.cmd === 'ls') {
            shell.ls(cmd.stdout);
        }
    });

    const MAX_FILE_SYSTEM_SIZE = 70000000;
    const TARGET_FREE_SPACE = 30000000;
    const TARGET_FILE_SYSTEM_SIZE = MAX_FILE_SYSTEM_SIZE - TARGET_FREE_SPACE;
    let root_size = root.getSize();
    let free_space = MAX_FILE_SYSTEM_SIZE - root_size;

    let dirs = root.getAllDirs()
    dirs = dirs.filter(dir => root_size - dir.size <= TARGET_FILE_SYSTEM_SIZE);

    return dirs.reduce((acc, dir) => { return dir.size < acc ? dir.size : acc }, Infinity);
}

// Part 1
parseFile(process.argv[2], 1).then(data => {
    console.log(`Part 1: ${solve1(data)}`);

})

// Part 2
parseFile(process.argv[2], 1).then(data => {
    console.log(`Part 2: ${solve2(data)}`);
})
