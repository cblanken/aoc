# Day 21: Dirac Dice
import sys
from copy import deepcopy

class Player:
    def __init__(self, pos=1, score=0):
        self.pos = pos
        self.score = score

    def __repr__(self):
        return 'pos: {0}, score: {1}'.format(str(self.pos), str(self.score))

    def increasePos(self, val):
        self.pos = (self.pos - 1 + val) % 10 + 1

    def increaseScore(self, val):
        self.score += val

def deterministicDie(start: int):
    i = start
    while i < 9999999:
        num = i % 100
        yield num + 1
        i += 1
    
def diracDie():
    i = 0
    while i < 99999:
        num = i % 3
        yield num + 1

def playRoundDeterministic(p1: Player, p2: Player, die, turn: bool):
    # turn True = P1 turn, False = P2 turn
    rolls = []
    if turn:
        rolls.append(next(die))
        rolls.append(next(die))
        rolls.append(next(die))
        rollTotal = sum(rolls)
        p1.increasePos(rollTotal)
        p1.increaseScore(p1.pos)
    else:
        rolls.append(next(die))
        rolls.append(next(die))
        rolls.append(next(die))
        rollTotal = sum(rolls)
        p2.increasePos(rollTotal)
        p2.increaseScore(p2.pos)

def playGameDeterministic(p1: Player, p2: Player, maxScore: int, start: int = 0):
    die = deterministicDie(start) 
    rollCount = 0
    turn = True
    while p1.score < maxScore and p2.score < maxScore:
        # print('p1:', p1, '| p2:', p2)
        playRoundDeterministic(p1, p2, die, turn)
        rollCount += 3
        turn = not turn

    return rollCount


def playGameDirac(p1: Player, p2: Player, maxScore: int, turn: bool, start: int, wins: dict):

    rolls = [1, 2, 3]
    g1 = playRoundDirac(deepcopy(p1), deepcopy(p2), 1, turn)
    g2 = playRoundDirac(deepcopy(p1), deepcopy(p2), 2, turn)
    g3 = playRoundDirac(deepcopy(p1), deepcopy(p2), 3, turn)
    playGameDirac(deepcopy(p1), deepcopy(p2), maxScore, turn, rolls[0], wins)
    playGameDirac(deepcopy(p1), deepcopy(p2), maxScore, turn, rolls[1], wins)    
    playGameDirac(deepcopy(p1), deepcopy(p2), maxScore, turn, rolls[2], wins)    

    wins['p1'] += g1['p1'] + g2['p1'] + g3['p1']
    wins['p2'] += g1['p2'] + g2['p2'] + g3['p2']

    return wins

def readInputFile(filePath: str):
    with open(filePath, 'r+') as f:
        line = f.readline().rstrip()
        return line

def part1(input):
    p1 = Player(4)
    p2 = Player(8)

    rollCount = playGameDeterministic(p1, p2, 1000)
    losingPlayer = p2 if p1.score >= 1000 else p1
    print('Part1:', rollCount * losingPlayer.score)


def part2(input):
    p1 = Player(4)
    p2 = Player(8)

    rollCount = playGameDirac(p1, p2, 1000)
    losingPlayer = p2 if p1.score >= 1000 else p1
    print('Part1:', rollCount * losingPlayer.score)

if __name__ == '__main__':
    if len(sys.argv) != 2:
        print('Usage: ')
    else:
        input = readInputFile(sys.argv[1])
        part1(input)
        # part2(input)
