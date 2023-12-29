import sys

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

        days = 80

        for i in range(days):
            newFishCount = 0
            for j in range(len(lanternFishes)):
                if lanternFishes[j] == 0:
                    lanternFishes[j] = 6
                    newFishCount += 1
                else:
                    lanternFishes[j] -= 1

            if newFishCount > 0:
                lanternFishes.extend([8] * newFishCount)

        # print(lanternFishes)
        print('Total lantern fishies: {0}'.format(len(lanternFishes)))


