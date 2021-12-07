import sys

class Line:
    def __init__(self, x1, y1, x2, y2):
        self.p1 = (x1, y1)
        self.p2 = (x2, y2)
    
    def __str__(self):
        return 'p1: {0}, p2: {1}'.format(self.p1, self.p2)



def readInputFile(filePath):
    with open(filePath, 'r+') as f:
        positions = f.readline().rstrip().split(',')
        positions = [int(x) for x in positions]
        return positions


if __name__ == '__main__':
    if len(sys.argv) != 2:
        print('Usage: ')
    else:
        positions = readInputFile(sys.argv[1])

        maxPos = max(positions)
        sumArr = [x for x in range(1, maxPos+1)]

        # Brute-force calculation
        fuelCosts = []
        for target in range(maxPos):
            print('Target:', target)
            fuelCost = 0
            for pos in positions:
                fuelCost += sum(sumArr[:abs(pos-target)]) 

            fuelCosts.append(fuelCost)

        minFuelCost = min(fuelCosts)
        print('Minimum fuel cost:', minFuelCost)


