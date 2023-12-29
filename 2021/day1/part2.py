import sys

def readInputFile(filePath):
    with open(filePath, 'r+') as f:
        lines = f.readlines()
        return [int(x) for x in lines]

def countDepthIncreases(depths, windowSize=1):
    increaseCount = 0
    decreaseCount = 0
    noChangeCount = 0
    last_sum = sum(depths[0:windowSize])
    depths = depths[1:]
    for i in range(len(depths)):
        if i > len(depths) - windowSize:
            break

        # Get sum based on window size
        windowSum = 0
        for j in range(0, windowSize):
            windowSum += depths[i + j] 

        # Increment depths change counts
        if windowSum > last_sum:
            increaseCount += 1
        elif windowSum < last_sum:
            decreaseCount += 1
        else:
            noChangeCount += 1

        last_sum = windowSum

    return (increaseCount, decreaseCount, noChangeCount)

if __name__ == '__main__':
    if len(sys.argv) != 2:
        print('Usage: ')
    else:
        depths = readInputFile(sys.argv[1])
        print(countDepthIncreases(depths, 3))
        
