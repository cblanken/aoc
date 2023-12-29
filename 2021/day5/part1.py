import sys

class Diagram:
    def getMaxSize(self):
        xMax = yMax = 0
        x1Max = max([line.p1[0] for line in self.lines])
        x2Max = max([line.p2[0] for line in self.lines])
        y1Max = max([line.p1[1] for line in self.lines])
        y2Max = max([line.p2[1] for line in self.lines])

        xMax = max(x1Max, x2Max) + 1
        yMax = max(y1Max, y2Max) + 1
        return (xMax, yMax)

    def drawLines(self):
        for line in self.lines:
            xDiff = line.p1[0] - line.p2[0]
            yDiff = line.p1[1] - line.p2[1]
            slope = int(yDiff / xDiff) if xDiff != 0 else 0
            b = yDiff - (slope * xDiff)

            xMin = min(line.p1[0], line.p2[0])
            yMin = min(line.p1[1], line.p2[1])

            xMax = max(line.p1[0], line.p2[0])
            yMax = max(line.p1[1], line.p2[1])

            # add horizontal lines
            if abs(xDiff) > 0 and yDiff == 0:
                for x in range(xMin, xMax+1):
                    self.board[yMin][x] += 1
                    if self.board[yMin][x] >= self.dangerLevel:
                        self.dangerZone[(x, yMin)] = self.board[yMin][x]
            # add vertical lines
            elif xDiff == 0 and abs(yDiff) > 0:
                for y in range(yMin, yMax+1):
                    self.board[y][xMin] += 1
                    if self.board[y][xMin] >= self.dangerLevel:
                        self.dangerZone[(xMin, y)] = self.board[x][yMin]
            # add diagonal lines
            # elif slope == 1 or slope == -1:
                # for x in range(xMin, xMax+1):
                    # y = slope * xDiff + b
                    # self.board[y][x] += 1
                    # if self.board[y][x] >= self.dangerLevel:
                        # self.dangerZone[(x, y)] = self.board[y][x]


    def __init__(self, lines, dangerLevel = 2):
        self.lines = lines
        self.maxSize = self.getMaxSize()
        self.board = [[0 for _ in range(self.maxSize[1])] for _ in range(self.maxSize[0])]
        self.dangerLevel = dangerLevel
        self.dangerZone = {} 

class Line:
    def __init__(self, x1, y1, x2, y2):
        self.p1 = (x1, y1)
        self.p2 = (x2, y2)
    
    def __str__(self):
        return 'p1: {0}, p2: {1}'.format(self.p1, self.p2)


def readInputFile(filePath):
    with open(filePath, 'r+') as f:
        line = f.readline()
        lineCoords = []
        while line:
            p1, p2 = line.rstrip().split(' -> ')
            x1, y1 = p1.split(',')
            x2, y2 = p2.split(',')

            lineCoords.append(Line(int(x1), int(y1), int(x2), int(y2)))

            line = f.readline()

        return lineCoords


if __name__ == '__main__':
    if len(sys.argv) != 2:
        print('Usage: ')
    else:
        lines = readInputFile(sys.argv[1])
        diag = Diagram(lines)
        diag.drawLines()

        for line in diag.board:
            print(line)

        print(diag.dangerZone)
        print('The danger zone includes {0} points'.format(len(diag.dangerZone)))
