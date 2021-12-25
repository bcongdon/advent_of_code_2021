use eyre::{eyre, Result};
use std::str::FromStr;

#[derive(Debug)]
enum Direction {
    Forward,
    Down,
    Up,
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    amount: i32,
}

impl FromStr for Instruction {
    type Err = eyre::Error;

    fn from_str(s: &str) -> eyre::Result<Instruction> {
        let mut pieces = s.splitn(2, ' ');
        let direction = match pieces.next() {
            Some("forward") => Ok(Direction::Forward),
            Some("down") => Ok(Direction::Down),
            Some("up") => Ok(Direction::Up),
            Some(unknown) => Err(eyre!("unknown dir: {}", unknown)),
            _ => Err(eyre!("no direction")),
        }?;
        let amount = pieces
            .next()
            .ok_or_else(|| eyre!("no amount"))?
            .parse::<i32>()?;
        Ok(Instruction { direction, amount })
    }
}

fn main() -> eyre::Result<()> {
    let input = include_str!("input.txt");
    let instructions = input
        .lines()
        .map(|line| Instruction::from_str(line))
        .collect::<Result<Vec<_>>>()?;

    let (mut pos, mut depth) = (0, 0);
    for Instruction { direction, amount } in instructions.iter() {
        match direction {
            Direction::Down => depth += amount,
            Direction::Up => depth -= amount,
            Direction::Forward => pos += amount,
        }
    }
    println!("Part 1: {}", pos * depth);

    let (mut pos, mut depth, mut aim) = (0, 0, 0);
    for Instruction { direction, amount } in instructions {
        match direction {
            Direction::Down => {
                aim += amount;
            }
            Direction::Up => {
                aim -= amount;
            }
            Direction::Forward => {
                pos += amount;
                depth += aim * amount;
            }
        }
    }
    println!("Part 2: {}", pos * depth);
    Ok(())
}
