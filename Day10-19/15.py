from collections import defaultdict
import heapq

if __name__ == "__main__":
    with open('15.txt') as f:
        grid = [[int(i) for i in l.strip()] for l in f.readlines()]
    
    costs = dict()
    costs[(0, 0)] = 0
    pqueue = [(0, (0, 0))]

    def grid_cost(x, y):
        mx, my = x/len(grid), y/len(grid[0])
        base_cost = grid[x%len(grid)][y%len(grid[0])]
        new_cost = base_cost + mx + my
        if new_cost > 9:
            new_cost -= 9
        return new_cost

    while pqueue:
        (cost, (x, y)) = heapq.heappop(pqueue)
        for nx, ny in [(x+1, y), (x-1, y), (x, y+1), (x, y-1)]:
            if not (0 <= nx < 5*len(grid) and 0 <= ny < 5*len(grid[0])):
                continue

            new_cost = cost + grid_cost(nx, ny)
            if (nx, ny) not in costs or new_cost < costs[(nx, ny)]:
                costs[nx, ny] = new_cost
                heapq.heappush(pqueue, (new_cost, (nx, ny)))
    print("Part 1: {}".format(costs[(len(grid)-1, len(grid[0])-1)]))
    print("Part 2: {}".format(costs[(5*len(grid)-1, 5*len(grid[0])-1)]))