use itertools::iproduct;
use ndarray::{Array2, Axis};

fn expand_image(
    grid: Array2<bool>,
    enhancement_lookup: &[bool],
    background_lit: bool,
) -> Array2<bool> {
    let mut out = Array2::<bool>::default((grid.shape()[0] + 2, grid.shape()[1] + 2));
    for (x, y) in iproduct![(0..out.shape()[0] as i32), (0..out.shape()[1] as i32)] {
        let mut enhancement_idx = 0;
        let mut i = 0;
        for ny in y - 1..=y + 1 {
            for nx in x - 1..=x + 1 {
                let is_currently_on = if nx < 1
                    || ny < 1
                    || (nx - 1) >= grid.shape()[0] as i32
                    || (ny - 1) >= grid.shape()[1] as i32
                {
                    background_lit
                } else {
                    grid[((nx - 1) as usize, (ny - 1) as usize)]
                };
                if is_currently_on {
                    enhancement_idx += 2_u32.pow(8 - i as u32);
                }
                i += 1;
            }
        }
        out[(x as usize, y as usize)] = enhancement_lookup[enhancement_idx as usize];
    }
    out
}
fn main() {
    let input = include_str!("input.txt").split("\n\n").collect::<Vec<_>>();
    let enhancement = input[0].chars().map(|c| c == '#').collect::<Vec<_>>();

    let grid_input = input[1].lines().map(|l| l.trim()).collect::<Vec<_>>();
    let mut grid = Array2::<bool>::default((grid_input[0].len(), grid_input.len()));
    for (j, mut row) in grid.axis_iter_mut(Axis(0)).enumerate() {
        for (i, col) in row.iter_mut().enumerate() {
            *col = grid_input[i].chars().nth(j).unwrap() == '#';
        }
    }

    let count_enabled = |grid: &Array2<bool>| grid.iter().filter(|v| **v).count();

    for i in 0..50 {
        grid = expand_image(grid, &enhancement, i % 2 == 1);

        if i + 1 == 2 {
            println!("Part 1: {}", count_enabled(&grid));
        } else if i + 2 == 50 {
            println!("Part 2: {}", count_enabled(&grid));
        }
    }
}
