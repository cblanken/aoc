# Day 14: Extended Polymerization
import sys
import copy
import time

def pairInsertion(template, rules, countsByRule):
    res = ""
    resCountByElement = {}
    for i in range(len(template) - 1):
        pair = template[i] + template[i+1]
        res += pair[0] + rules[pair]
        
        # Increment count for first character
        resCountByElement[pair[0]] = resCountByElement[pair[0]] + 1 if pair[0] in resCountByElement.keys() else 1

        # Increment counts for inserted string
        # if countsByRule:

        for ele, val in countsByRule[pair].items():
            resCountByElement[ele] = resCountByElement[ele] + val if ele in resCountByElement.keys() else val

    res += template[-1]

    # Increment count for final character in string
    resCountByElement[template[-1]] = resCountByElement[template[-1]] + 1 if template[-1] in resCountByElement.keys() else 1

    return (res, resCountByElement)

def expandPairInsertionRules(stepCount, rules, countsByRule):
    # Count for default ruleset
    elementCountsByPair = {k:{} for (k,_) in rules.items()}

    if stepCount == 0:
        elementCountsByPair = {k:{v:1} for (k,v) in rules.items()}
        return (rules, elementCountsByPair)
    else:
        res = {k:k for (k,_) in rules.items()}

    for _ in range(stepCount):
        print("Step", _)
        newRules = {}
        elementCountsByPair = {k:{} for (k,_) in rules.items()}
        for key, template in res.items():
            # Update expansion rules
            newTemplate = pairInsertion(template, rules, countsByRule)
            newRules[key] = newTemplate[0]
            elementCounts = newTemplate[1]

            # Remove count for first character in newtemplate for accurate counting
            elementCounts[template[0]] = elementCounts[template[0]] - 1
            # Remove count for last character in newTemplate for accurate counting
            elementCounts[template[-1]] = elementCounts[template[-1]] - 1

            # Track element counts
            for e, value in elementCounts.items():
                if key in elementCountsByPair.keys() and e in elementCountsByPair[key].keys():
                    elementCountsByPair[key][e] = elementCountsByPair[key][e] + value
                else:
                    elementCountsByPair[key][e] = value

        res = copy.deepcopy(newRules)

    # Remove start and end to leave only the insertion rule
    res = {k:v[1:-1] for (k,v) in res.items()}
    return (res, elementCountsByPair)

def expandPairInsertionRulesFast(stepCount, stepSize, rules, countsByRule):
    # Count for default ruleset
    elementCountsByPair = {k:{} for (k,_) in rules.items()}

    if stepCount == 0:
        elementCountsByPair = {k:{v:1} for (k,v) in rules.items()}
        return (rules, elementCountsByPair)
    else:
        res = {k:k for (k,_) in rules.items()}

    newRules = rules
    for _ in range(stepCount):
        print("Step", _)

        newRules = expandPairInsertionRules(stepSize, newRules, countsByRule)

        elementCountsByPair = {k:{} for (k,_) in rules.items()}
        for key, template in res.items():
            # Update expansion rules
            newTemplate = pairInsertion(template, rules, countsByRule)
            newRules[key] = newTemplate[0]
            elementCounts = newTemplate[1]

            # Remove count for first character in newtemplate for accurate counting
            elementCounts[template[0]] = elementCounts[template[0]] - 1
            # Remove count for last character in newTemplate for accurate counting
            elementCounts[template[-1]] = elementCounts[template[-1]] - 1

            # Track element counts
            for e, value in elementCounts.items():
                if key in elementCountsByPair.keys() and e in elementCountsByPair[key].keys():
                    elementCountsByPair[key][e] = elementCountsByPair[key][e] + value
                else:
                    elementCountsByPair[key][e] = value

        res = copy.deepcopy(newRules)

    # Remove start and end to leave only the insertion rule
    res = {k:v[1:-1] for (k,v) in res.items()}
    return (res, elementCountsByPair)


def readInputFile(filePath):
    with open(filePath, 'r+') as f:
        # Parse polymer template
        polymerTemplate = f.readline().rstrip()

        # Parse pair insertion rules
        insertionRules = {}
        line = f.readline() # ignore newline
        line = f.readline()
        while line:
            rule = [x.strip() for x in line.split('->')]
            insertionRules[rule[0]] = rule[1]
            line = f.readline()

        return (polymerTemplate, insertionRules)

def part1(input):
    polymer = input[0]
    rules = input[1]

    leastCommonElementCount = ''
    mostCommonElementCount = ''
    for i in range(1, 11):
        results = pairInsertion(polymer, rules)
        polymer = results[0]
        countByElement = results[1]
        # print('step {0}: {1}'.format(i, polymer))
        print('Step {0}: Count: {1}'.format(i, countByElement))

        leastCommonElementCount = min([countByElement[x] for x in countByElement.keys()])
        mostCommonElementCount = max([countByElement[x] for x in countByElement.keys()])

    print('Part 1:' , mostCommonElementCount - leastCommonElementCount)


def part2(input):
    polymer = input[0]
    rules = input[1]
    countsByRule = {k1:{v:1} for (k1,v) in rules.items()}

    leastCommonElementCount = ''
    mostCommonElementCount = ''

    # newRules = expandPairInsertionRules(2, rules, countsByRule)
    # print(newRules[1])
    # newRules = expandPairInsertionRules(2, newRules[0], countsByRule)
    # print(newRules[1])
    # newRules = expandPairInsertionRules(4, rules, countsByRule)
    # print(newRules)

    # s1 = time.time()
    # newRules40 = expandPairInsertionRules(40, rules, countsByRule)
    # s2 = time.time()
    # print(s2-s1)

    # print('')


    # s1 = time.time()
    newRules1 = expandPairInsertionRules(2, rules, countsByRule)
    print(pairInsertion('NNCB', newRules1[0], newRules1[1])[1])
    # s2 = time.time()
    # print(s2-s1)


    # print(pairInsertion('NNCB', newRules1[0], newRules1[1]))
    # print('')

if __name__ == '__main__':
    if len(sys.argv) != 2:
        print('Usage: ')
    else:
        input = readInputFile(sys.argv[1])
        # part1(input)
        part2(input)
