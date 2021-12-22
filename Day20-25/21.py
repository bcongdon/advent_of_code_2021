from functools import lru_cache
 
def wrap_around_10(val):
    val = val % 10
    if val == 0:
        val = 10
    return val

def part1(p1_pos, p2_pos):
    num_dice = 0
    d = 1
    p1_score, p2_score = (0, 0)
    while p1_score < 1000 and p2_score < 1000:
        p1_pos += (d+d+1+d+2)
        p1_pos = wrap_around_10(p1_pos)
        p1_score += p1_pos
        d += 3
        num_dice += 3
        
        if p1_score >= 1000:
            break

        p2_pos += (d+d+1+d+2)
        p2_pos = wrap_around_10(p2_pos)
        p2_score += p2_pos
        d += 3
        num_dice += 3
    return min(p1_score, p2_score) * num_dice

dice_frequency = {
    3: 1,
    4: 3,
    5: 6,
    6: 7,
    7: 6,
    8: 3,
    9: 1,
}

@lru_cache
def part2(p1_pos_init, p2_pos_init, p1_score_init=0, p2_score_init=0, p1_turn=True):
    num_wins = 0
    for die_result, freq in dice_frequency.items():
        p1_pos, p2_pos = p1_pos_init, p2_pos_init
        p1_score, p2_score = p1_score_init, p2_score_init
        if p1_turn:
            p1_pos += die_result
            p1_pos = wrap_around_10(p1_pos)
            p1_score += p1_pos
            if p1_score >= 21:
                num_wins += freq
            else:
                num_wins += freq * part2(p1_pos, p2_pos, p1_score, p2_score, p1_turn=False)
        else:
            p2_pos += die_result
            p2_pos = wrap_around_10(p2_pos)
            p2_score += p2_pos
            if p2_score >= 21:
                continue
            else:
                num_wins += freq * part2(p1_pos, p2_pos, p1_score, p2_score, p1_turn=True)
    return num_wins


if __name__ == '__main__':
    p1_pos = 10
    p2_pos = 2

    print("Part 1: {}".format(part1(p1_pos, p2_pos)))
    print("Part 2: {}".format(part2(p1_pos, p2_pos)))