from collections import Counter, defaultdict

def expand_template(templ_counter, rules):
    out = Counter()
    for pair, count in templ_counter.items():
        if pair in rules:
            a, c = pair
            b = rules[pair]
            out[a+b] += count
            out[b+c] += count
        else:
            out[pair] += count
    return out


if __name__ == '__main__':
    with open('14.txt') as f:
        lines = f.readlines()
    template = lines[0].strip()
    rules = [tuple(l.strip().split(' -> ')) for l in lines[2:]]
    rules = {r[0]: r[1] for r in rules}

    templ_counter = Counter(''.join(pair) for pair in zip(template, template[1:]))

    for idx in range(40):
        templ_counter = expand_template(templ_counter, rules)
    
        char_counts = defaultdict(lambda: (0, 0))
        for (a, b), count in templ_counter.items():
            char_counts[a] = (char_counts[a][0] + count, char_counts[a][1])
            char_counts[b] = (char_counts[b][0], char_counts[b][1] + count)
        char_counts = [max(cnt) for cnt in char_counts.values()]
        diff = max(char_counts) - min(char_counts)
        print(idx, diff)
        if idx+1 == 10:
            print("Part 1: {}".format(diff))
        elif idx+1 == 40:
            print("Part 2: {}".format(diff))
