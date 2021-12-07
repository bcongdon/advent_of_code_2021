MAX_COUNTER = 8
PART_1_DAYS = 80
PART_2_DAYS = 256

def iterate_fish(counters):
    num_reproduced = counters[0]
    counters = counters[1:] + [num_reproduced]
    counters[-3] += num_reproduced
    return counters


if __name__ == '__main__':
    with open('6.txt') as f:
        initial_fish = [int(i) for i in f.read().split(',')]
    
    counters = [0 for _ in range(MAX_COUNTER+1)]
    for f in initial_fish:
        counters[f] += 1

    part1_counters = list(counters)
    for _ in range(PART_1_DAYS):
        part1_counters = iterate_fish(part1_counters)
    print("Part 1: {}".format(sum(part1_counters)))

    part2_counters = list(counters)
    for _ in range(PART_2_DAYS):
        part2_counters = iterate_fish(part2_counters)
    print("Part 2: {}".format(sum(part2_counters)))