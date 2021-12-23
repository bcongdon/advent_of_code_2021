from collections import namedtuple

Point = namedtuple('Point', ['x', 'y', 'z'])

Prism = namedtuple('Prism', ['lower', 'upper', 'status'])

def combine(p1, p2):
    # Cases to consider: p1=[], p2={}
    # 1: [...]   {...} # Disjoint
    # 2: [...{xxx]...} # Partial overlap
    # 3: {...[xxx}...] # Partial overlap
    # 4: [...{xxx}...] # Complete overlap
    # 5: {...[xxx]...} # Complete overlap
    # 6: {...}   [...] # Disjoint
    new_lower, new_upper = [0, 0, 0], [0, 0, 0]
    for dim in range(0, 3):
        if p1.lower[dim] > p2.upper[dim]:  # Case 1
            return None
        elif p1.upper[dim] < p2.lower[dim]:  # Case 2
            return None
        elif p1.lower[dim] <= p2.lower[dim] and p1.upper[dim] <= p2.upper[dim]:  # Case 2
            new_lower[dim] = p2.lower[dim]
            new_upper[dim] = p1.upper[dim]
        elif p2.lower[dim] <= p1.lower[dim] and p2.upper[dim] <= p1.upper[dim]:  # Case 3
            new_lower[dim] = p1.lower[dim]
            new_upper[dim] = p2.upper[dim]
        elif p1.lower[dim] <= p2.lower[dim] and p2.upper[dim] <= p1.upper[dim]:  # Case 4:
            new_lower[dim] = p2.lower[dim]
            new_upper[dim] = p2.upper[dim]
        elif p2.lower[dim] <= p1.lower[dim] and p1.upper[dim] <= p2.upper[dim]:  # Case 5:
            new_lower[dim] = p1.lower[dim]
            new_upper[dim] = p1.upper[dim]
        else:
            raise Exception("unreachable")
    return Prism(Point(*new_lower), Point(*new_upper), True)

def subtract(p1, p2):
    if p1.lower == p2.lower and p1.upper == p2.upper:
        return []

    out = []
    # Cuboid above (Z) the intersection
    if p1.upper[2] != p2.upper[2]:
        lower = (p1.lower[0], p1.lower[1], p2.upper[2]+1)
        upper = (p1.upper[0], p1.upper[1], p1.upper[2])
        out.append(Prism(Point(*lower), Point(*upper), True))
    # Cuboid below (Z) the intersection
    if p1.lower[2] != p2.lower[2]:
        lower = (p1.lower[0], p1.lower[1], p1.lower[2])
        upper = (p1.upper[0], p1.upper[1], p2.lower[2]-1)
        out.append(Prism(Point(*lower), Point(*upper), True))
    # Y-lower cuboid in the Z intersection
    if p1.lower[1] != p2.lower[1]:
        lower = (p1.lower[0], p1.lower[1], p2.lower[2])
        upper = (p1.upper[0], p2.lower[1]-1, p2.upper[2])
        out.append(Prism(Point(*lower), Point(*upper), True))
    # Y-upper cuboid in the Z intersection
    if p1.upper[1] != p2.upper[1]:
        lower = (p1.lower[0], p2.upper[1]+1, p2.lower[2])
        upper = (p1.upper[0], p1.upper[1], p2.upper[2])
        out.append(Prism(Point(*lower), Point(*upper), True))
    # X-lower cuboid in the Z intersection
    if p1.lower[0] != p2.lower[0]:
        lower = (p1.lower[0], p2.lower[1], p2.lower[2])
        upper = (p2.lower[0]-1, p2.upper[1], p2.upper[2])
        out.append(Prism(Point(*lower), Point(*upper), True))
    # X-upper cuboid in the Z intersection
    if p1.upper[0] != p2.upper [0]:
        lower = (p2.upper[0]+1, p2.lower[1], p2.lower[2])
        upper = (p1.upper[0], p2.upper[1], p2.upper[2])
        out.append(Prism(Point(*lower), Point(*upper), True))
    return out
    

def parse_line(line):
    status, rest = line.split(' ')
    
    def parse_range(r):
        _min, _max = r[2:].split('..')
        return (int(_min), int(_max))
    
    rest = rest.strip().split(',')
    x0, x1 = parse_range(rest[0])
    y0, y1 = parse_range(rest[1])
    z0, z1 = parse_range(rest[2])
    return Prism(Point(x0, y0, z0), Point(x1, y1, z1), status=(status == "on"))

def calc_volume(prism):
    out = 1
    for dim in range(0, 3):
        out *= prism.upper[dim] - prism.lower[dim] + 1
    return out

if __name__ == "__main__":
    with open('22.txt') as f:
        lines = f.readlines()

    prisms = [parse_line(l) for l in lines]
    on_prisms = set()
    part1_done = False
    for p in prisms:
        if abs(p.upper[0]) > 50 and not part1_done:
            part1_done = True
            print("Part 1: {}".format(sum(calc_volume(p) for p in on_prisms)))
        new_prisms = []
        if p.status:
            new_prisms.append(p)
        for on_prism in list(on_prisms):
            overlap = combine(p, on_prism)
            if not overlap:
                continue
            on_prisms.remove(on_prism)
            new_prisms.extend(subtract(on_prism, overlap))
        for new_prism in new_prisms:
            on_prisms.add(new_prism)
    print("Part 2: {}".format(sum(calc_volume(p) for p in on_prisms)))