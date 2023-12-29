# Day 19: Beacon Scanner
import sys
from copy import deepcopy

# Assume default orientation face is +x, up is +z, left is +y
# Orientations stored by (x,y,z) 90° rotations
ORIENTATIONS = [
        # face +x
        (3, 0, 0), # up +y
        (1, 0, 0), # up -y
        (0, 0, 0), # up +z
        (2, 0, 0), # up -z
        # face -x
        (1, 0, 2), # up +y
        (3, 0, 2), # up -y
        (0, 0, 2), # up +z
        (0, 2, 0), # up -z
        # face +y
        (1, 0, 1), # up +x
        (3, 0, 1), # up -x
        (0, 0, 1), # up +z
        (2, 0, 1), # up -z
        # face -y
        (1, 0, 3), # up +x
        (3, 0, 3), # up -x
        (0, 0, 3), # up +z
        (0, 2, 1), # up -z
        # face +z
        (0, 3, 2), # up +x
        (0, 3, 0), # up -x
        (0, 3, 3), # up +y
        (0, 3, 1), # up -y
        # face -z
        (0, 1, 0), # up +x
        (0, 1, 2), # up -x
        (0, 1, 1), # up +y
        (0, 1, 3), # up -y
]
# NOTE: rotation are performed in the order of x, y, then z
# There is almost certainly a better way to encode all the possible orientations for the
# scanners to iterate through possible rotations by actually using linear algebra but
# this was the simplest solution I could think of without doing a bunch of studying

class Beacon:
    def __init__(self, x, y, z):
        self.x = x
        self.y = y
        self.z = z

    def __repr__(self):
        return str([self.x, self.y, self.z])


class Scanner:
    def __init__(self, beacons: list, rotation = [0, 0, 0], position = [0, 0, 0], maxRange = 100):
        self.beacons = beacons
        self.rotatedBeacons = beacons
        self.position = position
        self.rotation = rotation
        self.applyRotation(rotation)
        self.rotatedBeaconsWithPosition = self.rotatedBeacons
        self.applyPosition(position)
        self.maxRange = maxRange

        # Rotation: integer multiples of 90 degree rotations around x, y, z
        self.rotation = rotation

    def __repr__(self):
        return str(self.beacons)

    def applyPosition(self, pos = None):
        pos = self.position if pos == None else pos
        
        rotatedBeaconsWithPosition = deepcopy(self.rotatedBeacons)
        for i, r in enumerate(rotatedBeaconsWithPosition):
            rotatedBeaconsWithPosition[i].x -= pos[0] 
            rotatedBeaconsWithPosition[i].y -= pos[1] 
            rotatedBeaconsWithPosition[i].z -= pos[2] 
        self.rotatedBeaconsWithPosition = rotatedBeaconsWithPosition
        return rotatedBeaconsWithPosition


    def applyRotation(self, r = None):
        r = self.rotation if r == None else r
        rotatedBeacons = deepcopy(self.beacons)
        for i, b in enumerate(rotatedBeacons):
            b = self.rotateX(r[0], b)
            b = self.rotateY(r[1], b)
            b = self.rotateZ(r[2], b)
            rotatedBeacons[i] = b

        self.rotatedBeacons = rotatedBeacons
        return rotatedBeacons


    # Update position of beacon for rotation around x axis 's' times
    def rotateX(self, s: int, beacon: Beacon):
        pos = deepcopy(beacon)
        if s % 4 == 1:
            pos.y = beacon.z
            pos.z = -beacon.y
        elif s % 4 == 2:
            pos.y = -beacon.y
            pos.z = -beacon.z
        elif s % 4 == 3:
            pos.y = -beacon.z
            pos.z = beacon.y

        return pos

    # Update position of beacon for rotation around y axis 's' times
    def rotateY(self, s: int, beacon: Beacon):
        pos = deepcopy(beacon)
        if s % 4 == 1:
            pos.x = -beacon.z
            pos.z = beacon.x
        elif s % 4 == 2:
            pos.x = -beacon.x
            pos.z = -beacon.z
        elif s % 4 == 3:
            pos.x = beacon.z
            pos.z = -beacon.x

        return pos

    # Update position of beacon for rotation around z axis 's' times
    def rotateZ(self, s: int, beacon: Beacon):
        pos = deepcopy(beacon)
        if s % 4 == 1:
            pos.x = beacon.y
            pos.y = -beacon.x
        elif s % 4 == 2:
            pos.x = -beacon.x
            pos.y = -beacon.y
        elif s % 4 == 3:
            pos.x = -beacon.y
            pos.y = beacon.x

        return pos

