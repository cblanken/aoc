import sys

def readInputFile(filePath):
    with open(filePath, 'r+') as f:
        lines = f.read().splitlines()
        return lines

def getPowerConsumption(lines):
    gammaRate = b''
    epsilonRate = b''
    
    # Convert rows to columns to calculate gamma and epsilon rates
    columns = [[l[i] for l in lines] for i in range(len(lines[0]))]

    for c in columns:
        onesCount = 0
        for bit in c:
            if bit == '1':
                onesCount += 1
 
        if onesCount > len(c) / 2:
            gammaRate += b'1'
            epsilonRate += b'0'
        else:
            gammaRate += b'0'
            epsilonRate += b'1'

    print('gamma:', gammaRate, int(gammaRate, 2))
    print('epsilon:', epsilonRate, int(epsilonRate, 2))

    powerConsumption = int(gammaRate, 2) * int(epsilonRate, 2)
    return powerConsumption


if __name__ == '__main__':
    if len(sys.argv) != 2:
        print('Usage: ')
    else:
        lines = readInputFile(sys.argv[1])
        powerConsumption = getPowerConsumption(lines)
        print('Power Consumption: {0}'.format(powerConsumption))

        
