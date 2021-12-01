import sys

def readInputFile(filePath):
    with open(filePath, 'r+') as f:
        lines = f.readlines()
        return [int(x) for x in lines]

def countDepthIncreases(depths, windowSize=1):
    increaseCount = 0
    decreaseCount = 0
    noChangeCount = 0
    last_depth = depths[0] 
    depths = depths[1:]
    for depth in depths:
        if depth > last_depth:
            increaseCount += 1
        elif depth < last_depth:
            decreaseCount += 1
        else:
            noChangeCount += 1

        last_depth = depth

    return (increaseCount, decreaseCount, noChangeCount)

if __name__ == '__main__':
    if len(sys.argv) != 2:
        print('Usage: ')
    else:
        depths = readInputFile(sys.argv[1])
        print(countDepthIncreases(depths))
        
