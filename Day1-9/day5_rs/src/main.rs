use std::{cmp::max, cmp::min, collections::HashMap};

type Coord = (i32, i32);

fn parse_coord_pair(input: &str) -> Coord {
    let mut pieces = input.split(',');
    let x = pieces.next().unwrap().parse::<_>().unwrap();
    let y = pieces.next().unwrap().parse::<_>().unwrap();
    (x, y)
}

fn parse_input_line(line: &str) -> (Coord, Coord) {
    let mut pieces = line.split(" -> ");
    let start = parse_coord_pair(pieces.next().unwrap());
    let end = parse_coord_pair(pieces.next().unwrap());
    (start, end)
}

fn find_num_overlaps(lines: Vec<(Coord, Coord)>, consider_diagonals: bool) -> usize {
    let mut grid: HashMap<Coord, u32> = HashMap::new();
    let sort2 = |a, b| (min(a, b), max(a, b));
    for (start, end) in lines {
        let ((x1, y1), (x2, y2)) = (start, end);
        if x1 == x2 {
            let (y1, y2) = sort2(y1, y2);
            for y in y1..=y2 {
                let coord = (x1, y);
                grid.entry(coord).and_modify(|val| *val += 1).or_insert(1);
            }
        } else if y1 == y2 {
            let (x1, x2) = sort2(x1, x2);
            for x in x1..=x2 {
                let coord = (x, y1);
                grid.entry(coord).and_modify(|val| *val += 1).or_insert(1);
            }
        } else if consider_diagonals {
            let dy = if y2 > y1 { 1 } else { -1 };
            let dx = if x2 > x1 { 1 } else { -1 };
            let (mut x, mut y) = start;
            loop {
                grid.entry((x, y)).and_modify(|val| *val += 1).or_insert(1);
                if (x, y) == end {
                    break;
                }
                x += dx;
                y += dy;
            }
        }
    }
    grid.values().filter(|v| **v > 1).count()
}

fn main() {
    let input = include_str!("input.txt");

    let lines = input.lines().map(parse_input_line).collect::<Vec<_>>();

    println!("Part 1: {}", find_num_overlaps(lines.clone(), false));
    println!("Part 2: {}", find_num_overlaps(lines, true));
}
