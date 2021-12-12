from collections import defaultdict
from functools import lru_cache

if __name__ == '__main__':
    with open('12.txt') as f:
        lines = f.readlines()

    edges = defaultdict(list)
    for l in lines:
        a, b = l.strip().split('-')
        edges[a].append(b)
        edges[b].append(a)
    
    paths = 0
    @lru_cache()
    def dfs(node, path, allow_twice):
        if node == 'end':
            return 1
        child_paths = 0
        for neighbor in edges[node]:
            if neighbor == 'start':
                continue

            is_small_cave = neighbor.lower() == neighbor
            if is_small_cave and neighbor in path and not allow_twice:
                continue
            if allow_twice and is_small_cave and neighbor in path:
                new_path = tuple(set(path) | set([neighbor]))
                child_paths += dfs(neighbor, new_path, False)
            else:
                new_path = tuple(set(path) | set([neighbor]))
                child_paths += dfs(neighbor, new_path, allow_twice)
        return child_paths
    
    print("Part 1: {}".format(dfs('start', (), False)))
    print("Part 2: {}".format(dfs('start', (), True)))