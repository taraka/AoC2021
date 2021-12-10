

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

def basinSize(y, x, visited, map):
    if map[y][x] != 9:
        visited.append((y,x))
        if y > 0 and not (y-1,x) in visited:
            basinSize(y-1, x, visited, map)
        if y+1 < len(map) and not (y+1,x) in visited:
            basinSize(y+1, x, visited, map)
        if x > 0 and not (y,x-1) in visited:
            basinSize(y, x-1, visited, map)
        if x+1 < len(map[0]) and not (y,x+1) in visited:
            basinSize(y, x+1, visited, map)

with open("./input") as f:
		lines = f.readlines()
		map = [[int(x) for x in line if x.isdigit()] for line in lines]

lows = []
for y, row in enumerate(map):
    for x, value in enumerate(row):
        if isMinimum(y, x, map):
            lows.append((y, x))

totals = []

for y, x in lows:
    visited = []
    basinSize(y, x, visited, map)
    totals.append(len(visited))

totals.sort(reverse=True)
total = 1
for i in totals[:3]:
    total *= i
print(total)