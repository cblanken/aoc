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
    aim = 0
    for c in commands:
        if c[0] == 'forward':
            x_pos += c[1]
            y_pos += c[1] * aim
        elif c[0] == 'down':
            aim += c[1]
        elif c[0] == 'up':
            aim -= c[1]
    return (x_pos, y_pos)

if __name__ == '__main__':
    if len(sys.argv) != 2:
        print('Usage: ')
    else:
        commands = readInputFile(sys.argv[1])
        pos = trackSubmarinePosition(commands)
        print(pos)
        print(pos[0] * pos[1])

        
