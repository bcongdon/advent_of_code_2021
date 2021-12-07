type CostFn = fn(i32, i32) -> i32;

fn calculate_score(positions: &[i32], target: i32, cost_fn: CostFn) -> i32 {
    positions.iter().map(|p| cost_fn(*p, target)).sum()
}

fn main() {
    let initial_positions = include_str!("input.txt")
        .split(',')
        .map(|p| p.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let min = initial_positions.iter().min().unwrap();
    let max = initial_positions.iter().max().unwrap();

    let part1_cost: CostFn = |a, b| (a - b).abs();
    let part1 = (*min..=*max)
        .map(|target| calculate_score(&initial_positions, target, part1_cost))
        .min()
        .unwrap();
    println!("Part 1: {}", part1);

    let part2_cost: CostFn = |a, b| {
        let diff = (a - b).abs();
        diff * (diff + 1) / 2
    };
    let part2 = (*min..=*max)
        .map(|target| calculate_score(&initial_positions, target, part2_cost))
        .min()
        .unwrap();
    println!("Part 2: {}", part2);
}
