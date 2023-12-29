# Day 20: Trench Map
import sys
from copy import deepcopy

def padImage(image, width = 10, height = 10):
    pad = '.' * width
    # Padding left and right of image
    image = [pad + x + pad for x in image]
    verticalPadding = ['.' * len(image[0])] * height

    paddedImage = verticalPadding + image + verticalPadding
    offset = (height, width)
    return (paddedImage, offset)

def getPixelIndex(image, row, col, zeroPixel):
    # Account for infinite canvas by alternating zero pixel based on algo
    maxRow = len(image) - 1
    maxCol = len(image[0]) - 1
    square = [
            image[row-1][col-1] if row > 0 and col > 0 else zeroPixel,
            image[row-1][col] if row > 0 else zeroPixel,
            image[row-1][col+1] if row > 0 and col < maxCol else zeroPixel,
            image[row][col-1] if col > 0 else zeroPixel, 
            image[row][col],
            image[row][col+1] if col < maxCol else zeroPixel,
            image[row+1][col-1] if row < maxRow and col > 0 else zeroPixel,
            image[row+1][col] if row < maxRow else zeroPixel,
            image[row+1][col+1] if row < maxRow and col < maxCol else zeroPixel
        ]
    
    bin = ''.join(['1' if x == '#' else '0' for x in square])
    algoIndex = int(bin, 2)
    return algoIndex 


def enhanceImage(image, algo, enhanceCount):
    enhancedImage = deepcopy(image)
    if algo[0] == '#':
        zeroPixel = algo[0] if enhanceCount % 2 == 1 else algo[-1]
        pad = algo[0] if enhanceCount % 2 == 0 else algo[-1]
    else:
        zeroPixel = '.'
        pad = '.'
    for row in range(0, len(image)):
        line = pad
        for col in range(0, len(image[0])):
            index = getPixelIndex(image, row, col, zeroPixel)
            line += algo[index]
        line += pad
        enhancedImage[row] = line

    return enhancedImage

def countLitPixels(image):
    count = 0
    for line in image:
        for pixel in line:
            if pixel == '#':
                count += 1

    return count


def readInputFile(filePath: str):
    with open(filePath, 'r+') as f:
        algo = f.readline().rstrip()
        f.readline() # skip newline
        line = f.readline().rstrip()
        image = []
        while line:
            image.append(line)
            line = f.readline().rstrip()
        # Image format image[row][col]
        # Algo format algo[i]
        return (algo, image)

def part1(input):
    algo = input[0]
    paddedImage = padImage(input[1], 10, 10)
    image = paddedImage[0]

    enhancedImage = enhanceImage(image, algo, 0)
    enhancedImage = enhanceImage(enhancedImage, algo, 1)
    litPixelCount = countLitPixels(enhancedImage)
    print('Part1:', litPixelCount)


def part2(input):
    algo = input[0]
    paddedImage = padImage(input[1], 10, 60)
    image = paddedImage[0]

    enhancedImage = enhanceImage(image, algo, 0)
    for i in range(1, 50):
        enhancedImage = enhanceImage(enhancedImage, algo, i)
    litPixelCount = (countLitPixels(enhancedImage))

    print('Part2:', litPixelCount)

if __name__ == '__main__':
    if len(sys.argv) != 2:
        print('Usage: ')
    else:
        input = readInputFile(sys.argv[1])
        part1(input)
        part2(input)
