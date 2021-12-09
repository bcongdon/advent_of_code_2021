from itertools import product
from functools import reduce


def get_basin_size(grid, low_point):
    visited = set()
    to_visit = [low_point]
    while to_visit:
        curr = to_visit.pop()
        visited.add(curr)

        x, y = curr
        adjacent = ((x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1))
        for nx, ny in adjacent:
            if not (0 <= nx < len(grid)) or not (0 <= ny < len(grid[0])):
                continue
            if grid[nx][ny] == 9 or grid[nx][ny] < grid[x][y]:
                continue
            if (nx, ny) not in visited:
                to_visit.append((nx, ny))
    return len(visited)


if __name__ == "__main__":
    with open("9.txt") as f:
        grid = [[int(i) for i in l.strip()] for l in f.readlines()]

    low_points = []
    for x, y in product(range(len(grid)), range(len(grid[0]))):
        adjacent = ((x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1))

        vals = []
        for nx, ny in adjacent:
            if not (0 <= nx < len(grid)) or not (0 <= ny < len(grid[0])):
                continue
            vals.append(grid[nx][ny])
        if all(grid[x][y] < v for v in vals):
            low_points.append((x, y))
    part1 = sum(grid[x][y] + 1 for x, y in low_points)
    print("Part 1: {}".format(part1))

    basin_sizes = sorted([get_basin_size(grid, low_point) for low_point in low_points])
    part2 = reduce(lambda a, b: a * b, basin_sizes[-3:])
    print("Part 2: {}".format(part2))
