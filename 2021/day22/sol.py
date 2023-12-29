# Day 22: Reactor Reboot
import sys
from copy import deepcopy

def readInputFile(filePath: str):
    with open(filePath, 'r+') as f:
        lines = []
        line = f.readline().rstrip()
        while line:
            line = line.split(' ')
            status = 1 if line[0] == 'on' else 0
            line = line[1].split(',')
            x = [int(x) for x in line[0][2:].split('..')]
            y = [int(y) for y in line[1][2:].split('..')]
            z = [int(z) for z in line[2][2:].split('..')]
            lines.append((status, x, y, z)) 
            line = f.readline().rstrip()

        return lines

def part1(input):
    print('Part1:', input)


def part2(input):
    print('Part1:', input)

if __name__ == '__main__':
    if len(sys.argv) != 2:
        print('Usage: ')
    else:
        input = readInputFile(sys.argv[1])
        part1(input)
        # part2(input)
