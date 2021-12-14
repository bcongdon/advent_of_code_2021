use std::collections::HashSet;

type Point = (i32, i32);
type Instruction = (char, i32);

fn parse_coord(s: &str) -> Point {
    let mut pieces = s.trim().split(',').map(|p| p.parse::<i32>().unwrap());
    (pieces.next().unwrap(), pieces.next().unwrap())
}

fn parse_instruction(s: &str) -> Instruction {
    let instruction = s.split(' ').last().unwrap();
    let mut pieces = instruction.split('=');
    (
        pieces.next().unwrap().chars().next().unwrap(),
        pieces.next().unwrap().parse::<i32>().unwrap(),
    )
}

fn perform_fold(points: &mut HashSet<Point>, instruction: Instruction) {
    let (axis, loc) = instruction;

    let curr_points = points.iter().cloned().collect::<Vec<_>>();
    for el @ (x, y) in curr_points {
        if axis == 'x' && x > loc {
            points.remove(&el);
            points.insert((loc - (x - loc), y));
        } else if axis == 'y' && y > loc {
            points.remove(&el);
            points.insert((x, loc - (y - loc)));
        }
    }
}

fn main() {
    let mut input = include_str!("input.txt").split("\n\n");
    let points = input.next().unwrap();
    let mut points = points.lines().map(parse_coord).collect::<HashSet<_>>();

    let instructions = input
        .next()
        .unwrap()
        .lines()
        .map(parse_instruction)
        .collect::<Vec<_>>();

    for (idx, &instruction) in instructions.iter().enumerate() {
        perform_fold(&mut points, instruction);
        if idx == 0 {
            println!("Part 1: {}", points.len());
        }
    }

    let x_min = *points.iter().map(|(x, _)| x).min().unwrap();
    let x_max = *points.iter().map(|(x, _)| x).max().unwrap();
    let y_min = *points.iter().map(|(_, y)| y).min().unwrap();
    let y_max = *points.iter().map(|(_, y)| y).max().unwrap();

    println!("Part 2:");
    for y in y_min..=y_max {
        let mut l = String::new();
        for x in x_min..=x_max {
            l += if points.contains(&(x, y)) { "*" } else { " " };
        }
        println!("{}", l);
    }
}
