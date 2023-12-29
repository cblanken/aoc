# Day 16: Packet Decoder
import sys
from functools import reduce

def hexToBin(hex):
    hexMap = {
        '0': '0000', 
        '1': '0001',
        '2': '0010',
        '3': '0011',
        '4': '0100',
        '5': '0101',
        '6': '0110',
        '7': '0111',
        '8': '1000',
        '9': '1001',
        'A': '1010',
        'B': '1011',
        'C': '1100',
        'D': '1101',
        'E': '1110',
        'F': '1111'
    }

    return ''.join([hexMap[c] for c in hex])

def readInputFile(filePath):
    with open(filePath, 'r+') as f:
        transmission = f.readline().rstrip()

    return transmission

def parsePacket(packet, startIndex = 0, depth = 0):

    version = int(packet[startIndex:startIndex+3], 2)
    typeID = int(packet[startIndex+3:startIndex+6], 2)

    literal = ""
    headerOffset = startIndex + 6
    groups = []
    lastIndex = 0
    subPackets = [] 
    if int(typeID) == 4: # packet represents literal value
        i = headerOffset
        while True:
            # offset = headerOffset + i
            group = packet[i+1:i+5]
            groups.append(group)
            if packet[i] == '0': # last group
                break
            i += 5
        i += 5

        lastIndex = i
        literal = int(''.join(groups), 2)
    else: # packet represents an operator
        lengthTypeID = packet[headerOffset]
        totalLengthOfSubPackets = 0
        currentSubPacketCount = 0
        if lengthTypeID == '0': # next 15 bits represent total length in bits of sub-packets
            totalLengthOfSubPackets = int(packet[headerOffset+1:headerOffset+16], 2)
            offset = headerOffset + 16
            # subPacketRange = packet[offset:offset+totalLengthOfSubPackets]

            i = offset
            while i < offset + totalLengthOfSubPackets - 1:
                subPacket = parsePacket(packet, i, depth = depth + 1)
                subPackets.append(subPacket) 
                i = subPacket[4]
            lastIndex = i

        elif lengthTypeID == '1': # next 11 bits represent the number of subpackets
            totalSubPacketCount = int(packet[headerOffset+1:headerOffset+12], 2)
            offset = headerOffset + 12
            subPackets = []

            i = offset
            for _ in range(totalSubPacketCount):
                if i >= len(packet):
                    break
                subPacket = parsePacket(packet, i, depth = depth + 1)
                subPackets.append(subPacket) 
                i = subPacket[4]
            lastIndex = i

    return (version, typeID, groups, literal, lastIndex, subPackets)

# Recursively search a packet and sum all the version numbers
def getTotalVersionSum(parsedPacket):
    versionSum = parsedPacket[0]
    subPackets = parsedPacket[5]
    for subPacket in subPackets:
        versionSum += getTotalVersionSum(subPacket)

    return versionSum

def evalPacket(parsedPacket):
    if parsedPacket[1] == 4: # literal packet
        return parsedPacket[3]
    else:
        vals = []
        subPackets = parsedPacket[5]
        for subPacket in subPackets:
            vals.append(evalPacket(subPacket))

        typeID = parsedPacket[1]
        if typeID == 0: # sum
            return sum(vals)
        elif typeID == 1: # product
            return reduce(lambda a,b: a * b, vals)
        elif typeID == 2: # minimum
            return min(vals)
        elif typeID == 3: # maximim
            return max(vals)
        elif typeID == 5: # greater than
            return 1 if vals[0] > vals[1] else 0
        elif typeID == 6: # less than
            return 1 if vals[0] < vals[1] else 0
        elif typeID == 7: # equal to
            return 1 if vals[0] == vals[1] else 0

def part1(input):
    bin = hexToBin(input)
    parsedPacket = parsePacket(bin)
    print('Part1:', getTotalVersionSum(parsedPacket))


def part2(input):
    bin = hexToBin(input)
    parsedPacket = parsePacket(bin)
    print('Part2:', evalPacket(parsedPacket))

if __name__ == '__main__':
    if len(sys.argv) != 2:
        print('Usage: ')
    else:
        input = readInputFile(sys.argv[1])
        part1(input)
        part2(input)
