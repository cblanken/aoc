# Day 9: Smoke Basin
import sys

def boldText(text):
    return '\033[1m' + text + '\033[0m'

class OctoGrid:
    def __init__(self, energyLevels: list):
        self.energyLevels = energyLevels
        self.rowCount = len(energyLevels)
        self.colCount = len(energyLevels[0])
        self.flashCount = 0
        self.flashed = []
    
    def __repr__(self):
        lines = [''.join(str(x)) for x in self.energyLevels]
        return '\n'.join(lines)

    def isSynced(self):
        for row in self.energyLevels:
            for level in row:
                if level != 0:
                    return False

        return True

    def step(self):
        self.flashed = []
        for r, row in enumerate(self.energyLevels):
            for c, level in enumerate(row):
                self.energyLevels[r][c] += 1

        for r, row in enumerate(self.energyLevels):
            for c, level in enumerate(row):
                if level > 9:
                    self.flash(r, c)
        
        for pos in self.flashed:
            self.energyLevels[pos[0]][pos[1]] = 0
                

    def updateEnergyLevel(self, row, col):
        self.energyLevels[row][col] += 1
        if self.energyLevels[row][col] > 9 and (row, col):
            self.flash(row, col)
            
    def flash(self, row, col):
        if (row, col) not in self.flashed:
            self.flashCount += 1
            self.flashed.append((row, col))

            if row > 0:
                self.updateEnergyLevel(row-1, col) # up
            if row > 0 and col > 0:
                self.updateEnergyLevel(row-1, col-1) # up-left
            if row > 0 and col + 1 < self.colCount:
                self.updateEnergyLevel(row-1, col+1) # up-right
            if row + 1 < self.rowCount:
                self.updateEnergyLevel(row+1, col) # down
            if row + 1 < self.rowCount and col > 0:
                self.updateEnergyLevel(row+1, col-1) # down-left
            if row + 1 < self.rowCount and col + 1 < self.colCount:
                self.updateEnergyLevel(row+1, col+1) # down-right
            if col > 0:
                self.updateEnergyLevel(row, col-1) # left
            if col + 1 < self.colCount:
                self.updateEnergyLevel(row, col+1) # right
        

def readInputFile(filePath):
    with open(filePath, 'r+') as f:
        lines = []
        for line in f:
            lines.append([int(x) for x in line.rstrip()])
            
        return lines


def part1(input, steps):
    octoGrid = OctoGrid(input)
    for _ in range(steps):
        octoGrid.step()
        print()
        print('Step', _ + 1)
        # print(octoGrid)
        # print('Flashed: ', sorted(octoGrid.flashed))

    print('Flashes: ', octoGrid.flashCount)


def part2(input, steps):
    octoGrid = OctoGrid(input)
    for _ in range(steps):
        octoGrid.step()
        print('Step', _ + 1)

        if octoGrid.isSynced():
            print(octoGrid)
            print('OCTOPUS FLASHES SYNCED! GO! GO! GO!')
            break


if __name__ == '__main__':
    if len(sys.argv) != 2:
        print('Usage: ')
    else:
        input = readInputFile(sys.argv[1])

        # part1(input, 100)
        part2(input, 1000)


