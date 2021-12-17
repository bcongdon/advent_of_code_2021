from itertools import product

def check_if_lands_in_target(velocity, target_x, target_y):
    tx_min, tx_max = target_x
    ty_min, ty_max = target_y
    vx, vy = velocity

    x, y, max_y = 0, 0, -10**10
    while x <= tx_max and y >= ty_min:
        if tx_min <= x <= tx_max and ty_min <= y <= ty_max:
            return True, max_y
        x += vx
        y += vy
        max_y = max(max_y, y)
        if vx > 0:
            vx -= 1
        elif vx < 0:
            vx += 1
        vy -= 1
    return False, 0

if __name__ == '__main__':
    target_x = (25, 67)
    target_y = (-260, -200)

    part1 = -10**10
    part2 = 0
    for vx, vy in product(range(0, target_x[1]+1), range(target_y[0]-1, -target_y[0])):
        lands, max_y = check_if_lands_in_target((vx, vy), target_x, target_y)
        if not lands:
            continue
        part1 = max(max_y, part1)
        part2 += 1
    print("Part 1: {}".format(part1))
    print("Part 2: {}".format(part2))