const MAX_COUNTER: usize = 8;
const PART_1_DAYS: usize = 80;
const PART_2_DAYS: usize = 256;

fn iterate_fish(counters: &mut [usize; MAX_COUNTER + 1]) {
    let num_reproduced = counters[0];
    for idx in 0..MAX_COUNTER {
        counters[idx] = counters[idx + 1];
    }
    counters[MAX_COUNTER] = num_reproduced;
    counters[MAX_COUNTER - 2] += num_reproduced;
}

fn total_fish(starting: &[usize; MAX_COUNTER + 1], num_days: usize) -> usize {
    let mut counters = starting.clone();
    for _ in 0..num_days {
        iterate_fish(&mut counters);
    }
    counters.iter().sum()
}

fn main() {
    let input = include_str!("input.txt");
    let initial_fish = input
        .split(',')
        .map(|n| n.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let mut counters = [0; MAX_COUNTER + 1];
    for i in initial_fish {
        counters[i] += 1;
    }

    println!("Part 1: {}", total_fish(&counters, PART_1_DAYS));
    println!("Part 2: {}", total_fish(&counters, PART_2_DAYS));
}
