#!/usr/bin/env python3
# -*- coding: utf-8 -*-

import os
import itertools

day_labels = [
    ":santa:",
    ":star2:",
    ":snowflake:",
    ":snowman:",
    ":sparkles:",
    ":fire:",
    ":christmas_tree:",
    ":gift:",
    ":bell:",
    ":tada:",
]

folders = [(10, "Day1-9"), (20, "Day10-19"), (26, "Day20-25")]

extensions = {
    "py": "Python",
    "s": "MIPS",
    "hs": "Haskell",
    "cpp": "C++",
    "c": "C",
    "rb": "Ruby",
    "swift": "Swift",
    "java": "Java",
    "js": "Javascript",
    "go": "Go",
    "rs": "Rust",
    "fs": "F#",
}

TEMPLATE = """# advent_of_code_2020
🎅 My solutions to the 2020 "Advent of Code"


## Solutions

{}"""


def split_entry(entry):
    entry = os.path.basename(entry)
    if "." not in entry:
        name, ext = entry.split("_")
    else:
        name, ext = os.path.splitext(entry)
        ext = ext[1:]
    day = next(int(name[i:]) for i in range(len(name)) if name[i:].isdigit())
    return (day, ext)


def files_for_day(day):
    fdr = next(x[1] for x in folders if x[0] > i)
    if not os.path.isdir(fdr):
        return []

    files = []
    for fn in os.listdir(fdr):
        try:
            day, ext = split_entry(fn)
        except:
            continue
        if ext in extensions and day == i:
            files.append(os.path.join(fdr, fn))
    return sorted(files)


def pretty_extension_name(path):
    _, ext = split_entry(path)
    return extensions[ext]


if __name__ == "__main__":
    out = ""
    soln_temp = "    * [{}]({})\n"
    symbols = itertools.cycle(list(day_labels))
    for i in range(1, 26):
        header = "* Day {}:  {}\n".format(i, next(symbols))
        solution_files = files_for_day(i)

        tmp = ""
        for s in solution_files:
            sol_type = pretty_extension_name(s)
            tmp += soln_temp.format(sol_type, s)

        if tmp != "":
            out += header
            out += tmp
    print(TEMPLATE.format(out))
