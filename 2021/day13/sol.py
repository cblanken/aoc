# Day 13: Transparent Origami
import sys

def foldUp(points, yAxis):
    points = set(points)
    pointsToFold = [p for p in points if p[1] - yAxis > 0]
    staticPoints= [p for p in points if p[1] - yAxis <= 0]
    newPoints = set(staticPoints)
    for p in pointsToFold:
        distFromAxis = p[1] - yAxis
        newPoints.add((p[0], yAxis - distFromAxis))

    return newPoints

def foldLeft(points, xAxis):
    points = set(points)
    pointsToFold = [p for p in points if p[0] - xAxis > 0]
    staticPoints= [p for p in points if p[0] - xAxis <= 0]
    newPoints = set(staticPoints)
    for p in pointsToFold:
        distFromAxis = p[0] - xAxis
        newPoints.add((xAxis - distFromAxis, p[1]))

    return newPoints

def readInputFile(filePath):
    with open(filePath, 'r+') as f:
        # Parse transparent paper dots
        points = []
        line = f.readline()
        while line != '\n':
            points.append( tuple([int(x) for x in line.rstrip().split(',')]) )
            line = f.readline()

        # Parse fold parameters
        folds = []
        line = f.readline()
        while line:
            parts = line.split('=')
            axis = parts[0][-1:]
            val = int(parts[1].rstrip())
            folds.append((axis, val))
            line = f.readline()

        return (points, folds)

def printPoints(points, rowMax, colMax):
    for x in range(rowMax):
        for y in range(colMax):
            if (y, x) in points:
                print('#', end='')
            else:
                print('.', end='')
        print('')


def part1(input):
    points = input[0]
    folds = input[1]

    cnt = 0
    for f in folds:
        if cnt > 0:
            break
        axis = f[1]
        if f[0] == 'x':
            points = foldLeft(points, axis)
        elif f[0] == 'y':
            points = foldUp(points, axis)
        cnt += 1

    print('Total visible point:', len(points))


def part2(input):
    points = input[0]
    folds = input[1]

    for f in folds:
        axis = f[1]
        if f[0] == 'x':
            points = foldLeft(points, axis)
        elif f[0] == 'y':
            points = foldUp(points, axis)

    print(points)
    printPoints(points, 10, 50)
    print('Total visible point:', len(points))

if __name__ == '__main__':
    if len(sys.argv) != 2:
        print('Usage: ')
    else:
        input = readInputFile(sys.argv[1])
        part1(input)
        part2(input)
