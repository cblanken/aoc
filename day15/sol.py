# Day 15: Chiton
import sys

def readInputFile(filePath):
    with open(filePath, 'r+') as f:
        riskMap = []
        line = ""
        while line:
            line = f.readline().rstrip()
            riskMap.append(line)

        riskMap = [x.rstrip() for x in f.readlines()]
        return riskMap

def findSafestPath(riskMap, start, end):
    totalRisk = 0


def part1(input):
    print(input)
    print('part1')

def part2(input):
    print('part2')


if __name__ == '__main__':
    if len(sys.argv) != 2:
        print('Usage: ')
    else:
        input = readInputFile(sys.argv[1])
        part1(input)
        # part2(input)
