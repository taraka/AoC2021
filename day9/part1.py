

def isMinimum(y, x, map):
    values = []

    if y > 0:
        values.append(map[y-1][x])
    if y+1 < len(map):
        values.append(map[y+1][x])
    if x > 0:
        values.append(map[y][x-1])
    if x+1 < len(map[0]):
        values.append(map[y][x+1])

    return min(values) > map[y][x]
        

with open("./input") as f:
		lines = f.readlines()
		map = [[int(x) for x in line if x.isdigit()] for line in lines]

risk = 0
for y, row in enumerate(map):
    for x, value in enumerate(row):
        if isMinimum(y, x, map):
            risk += (1 + value)

print(risk)