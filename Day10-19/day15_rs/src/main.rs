use ndarray::{Array2, Axis};
use std::{
    cmp::Ordering,
    collections::{BTreeMap, BinaryHeap},
};

fn grid_cost(grid: &Array2<u32>, x: usize, y: usize) -> u32 {
    let (mx, my) = (x / grid.shape()[0], y / grid.shape()[0]);
    let base_cost = grid[(x % grid.shape()[0], y % grid.shape()[1])];
    let new_cost = base_cost + mx as u32 + my as u32;
    if new_cost > 9 {
        new_cost - 9
    } else {
        new_cost
    }
}

fn checked_neighbors(x: usize, y: usize, n: usize, m: usize) -> Vec<(usize, usize)> {
    let (i_x, i_y, i_n, i_m) = (x as isize, y as isize, n as isize, m as isize);
    vec![
        (i_x + 1, i_y),
        (i_x - 1, i_y),
        (i_x, i_y + 1),
        (i_x, i_y - 1),
    ]
    .into_iter()
    .filter(|&(n_x, n_y)| {
        n_x >= 0 && n_x < i_n && n_y >= 0 && n_y < i_m && (i_x, i_y) != (n_x, n_y)
    })
    .map(|(n_x, n_y)| (n_x as usize, n_y as usize))
    .collect()
}

#[derive(PartialEq, Debug)]
struct Min(u32);

impl Eq for Min {}

impl PartialOrd for Min {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.0.partial_cmp(&self.0)
    }
}

impl Ord for Min {
    fn cmp(&self, other: &Min) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
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

    let (n, m) = (input.len(), input[0].len());
    let mut grid = Array2::zeros((n, m));
    for (i, mut row) in grid.axis_iter_mut(Axis(0)).enumerate() {
        for (j, col) in row.iter_mut().enumerate() {
            *col = input[i][j];
        }
    }

    let mut queue: BinaryHeap<(Min, (usize, usize))> = BinaryHeap::new();
    queue.push((Min(0), (0, 0)));
    let mut costs: BTreeMap<(usize, usize), u32> = BTreeMap::new();
    costs.insert((0, 0), 0);

    while let Some((cost, (x, y))) = queue.pop() {
        for (nx, ny) in checked_neighbors(x, y, 5 * n, 5 * m) {
            let new_cost = cost.0 + grid_cost(&grid, nx, ny);
            let cost_entry = costs.entry((nx, ny)).or_insert(u32::MAX);
            if new_cost < *cost_entry {
                *cost_entry = new_cost;
                queue.push((Min(new_cost), (nx, ny)));
            }
        }
    }
    println!("Part 1: {}", costs[&(n - 1, m - 1)]);
    println!("Part 2: {}", costs[&(5 * n - 1, 5 * m - 1)]);
}
