use ndarray::{Array2, Axis};
use std::collections::BTreeSet;

fn checked_neighbors(x: usize, y: usize, n: usize, m: usize) -> Vec<(usize, usize)> {
    let (i_x, i_y, i_n, i_m) = (x as isize, y as isize, n as isize, m as isize);
    let mut out = Vec::new();
    for n_x in i_x - 1..=i_x + 1 {
        if n_x < 0 || n_x >= i_n {
            continue;
        }
        for n_y in i_y - 1..=i_y + 1 {
            if n_y >= 0 && n_y < i_m && (i_x, i_y) != (n_x, n_y) {
                out.push((n_x as usize, n_y as usize));
            }
        }
    }
    out
}

fn iterate_grid(grid: &mut Array2<u32>) -> usize {
    let mut to_check = Vec::new();
    let mut flashed = BTreeSet::new();

    for ((x, y), val) in grid.indexed_iter_mut() {
        *val += 1;
        if *val > 9 {
            to_check.push((x, y));
        }
    }

    while let Some(el @ (x, y)) = to_check.pop() {
        if flashed.contains(&el) {
            continue;
        }
        flashed.insert(el);
        for neighbor in checked_neighbors(x, y, grid.shape()[0], grid.shape()[1]) {
            if !flashed.contains(&neighbor) {
                grid[neighbor] += 1;
                if grid[neighbor] > 9 {
                    to_check.push(neighbor)
                }
            }
        }
    }

    for coord in flashed.iter() {
        grid[*coord] = 0;
    }

    flashed.len()
}

fn main() {
    let input = include_str!("input.txt")
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut grid = Array2::zeros((input.len(), input[0].len()));
    for (i, mut row) in grid.axis_iter_mut(Axis(0)).enumerate() {
        for (j, col) in row.iter_mut().enumerate() {
            *col = input[i][j];
        }
    }

    let (mut part1, mut part2) = (0, 0);
    for idx in 0..1000 {
        let flashed = iterate_grid(&mut grid);
        if idx < 100 {
            part1 += flashed;
        }
        if flashed == grid.len() {
            part2 = idx + 1;
            break;
        }
    }
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
