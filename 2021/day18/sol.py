# Day 18: Snailfish
import sys
from copy import deepcopy

def readInputFile(filePath):
    with open(filePath, 'r+') as f:
        numbers = []
        line = f.readline().rstrip()
        while line:
            numbers.append(line)
            line = f.readline().rstrip()
        return numbers

def parseSnailNum(num):
    # read snail numbers from file input
    return num

# Traverse snailNum tree with given constraints to find nearest regular num on the right
def findNextRegNumRight(path):
    print(path) 

# Traverse snailNum tree via path and return the number with update value
def updateRegNum(snailNum, path, val):
    print(snailNum, path)
    return snailNum

# Reduce snail number by exploding and splitting rules
def reduceSnailNum(num, source, depth=1, path=[], nearestLeftRegNum=(None, []), nearestRightRegNum=(None, [])):
    # print('n:', n, 'type:', type(n), 'depth:', depth, 'first:', firstRegularNum)
    leftType = type(num[0])
    rightType = type(num[1])

    # Update nearest left regular number and path
    if leftType is int and rightType is list:
        nearestLeftRegNum = (num[0], path)
    # Update nearest right regular number and path
    elif leftType is list and rightType is int:
        nearestRightRegNum = (num[1], path)

    # Explode left pair
    if depth == 4 and leftType is list:
        if nearestLeftRegNum[0] != None:
            num[0] = num[0] + nearestLeftRegNum 
        else:
            num[0] = 0

    # Explode right pair (only if no left pair)
    elif depth == 4 and type(num[1]) == list:
        print('explode right pair')

    # Split pair
    elif type(num[0]) is int and num[0] >= 10:
        # Split left num
        print('n >= 10')
    elif type(num[1]) is int and num[1] >= 10:
        # Split right num
        print('n >= 10')
        
    return sum(literals)

def getSnailNumMagnitude(num):
    print('eval snail num', num)


# def explodeSnailNum(num):
    # newNum = deepcopy(num)
        # # print('n:', n, 'type:', type(n), 'depth:', depth, 'first:', firstRegularNum)
    # # Right side has reg num
    # newNum[1] = num[1][0] + num[1] if type(num[0]) is list and type(num[1]) is int else 0
    # # Left side has reg num
    # newNum[0] = num[0] + num[1][0] if type(num[0]) is list and type(num[1]) is int else 0

    # newSnailNum = [0, 0]

    # return newNum


def part1(input):
    print(reduceSnailNum(input, input))
    print('Part1:')

def part2(input):
    print('Part2:')

if __name__ == '__main__':
    if len(sys.argv) != 2:
        print('Usage: ')
    else:
        # input = readInputFile(sys.argv[1])
        input = [
            [1,2],
            [[1,2],3],
            [9,[8,7]],
            [[1,9],[8,5]],
            [[[[1,2],[3,4]],[[5,6],[7,8]]],9],
            [[[9,[3,8]],[[0,9],6]],[[[3,7],[4,9]],3]],
            [[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]],
        ]

        input = [[[[[9,8],1],2],3],4]
        input = [7,[6,[5,[4,[3,2]]]]]
        input = [[6,[5,[4,[3,2]]]],1]
        input = [[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]
        part1(input)
        # part2(input)
