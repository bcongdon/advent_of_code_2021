def parse_coord(s):
    x, y = s.strip().split(',')
    return (int(x), int(y))

def parse_inst(s):
    inst = s.strip().split(' ')[-1]
    axis, loc = inst.split('=')
    return (axis, int(loc))

def make_fold(points, inst):
    axis, loc = inst
    out = set()
    for x, y in points:
        if axis == 'x' and x > loc:
            x = loc - (x - loc)
        elif axis == 'y' and y > loc:
            y = loc - (y - loc)
        out.add((x, y))
    return out

if __name__ == '__main__':
    with open('13.txt') as f:
        coords_raw, instructions_raw = f.read().split('\n\n')
    coords = set(parse_coord(l) for l in coords_raw.split() if l.strip())
    insts = [parse_inst(l) for l in instructions_raw.split('\n') if l.strip()]

    part1 = make_fold(coords, insts[0])
    print("Part 1: {}".format(len(part1)))

    part2 = coords
    for i in insts:
        part2 = make_fold(part2, i)
    
    min_x, min_y = min(x for x, _ in part2), min(y for _, y in part2)
    max_x, max_y = max(x for x, _ in part2), max(y for _, y in part2)

    print("Part 2:")
    for y in range(min_y, max_y+1):
        l = ''
        for x in range(min_x, max_x+1):
            if (x, y) in part2:
                l += '*'
            else:
                l += ' '
        print(l)
