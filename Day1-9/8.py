from functools import reduce


def decode_entry(entry):
    in_signals, out_signals = entry
    in_signals = ["".join(sorted(list(e))) for e in in_signals]
    out_signals = ["".join(sorted(list(e))) for e in out_signals]

    def contains_all_chars(needle, haystack):
        return all(c in haystack for c in needle)

    mappings = dict()
    for i in in_signals:
        if len(i) == 2:
            mappings[1] = i
        elif len(i) == 4:
            mappings[4] = i
        elif len(i) == 3:
            mappings[7] = i
        elif len(i) == 7:
            mappings[8] = i
    for i in in_signals:
        if len(i) != 5:
            continue
        if contains_all_chars(mappings[1], i):
            mappings[3] = i
        elif contains_all_chars(set(mappings[4]) - set(mappings[1]), i):
            mappings[5] = i
        else:
            mappings[2] = i

    mappings[9] = "".join(sorted(list(set(mappings[1]) | set(mappings[5]))))
    mappings[6] = "".join(
        sorted(list((set(mappings[8]) - set(mappings[1])) | set(mappings[5])))
    )
    mappings[0] = next(i for i in in_signals if i not in mappings.values())
    assert len(mappings[0]) == 6

    rev_mappings = {v: k for k, v in mappings.items()}
    values = [rev_mappings[s] for s in out_signals]
    return values


if __name__ == "__main__":
    with open("8_test.txt") as f:
        entries = [
            map(lambda e: e.strip().split(" "), l.split("|")) for l in f.readlines()
        ]

    part1, part2 = 0, 0
    for e in entries:
        decoded_digits = decode_entry(e)
        part1 += sum(1 for v in decoded_digits if v in [1, 4, 7, 8])
        part2 += reduce(
            lambda acc, entry: acc + entry[1] * 10 ** entry[0],
            enumerate(decoded_digits[::-1]),
            0,
        )
    print("Part 1: {}".format(part1))
    print("Part 2: {}".format(part2))
