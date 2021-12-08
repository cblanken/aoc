import sys

# Seven Segement Display Format
#  0:      1:      2:      3:      4:
#  aaaa    ....    aaaa    aaaa    ....
# b    c  .    c  .    c  .    c  b    c
# b    c  .    c  .    c  .    c  b    c
#  ....    ....    dddd    dddd    dddd
# e    f  .    f  e    .  .    f  .    f
# e    f  .    f  e    .  .    f  .    f
#  gggg    ....    gggg    gggg    ....

#  5:      6:      7:      8:      9:
#  aaaa    aaaa    aaaa    aaaa    aaaa
# b    .  b    .  .    c  b    c  b    c
# b    .  b    .  .    c  b    c  b    c
#  dddd    dddd    ....    dddd    dddd
# .    f  e    f  .    f  e    f  .    f
# .    f  e    f  .    f  e    f  .    f
#  gggg    gggg    ....    gggg    gggg

class SegmentWireMapping:
    def __init__(self, signalPatterns: list, outputValues: list):
        self.signalPatterns = signalPatterns
        self.outputValues = outputValues
        self.a = 'a'
        self.b = 'b'
        self.c = 'c'
        self.d = 'd'
        self.e = 'e'
        self.f = 'f'
        self.g = 'g'
        self.digitMap = self.mapDigits()

    def __repr__(self):
        return 'a={0}, b={1}, c={2}, d={3}, e={4}, f={5}, g={6}'.format(self.a, self.b, self.c, self.d, self.e, self.f, self.g)

    def mapDigits(self):
        # Find 1, 4, 7, and 8 to map segments
        one = list(filter(lambda x: len(x) == 2, self.signalPatterns))[0]
        four = list(filter(lambda x: len(x) == 4, self.signalPatterns))[0]
        seven = list(filter(lambda x: len(x) == 3, self.signalPatterns))[0]
        eight = list(filter(lambda x: len(x) == 7, self.signalPatterns))[0]

        # Segment 'a' with set difference between 1 and 7
        set1 = set(one)
        set7 = set(seven)
        self.a = list(set7 - set1)[0]
        
        # Segment 'd' doesn't exist in the 0 pattern making it possible to infer the 
        # 'd' and 'b' segments
        zeroSixNine = list(filter(lambda x: len(x) == 6, self.signalPatterns))
        set4 = set(four)
        bd = list(set4 - set1)
        for signal in zeroSixNine:
            for i, letter in enumerate(bd):
                if letter not in signal:
                    self.d = letter
                    self.b = bd[(i+1) % len(bd)]
                    break
        
        # Segent 'g' 
        twoThreeFive = list(filter(lambda x: len(x) == 5, self.signalPatterns))
        acdf = set1.union({self.a, self.d})
        for signal in twoThreeFive:
            if one[0] in signal and one[1] in signal: # found signal representing 3
                self.g = list(set(signal) - acdf)[0]
                break

        # Segment 'e'
        set8 = set(eight)
        abcdfg = set1.union({self.a, self.b, self.d, self.g})
        self.e = list(set8 - abcdfg)[0]

        # Segments 'c' and 'f' differentiation
        for signal in twoThreeFive:
            if self.e in signal: # found signal representing 2
                for i, letter in enumerate(one):
                    if letter in signal:
                        self.c = letter
                        self.f = one[(i+1) % len(one)]
                        break

        zero = set([self.a, self.b, self.c, self.e, self.f, self.g])
        one = set(one)
        two = set([self.a, self.c, self.d, self.e, self.g])
        three = set([self.a, self.c, self.d, self.f, self.g])
        four = set(four)
        five = set([self.a, self.b, self.d, self.f, self.g])
        six = set([self.a, self.b, self.d, self.e, self.f, self.g])
        seven = set(seven)
        eight = set(eight)
        nine = set([self.a, self.b, self.c, self.d, self.f, self.g])

        return (zero, one, two, three, four, five, six, seven, eight, nine)

    def identifyOutput(self):
        output = ''
        for digit in self.outputValues:
            digitSet = set(list(digit))
            for i, n in enumerate(self.digitMap):
                # print('digitSet:', digitSet,'n:', n)
                if digitSet == n:
                    output += str(i)

        return int(output)

def readInputFile(filePath):
    with open(filePath, 'r+') as f:
        displays = []
        line = f.readline().rstrip().split('|')
        while line[0]:
            signalPatterns = line[0].strip().split(' ')
            outputValues = line[1].strip().split(' ')
            displays.append((signalPatterns, outputValues))
            line = f.readline().rstrip().split('|')
        return displays

def identifyPatternPart1(pattern):
    if len(pattern) == 2:
        return 1
    elif len(pattern) == 4:
        return 4
    elif len(pattern) == 3:
        return 7
    elif len(pattern) == 7:
        return 8
    else:
        return -1

def part1(input):
    digitCount = 0 
    
    digitMappings = []
    for line in input:
        digitMapping = {}
        signalPatterns = line[0]
        outputValues = line[1]
        for pattern in outputValues:
            digitMapping[pattern] = identifyPatternPart1(pattern)
            if digitMapping[pattern] > 0:
                digitCount += 1
        digitMappings.append(digitMapping)

    # for mapping in digitMappings:
        # print(mapping)
    
    print('Part 1 digit count: {0}'.format(digitCount))

def part2(input):
    sum = 0
    for line in input:
        mapping = SegmentWireMapping(line[0], line[1])
        mapping.mapDigits()
        # print('Segment Mapping:', mapping)
        output = mapping.identifyOutput()
        print(output, '\t', mapping)
        sum += output

    print('Sum:', sum)



if __name__ == '__main__':
    if len(sys.argv) != 2:
        print('Usage: ')
    else:
        input = readInputFile(sys.argv[1])
        part1(input)
        part2(input)


