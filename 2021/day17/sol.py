# Day 17: Trick Shot
import sys
from functools import reduce

def readInputFile(filePath):
    with open(filePath, 'r+') as f:
        targetArea = f.readline()[13:].rstrip().split(',')

        xMin = int(targetArea[0].split('..')[0][2:])
        xMax = int(targetArea[0].split('..')[1])
        
        yMin = int(targetArea[1].split('..')[0][3:])
        yMax = int(targetArea[1].split('..')[1])

        return (xMin, xMax, yMin, yMax)

def launchProbe(xSpeed, ySpeed, steps, start = [0, 0], maxHeight = float("-inf")):
    # start x, start y, maxHeight, xSpeed, ySpeed
    launchData = [start[0], start[1], 0, 0, 0]
    for _ in range(steps):
        # Update launchData
        launchData[0] += xSpeed
        launchData[1] += ySpeed

        # Max Y height tracking       
        maxHeight = launchData[1] if launchData[1] > maxHeight else maxHeight

        # Drag
        if xSpeed > 0:
            xSpeed -= 1
        elif xSpeed < 0:
            xSpeed += 1

        # Gravity
        ySpeed -= 1

    launchData[2] = maxHeight
    launchData[3] = xSpeed
    launchData[4] = ySpeed
    return launchData

# Given previous probe launchData extend launch by 'steps'
def extendProbeLaunch(launchData, steps = 1):
    xPos = launchData[0]
    yPos = launchData[1]
    maxHeight = launchData[2]
    xSpeed = launchData[3]
    ySpeed = launchData[4]
    return launchProbe(xSpeed, ySpeed, steps, [xPos, yPos], maxHeight)

def probeInArea(launchData, target):
    x = launchData[0]
    y = launchData[1]
    return x >= target[0] and x <= target[1] and y >= target[2] and y <= target[3]

def findProbeVelocities(targetArea, maxY, maxSteps):
    minX = 1
    maxX = targetArea[1] + 1
    minY = targetArea[2] - 1
    velocities = []

    for x in range(minX, maxX):
        # if x % 5 == 0: print('Checking xSpeed:', x)
        for y in range(minY, maxY):
            # Check one-step variations
            launchData = launchProbe(x, y, 1)
            if probeInArea(launchData, targetArea):
                velocities.append((x, y, launchData))
            for s in range(maxSteps - 1):
                launchData = extendProbeLaunch(launchData, 1)
                if probeInArea(launchData, targetArea):
                    velocities.append((x, y, launchData))

    return velocities

def part1(input):
    # Find highest y
    velocities =  findProbeVelocities(input, 450, 450)
    maxY = max([v[2][2] for v in velocities])
    print('Part1:', maxY)

def part2(input):
    # Find the set of distinct initial velocities that land in the target zone
    velocities =  [x[:-1] for x in findProbeVelocities(input, 450, 450)]
    distinctVelocityCount = len(set(velocities))
    print('Part2:', distinctVelocityCount)

if __name__ == '__main__':
    if len(sys.argv) != 2:
        print('Usage: ')
    else:
        input = readInputFile(sys.argv[1])
        part1(input)
        part2(input)
