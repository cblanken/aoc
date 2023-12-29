import sys

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


