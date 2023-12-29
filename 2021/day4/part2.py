import sys

class BingoBoard:
    def __init__(self, size, board=None):
        self.size = size
        if board == None:
            self.board = [[0 * size]] * size
        else:
            self.board = board

        self.markedBoard = [[0 for _ in range(size)] for _ in range(size)]
        self.markedNumberCount = 0
        self.lastMarkedNumber = 0

    def setValue(self, row, col, value):
        self.board[row][col] = value

    def markValue(self, value):
        for row in range(self.size):
            for col in range(self.size):
                if value == self.board[row][col]:
                    self.markedBoard[row][col] = 1
                    self.lastMarkedNumber = value

    def getScore(self):
        score = 0
        for r, row in enumerate(self.markedBoard):
            for c, cell in enumerate(row):
                if cell == 0:
                    score += int(self.board[r][c])
        
        score = score * int(self.lastMarkedNumber)
        return score

    def checkWinCount(self):
        winCount = 0

        # Check rows
        for row in range(self.size):
            for col in range(self.size):
                if self.markedBoard[row][col] == 0:
                    break
                elif col == len(self.markedBoard) - 1:
                    # print('winning row!', row)
                    winCount += 1

        # Check columns
        for col in range(self.size):
            for row in range(self.size):
                if self.markedBoard[row][col] == 0:
                    break
                elif row == len(self.markedBoard) - 1:
                    # print('winning column!', col)
                    winCount += 1

        return winCount

def readInputFile(filePath):
    with open(filePath, 'r+') as f:
        numSequence = f.readline().rstrip().split(',')
        print(numSequence)

        # read in board data
        boards = []
        line = f.readline()
        while line:
            board = []
            for _ in range(5):
                line = f.readline()
                row = line.rstrip().split()
                board.append(row)

            boards.append(board)
            
            # Skip newline splitting boards
            line = f.readline()

        return (numSequence, boards)

def checkForWinner(nums, boards):
    lastWinningBoard = BingoBoard(5)
    for n in nums:
        winningIndexes = []
        for i, board in enumerate(boards):
            board.markValue(n)
            if board.checkWinCount() > 0:
                winningIndexes.append(i)
                lastWinningBoard = board

        for i in winningIndexes[::-1]:
            boards.pop(i)

        if len(boards) < 1:
            break

    return lastWinningBoard


if __name__ == '__main__':
    if len(sys.argv) != 2:
        print('Usage: ')
    else:
        puzzleInput = readInputFile(sys.argv[1])
        nums = puzzleInput[0]

        boards = []
        for board in puzzleInput[1]:
            boards.append(BingoBoard(5, board))

        lastWinningBoard = checkForWinner(nums, boards)

        for x in lastWinningBoard.board:
            print(x)
        for x in lastWinningBoard.markedBoard:
            print(x)

        print('last num:', lastWinningBoard.lastMarkedNumber)
        print('score:', lastWinningBoard.getScore())
