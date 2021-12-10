fn illegal_char_score(c: char) -> usize {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("invalid illegal character"),
    }
}

fn corresponding_char(c: char) -> char {
    match c {
        ')' => '(',
        ']' => '[',
        '}' => '{',
        '>' => '<',
        _ => panic!("illegal character"),
    }
}

enum Report {
    Corrupted(usize),
    Incomplete(usize),
}

fn check_line(line: &str) -> Report {
    let mut stack = vec![];
    for c in line.chars() {
        match c {
            '(' | '{' | '[' | '<' => {
                stack.push(c);
            }
            ')' | '}' | ']' | '>' => {
                let expected = stack.pop().unwrap();
                if corresponding_char(c) != expected {
                    return Report::Corrupted(illegal_char_score(c));
                }
            }
            _ => panic!("invalid character: {}", c),
        }
    }
    let completion_chars = stack.into_iter().rev();
    let mut completion_score = 0;
    for c in completion_chars {
        completion_score *= 5;
        completion_score += match c {
            '(' => 1,
            '[' => 2,
            '{' => 3,
            '<' => 4,
            _ => panic!("invalid character"),
        };
    }
    Report::Incomplete(completion_score)
}

fn main() {
    let lines = include_str!("input.txt").lines();
    let reports = lines.into_iter().map(check_line);

    let mut part1 = 0;
    let mut part2 = vec![];
    for r in reports {
        match r {
            Report::Corrupted(v) => part1 += v,
            Report::Incomplete(v) => part2.push(v),
        }
    }
    println!("Part 1: {}", part1);

    part2.sort_unstable();
    let part2 = part2[part2.len() / 2];
    println!("Part 2: {}", part2);
}
