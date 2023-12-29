# Day 12: Octopathing
import sys
import copy

def findSubPaths(firstConn, connections: list, nodes: dict, start='start', end='end', depth=0, isPart2=False):
    lowerNodesVisitedTwice = {k:v for (k,v) in nodes.items() if k.islower() and v == 2}

    # Bubble up if more than one small cave visited twice
    if len(lowerNodesVisitedTwice) == 2:
        return []

    if firstConn[1] == end:
        return [[firstConn]]

    else:
        paths = []
        nodesCopy = copy.deepcopy(nodes)

        # Edges that can be taken based on provided rules
        validConnections = [x for x in connections if 
                x[0] == firstConn[1] and 
                not (x[1].islower() and nodes[x[1]] != 0)]

        # Allow 2nd visit to a single small (lowercase) cave
        # lowerNodesVisitedTwice = {k:v for (k,v) in nodes.items() if k.islower() and v == 2}

        if not isPart2 or len(lowerNodesVisitedTwice) > 0:
            for c in validConnections:
                # print('depth:', depth, 'first:', firstConn, 'c:', c, 'validConns:', validConnections)
                # print('nodesCopy:', nodesCopy)
                nodesCopy[c[0]] = nodes[c[0]] + 1


                subPaths = findSubPaths(c, connections, nodesCopy, start, end, depth=depth+1, isPart2=isPart2)
                if len(subPaths) > 0:
                    paths += [[firstConn] + x for x in subPaths]
        else:
            validConnections = [x for x in connections if 
                    x[0] == firstConn[1] and
                    not (x[1].islower() and nodes[x[1]] > 1)]

            for c in validConnections:
                nodesCopy[c[0]] = nodes[c[0]] + 1
                subPaths = findSubPaths(c, connections, nodesCopy, start, end, depth=depth+1, isPart2=isPart2)
                if len(subPaths) > 0:
                    paths += [[firstConn] + x for x in subPaths]

        return paths

def findPaths(connections: list, start='start', end='end', isPart2 = False):
    # Fix 'start' and 'end' edges if backwards
    connections = [(c[1], c[0]) if c[1] == start else c for c in connections]
    connections = [(c[1], c[0]) if c[0] == end else c for c in connections]

    startConnections = list(filter(lambda x: x[0] == start, connections))
    newConns = [(c[1], c[0]) for c in connections if c[0] != start and c[1] != end]

    connections += newConns 
    print('StartConns:', startConnections)
    print('Connections:', connections)

    nodes = {}
    for c in connections:
        nodes[c[0]] = 0
        nodes[c[1]] = 0

    paths = []
    for s in startConnections:
        paths += findSubPaths(s, connections, nodes, isPart2=isPart2)

    return paths


def readInputFile(filePath):
    with open(filePath, 'r+') as f:
        lines = []
        for line in f:
            lines.append(tuple(line.rstrip().split('-')))
            
        return lines


def part1(input):
    paths = findPaths(input)
    print('Part1: found {0} paths'.format(len(paths)))
    # for p in paths:
        # print('start,' + ','.join([x[1] for x in p]))


def part2(input):
    paths = findPaths(input, isPart2=True)
    print('Part2: found {0} paths'.format(len(paths)))
    # for p in paths:
        # print('start,' + ','.join([x[1] for x in p]))

if __name__ == '__main__':
    if len(sys.argv) != 2:
        print('Usage: ')
    else:
        input = readInputFile(sys.argv[1])
        # part1(input)
        part2(input)
