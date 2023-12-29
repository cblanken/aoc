import sys

def readInputFile(filePath):
    with open(filePath, 'r+') as f:
        lines = f.read().splitlines()
        return lines

def getMostCommonBit(lines, col):
    onesCount = 0
    for l in lines:
        if l[col] == '1':
            onesCount += 1

    if onesCount >= len(lines) / 2:
        return '1'
    else:
        return '0'

def getLeastCommonBit(lines, col):
    onesCount = 0
    for l in lines:
        if l[col] == '1':
            onesCount += 1

    if onesCount < len(lines) / 2:
        return '1'
    else:
        return '0'


def getLifeSupportRating(lines):
    oxygen_lines = co2_lines = lines

    for i in range(len(lines[0])):
        if len(oxygen_lines) > 1:
            oxygen_mcb = getMostCommonBit(oxygen_lines, i)
            oxygen_lines = list(filter(lambda x: x[i] == oxygen_mcb, oxygen_lines))
            print("oxy mcb:", oxygen_mcb)

        if len(co2_lines) > 1:
            co2_lcb = getLeastCommonBit(co2_lines, i)
            co2_lines = list(filter(lambda x: x[i] == co2_lcb, co2_lines))
            print("co2 lcb:", co2_lcb)

    print(oxygen_lines)
    oxygen_rating = oxygen_lines[0]

    print(co2_lines)
    co2_rating = co2_lines[0]

    lifeSupportRating = int(oxygen_rating, 2) * int(co2_rating, 2)
    return lifeSupportRating

if __name__ == '__main__':
    if len(sys.argv) != 2:
        print('Usage: ')
    else:
        lines = readInputFile(sys.argv[1])
        lifeSupportRating = getLifeSupportRating(lines)
        print('Life Support Rating: {0}'.format(lifeSupportRating))
