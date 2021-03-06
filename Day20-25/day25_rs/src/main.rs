use std::fmt::Display;

use ndarray::{Array2, Axis};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
    Empty,
    SouthCucumber,
    EastCucumber,
}

impl Default for Tile {
    fn default() -> Self {
        Self::Empty
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match *self {
            Self::Empty => '.',
            Self::EastCucumber => '>',
            Self::SouthCucumber => 'v',
        };
        write!(f, "{}", c)
    }
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '>' => Self::EastCucumber,
            'v' => Self::SouthCucumber,
            '.' => Self::Empty,
            _ => panic!("unknown tile"),
        }
    }
}

fn print_grid(grid: &Array2<Tile>) {
    for y in 0..grid.shape()[1] {
        for x in 0..grid.shape()[0] {
            print!("{}", grid[(x, y)]);
        }
        println!();
    }
    println!();
}

fn iterate_grid(grid: &Array2<Tile>) -> (Array2<Tile>, bool) {
    let mut out = grid.clone();
    let mut moved = false;

    let movement_directions = vec![(Tile::EastCucumber, (1, 0)), (Tile::SouthCucumber, (0, 1))];
    for (tile, (dx, dy)) in movement_directions {
        let moves = out
            .indexed_iter()
            .filter(|(_, t)| **t == tile)
            .map(|((x, y), _)| {
                let nx = (x + dx) % out.shape()[0];
                let ny = (y + dy) % out.shape()[1];
                if out[(nx, ny)] == Tile::Empty {
                    ((x, y), (nx, ny))
                } else {
                    ((x, y), (x, y))
                }
            })
            .collect::<Vec<_>>();
        for (p1 @ (x, y), p2 @ (nx, ny)) in moves {
            if p1 == p2 {
                continue;
            }
            moved = true;
            out[(x, y)] = Tile::Empty;
            out[(nx, ny)] = tile;
        }
    }

    (out, moved)
}

fn main() {
    let input = include_str!("input.txt")
        .lines()
        .map(|line| line.trim().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut grid = Array2::<Tile>::default((input[0].len(), input.len()));
    for (i, mut row) in grid.axis_iter_mut(Axis(0)).enumerate() {
        for (j, col) in row.iter_mut().enumerate() {
            *col = input[j][i].into();
        }
    }

    for i in 0.. {
        let (new_grid, moved) = iterate_grid(&grid);
        if !moved {
            println!("Part 1: {}", i + 1);
            break;
        }
        grid = new_grid;
    }
}
