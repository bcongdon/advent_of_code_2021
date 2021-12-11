from itertools import product

def iterate_grid(grid):
    to_check = []
    flashed = set()
    for x, y in product(range(0, len(grid)), range(0, len(grid[0]))):
        grid[x][y] += 1
        if grid[x][y] > 9:
            to_check.append((x, y))
    while to_check:
        x, y = to_check.pop()
        if (x, y) in flashed:
            continue
        flashed.add((x, y))
        for nx in range(x-1, x+2):
            for ny in range(y-1, y+2):
                if (x, y) != (nx, ny) and 0 <= nx < len(grid) and 0 <= ny < len(grid[0]) and (nx, ny) not in flashed:
                    grid[nx][ny] += 1
                    if grid[nx][ny] > 9:
                        to_check.append((nx, ny))
    for x, y in flashed:
        grid[x][y] = 0
    return grid, len(flashed)
    

if __name__ == '__main__':
    with open('11.txt') as f:
        grid = [[int(i) for i in l.strip()] for l in f.readlines()]
    flashes = 0
    part2 = -1
    for idx in range(1000):
        grid, flashed = iterate_grid(grid)
        if idx < 100:
            flashes += flashed
        if flashed == (len(grid) * len(grid[0])):
            part2 = idx+1
            break
    print("Part 1: {}".format(flashes))
    print("Part 2: {}".format(part2))