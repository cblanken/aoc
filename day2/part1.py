import sys

def readInputFile(filePath):
    with open(filePath, 'r+') as f:
        lines = f.read().splitlines()
        commands = map(lambda line: line.split(' '), lines)
        commands = map(lambda cmd: (cmd[0], int(cmd[1])), commands) 
        return commands

def trackSubmarinePosition(commands):
    x_pos = 0   # horizontal position
    y_pos = 0   # vertical position
    for c in commands:
        if c[0] == 'forward':
            x_pos += c[1]
        elif c[0] == 'down':
            y_pos += c[1]
        elif c[0] == 'up':
            y_pos -= c[1]
    return (x_pos, y_pos)

if __name__ == '__main__':
    if len(sys.argv) != 2:
        print('Usage: ')
    else:
        commands = readInputFile(sys.argv[1])
        print(trackSubmarinePosition(commands))
        
