use itertools::iproduct;
use std::collections::HashSet;

type Coord = (i32, i32);
type Grid = Vec<Vec<i32>>;

fn get_basin_size(grid: &Grid, low_point: Coord) -> usize {
    let mut visited: HashSet<Coord> = HashSet::new();
    let mut to_visit: Vec<Coord> = vec![low_point];
    while let Some(curr) = to_visit.pop() {
        visited.insert(curr);

        let (cx, cy) = (curr.0 as i32, curr.1 as i32);
        let adjacent_coords = [(cx - 1, cy), (cx + 1, cy), (cx, cy - 1), (cx, cy + 1)]
            .into_iter()
            .filter(|&(nx, ny)| {
                nx >= 0 && ny >= 0 && nx < grid.len() as i32 && ny < grid[0].len() as i32
            });
        for (nx, ny) in adjacent_coords {
            if grid[nx as usize][ny as usize] == 9
                || grid[nx as usize][ny as usize] < grid[curr.0 as usize][curr.1 as usize]
            {
                continue;
            }
            if !visited.contains(&(nx, ny)) {
                to_visit.push((nx, ny));
            }
        }
    }
    visited.len()
}

fn main() {
    let input = include_str!("input.txt");

    let grid: Grid = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse::<_>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();

    let low_points = iproduct!((0..grid.len()), (0..grid[0].len()))
        .filter(|&(x, y)| {
            let (cx, cy) = (x as i32, y as i32);
            let mut adjacent_vals = [(cx - 1, cy), (cx + 1, cy), (cx, cy - 1), (cx, cy + 1)]
                .into_iter()
                .filter(|&(nx, ny)| {
                    nx >= 0 && ny >= 0 && nx < grid.len() as i32 && ny < grid[0].len() as i32
                })
                .map(|(nx, ny)| grid[nx as usize][ny as usize]);
            adjacent_vals.all(|val| grid[x as usize][y as usize] < val)
        })
        .map(|(x, y)| (x as i32, y as i32))
        .collect::<Vec<Coord>>();
    let part1: i32 = low_points
        .iter()
        .map(|&(x, y)| grid[x as usize][y as usize] + 1)
        .sum();
    println!("Part 1: {}", part1);

    let mut basin_sizes = low_points
        .iter()
        .map(|p| get_basin_size(&grid, *p))
        .collect::<Vec<_>>();
    basin_sizes.sort_unstable();
    println!(
        "Part 2: {}",
        basin_sizes
            .into_iter()
            .rev()
            .take(3)
            .reduce(|a, b| a * b)
            .unwrap()
    );
}