def findAllScannerRotations(scanner: Scanner):
    rotatedScanners = {}
    for o in ORIENTATIONS:
        x = o[0]
        y = o[1]
        z = o[2]
        newScanner = deepcopy(scanner)
        newScanner.applyRotation((x, y, z))
        rotatedScanners[(x,y,z)] = newScanner

    return rotatedScanners

def findAllScannerRotationsWithPosition(scanner: Scanner, pos: list):
    rotatedScanners = {}
    for o in ORIENTATIONS:
        x = o[0]
        y = o[1]
        z = o[2]
        newScanner = deepcopy(scanner)
        newScanner.applyRotation((x, y, z))
        newScanner.applyPosition([pos[0], pos[1], pos[2]])
        rotatedScanners[(x,y,z)] = newScanner

    return rotatedScanners

def findScannerOverlapBruteForce(s1: Scanner, s2: Scanner):
    s1Rotations = findAllScannerRotations(s1)
    s2Rotations = findAllScannerRotations(s2)
    for x in range(s1.position[0] - s1.maxRange, s1.position[0] + s1.maxRange):
        print('x:', x)
        for y in range(s1.position[1] - s1.maxRange, s1.position[1] + s1.maxRange):
            print('y:', y)
            for z in range(s1.position[2] - s1.maxRange, s1.position[2] + s1.maxRange):
    # for x in range(67, 69):
        # print('x:', x)
        # for y in range(-1247, -1245):
            # print('y:', y)
            # for z in range(-44, -42):
                # print('z:', z)
                overlapCount = 0
                for r in s2Rotations.values():
                    r.applyPosition([x, y, z])
                for k1 in s1Rotations.keys():
                    for k2 in s2Rotations.keys():
                        if s1Rotations[k1].rotatedBeaconsWithPosition == s2Rotations[k2].rotatedBeaconsWithPosition:
                            overlapCount += 1
                            if overlapCount >= 12:
                                return (x, y, z)
                        # print('over:', overlapCount)
                s2.position = [x, y, z]
                s2RotationsTemp = []
                        
    return [0, 0, 0]

def readInputFile(filePath: str):
    with open(filePath, 'r+') as f:
        scanners = {}
        line = True
        while line:
            line = f.readline().rstrip().split(' ')
            scannerID = line[2]
            beacons = []
            line = f.readline()
            while len(line) > 1:
                pos = [int(x) for x in line.rstrip().split(',')]
                if len(pos) == 2:
                    beacons.append(Beacon(pos[0], pos[1], 0))
                else:
                    beacons.append(Beacon(pos[0], pos[1], pos[2]))
                line = f.readline()

                 
            scanners[scannerID] = Scanner(beacons)
        return scanners

def part1(input):
    # print('Part1:', scanners)

    scanners = input
    s0 = scanners['0']
    s1 = scanners['1']
    # r0 = findAllScannerRotations(s0)
    # r1 = findAllScannerRotations(s1)
    # for k, r in r2.items():
        # print(k, r)
    # print('')
    s2RelPos = findScannerOverlapBruteForce(s0, s1)
    print(s2RelPos)


def part2(input):
    print('Part2:', input)

if __name__ == '__main__':
    if len(sys.argv) != 2:
        print('Usage: ')
    else:
        input = readInputFile(sys.argv[1])
        part1(input)
        # part2(input)
