# Day 9: Smoke Basin
import sys
import functools

class Chunk:
    def __init__(self, input, parentChunk):
        self.input = input
        self.parentChunk
        self.openSymbols = '([{<'
        self.closeSymbols = ')]}>'
        self.symbolMap = {
            '(': ')',
            '[': ']',
            '{': '}',
            '<': '>',
        }
        # self.internalChunks = self.getInternalChunks()
        self.isIncomplete = False
        self.isCorrupted = False
        self.syntaxErrors = {}
        [self.syntaxErrors.setdefault(x, 0) for x in self.closeSymbols]

    def findClosingSymbolIndex(self, index):
        print(self.input)
        openSymbolCount = 0
        for i in range(index + 1, len(self.input) - 1):
            char = self.input[i]
            # Only close chunk if all other internal chunks with the same openSymbol have closed
            # print(openSymbolCount, i, char)
            if char == self.input[index]:
                openSymbolCount += 1
            elif char in self.closeSymbols and char == self.symbolMap[self.input[index]]:
                if openSymbolCount > 0:
                    openSymbolCount -= 1
                else:
                    # Chunk closing symbol found
                    return i

        return None

    def findCorruptedChunks(self):
        startingChunk = None
        # Find first valid atomic chunk "()", "[]", "{}", "<>"
        for i, char in enumerate(self.input):
            # Find valid open symbol and look ahead 1 for closing symbol
            if char in self.openSymbols and self.input[i+1] == self.symbolMap[char]:
                startingChunk = char + self.symbolMap[char]
                
                openSymbols = []
                closeSymbols = []
                # look left for opening symbol
                if i > 0 and input[i-1] in self.openSymbols:
                    openSymbols.append(input[i-1]) 
                # look right for closing symbol
                elif i < len(input) - 2 and input[i+1] in self.closeSymbols:
                    closeSymbols.append(input[i+1])

        # ATOMIC (valid) chunk expansion
        #   if closing sym on left AND right => inside larger chunk (right-side)
        #   if opening sym on left AND right => inside larger chunk (left-side)
        #   if opening sym on left AND closing sym on right =>  
        #       if they match valid parent chunk
        #       else invalid parent chunk
        #   if closing sym on left AND opening sym on left =>
        #       reached max expansion range for given atomic chunk
        #
        # EDGE CASES
        #   LEFT-MOST chunk
        #       if opening sym on right then valid chunk otherwise invalid chunk
        #   RIGHT-MOST chunk
        #       if closing sym on left then valid chunk otherwise invalid chunk



        if startingChunk:
            print('do stuff')
             


    def getInternalChunks(self):
        # find open symbol and its closing symbol and recursively retrieve internal chunks
        chunks = []
        # if functools.reduce(lambda a, b: a.isCorrupted | b.isCorrupted, self.internalChunks):
            # self.isCorrupted = True
        
        index = self.findClosingSymbolIndex(self.input[0])
        while index:
            index = self.findClosingSymbolIndex(self.input[index + 1])
        return chunks


def readInputFile(filePath):
    with open(filePath, 'r+') as f:
        lines = []
        for line in f:
            lines.append(line.rstrip())
            
        return lines


def part1(input):
    print('Part 1 solution:', 0)
    # for x in input:
        # print(x)
        # chunk = Chunk(x)
        # print(chunk.findClosingSymbolIndex(chunk.input[3]))
    # chunk = Chunk(input[1])
    # index = chunk.findClosingSymbolIndex(0)
    for line in input:
        chunk = Chunk(line)
        index = chunk.findClosingSymbolIndex(0)
        if index == None:
            print('NO CLOSING SYMBOL FOUND')
        else:
            print('i:', index, 'val:', chunk.input[index])


def part2(input):
    print('Part 2 solution:', 0)


if __name__ == '__main__':
    if len(sys.argv) != 2:
        print('Usage: ')
    else:
        input = readInputFile(sys.argv[1])

        part1(input)
        # part2(input)


