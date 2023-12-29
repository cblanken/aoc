import sys
import functools

class Line:
    def __init__(self, x1, y1, x2, y2):
        self.p1 = (x1, y1)
        self.p2 = (x2, y2)
    
    def __str__(self):
        return 'p1: {0}, p2: {1}'.format(self.p1, self.p2)



def readInputFile(filePath):
    with open(filePath, 'r+') as f:
        lanternFishes = f.readline().rstrip().split(',')
        lanternFishes = [int(x) for x in lanternFishes]
        return lanternFishes


if __name__ == '__main__':
    if len(sys.argv) != 2:
        print('Usage: ')
    else:
        lanternFishes = readInputFile(sys.argv[1])
        print(lanternFishes)

        days = 256

        # Keep count of fish with 'x' days remaining
        lanternFishTimers = [0] * 9
        for t in lanternFishes:
            lanternFishTimers[t] += 1

        print(lanternFishTimers)

        for i in range(days):
            newFishCount = lanternFishTimers[0]
            
            # Subtract one from each timer
            for f in range(len(lanternFishTimers) - 1):
                lanternFishTimers[f] = lanternFishTimers[f+1]

            # Update new fish count
            lanternFishTimers[6] += newFishCount
            lanternFishTimers[8] = newFishCount
            print('Day {0}: {1}'.format(i, lanternFishTimers))

        # print(lanternFishTimers)
        totalLanternFishies = functools.reduce(lambda a,b: a + b, lanternFishTimers)
        print('Total lantern fishies: {0}'.format(totalLanternFishies))


