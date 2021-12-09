# Day 9: Smoke Basin
import sys
import math
import functools

class HeightMap:
    def __init__(self, grid: list):
        self.grid = grid
        self.rowCount = len(self.grid)
        self.colCount = len(self.grid[0])

    def lookLeft(self, row, col):
        return self.grid[row][col-1] if col > 0 else math.inf

    def lookRight(self, row, col):
        return self.grid[row][col+1] if col < self.colCount - 1 else math.inf

    def lookUp(self, row, col):
        return self.grid[row-1][col] if row > 0 else math.inf

    def lookDown(self, row, col):
        return self.grid[row+1][col] if row < self.rowCount - 1 else math.inf

    def getLowestAdjacentValue(self, row, col):
        lowestAdjacentValue = math.inf
        lowestAdjacentValue = min(self.lookLeft(row, col), lowestAdjacentValue)
        lowestAdjacentValue = min(self.lookRight(row, col), lowestAdjacentValue)
        lowestAdjacentValue = min(self.lookUp(row, col), lowestAdjacentValue)
        lowestAdjacentValue = min(self.lookDown(row, col), lowestAdjacentValue)
        # print('row:', row, 'col:', col, 'lowestAdjacent:', lowestAdjacentValue)
        return lowestAdjacentValue
    
    def getHighestAdjacentValue(self, row, col):
        highestAdjacentValue = -math.inf
        highestAdjacentValue = max(self.lookLeft(row, col), highestAdjacentValue)
        highestAdjacentValue = max(self.lookRight(row, col), highestAdjacentValue)
        highestAdjacentValue = max(self.lookUp(row, col), highestAdjacentValue)
        highestAdjacentValue = max(self.lookDown(row, col), highestAdjacentValue)
        return highestAdjacentValue
    
    def getAdjacentPoints(self, row, col, maxHeight) -> list:
        adjacentPoints = []
        # Ignore points higher than maxHeight which consitute the basin's edge
        if col > 0 and self.grid[row][col-1] < maxHeight:
            adjacentPoints.append((row, col-1)) # check left edge
        if col < self.colCount - 1 and self.grid[row][col+1] < maxHeight:
            adjacentPoints.append((row, col+1)) # check right edge
        if row > 0 and self.grid[row-1][col] < maxHeight:
            adjacentPoints.append((row-1, col)) # check top edge
        if row < self.rowCount - 1 and self.grid[row+1][col] < maxHeight:
            adjacentPoints.append((row+1, col)) # check bottom edge
        return adjacentPoints
    
    def calcRiskLevel(self, row, col):
        return 1 + self.grid[row][col]

    def findLowPoints(self):
        lowPoints = []
        for row in range(self.rowCount):
            for col in range(self.colCount):
                if self.getLowestAdjacentValue(row, col) > self.grid[row][col]:
                    lowPoints.append((row, col))
                
        return lowPoints

    def findBasin(self, lowPoint):
        row = lowPoint[0]
        col = lowPoint[1]
        basin = [(row, col)]
        basinFilled = False
        while not basinFilled:
            newPoints = []
            for point in basin:
                row = point[0]
                col = point[1]
                # Get list adjacent points less than the max height and not already in basin
                adjacentPoints = self.getAdjacentPoints(row, col, maxHeight=9)
                newPoints += list(filter(lambda x: x not in basin and x not in newPoints, adjacentPoints))

            basin += newPoints
            
            if len(newPoints) == 0:
                basinFilled = True

        return basin
                
    def findBasins(self, lowPoints):
        basins = []
        for lowPoint in lowPoints:
            basins.append(self.findBasin(lowPoint))

        return basins
            

    def calcBasinSize(self, basin):
        basinSize = 0
        return basinSize


def readInputFile(filePath):
    with open(filePath, 'r+') as f:
        grid = []
        for line in f:
            grid.append([int(x) for x in list(line.rstrip())])
            
        return grid


def part1(heightMap):
    heightMap = HeightMap(input)
    lowPoints = heightMap.findLowPoints()
    riskLevels = []
    for p in lowPoints:
        riskLevels.append(heightMap.calcRiskLevel(p[0], p[1]))

    print('Sum of risk levels:', sum(riskLevels))

def part2(heightMap):
    heightMap = HeightMap(input)
    lowPoints = heightMap.findLowPoints()
    basins = heightMap.findBasins(lowPoints)
    basinSizes = list(map(lambda x: len(x), basins))
    threeLargestBasins = sorted(zip(basinSizes, basins), reverse=True)[:3]
    threeLargestBasinSizes = [x[0] for x in threeLargestBasins]
    print('Size of 3 largest basins:', functools.reduce(lambda a, b: a*b, threeLargestBasinSizes))


if __name__ == '__main__':
    if len(sys.argv) != 2:
        print('Usage: ')
    else:
        input = readInputFile(sys.argv[1])

        part1(input)
        part2(input)


