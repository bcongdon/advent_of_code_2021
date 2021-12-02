fn main() {
    let input = include_str!("input.txt");

    let nums = input
        .lines()
        .map(|l| l.parse::<i32>().expect("line number"))
        .collect::<Vec<_>>();

    let mut num_increasing = 0;
    for idx in 0..(nums.len() - 1) {
        if nums[idx] < nums[idx + 1] {
            num_increasing += 1;
        }
    }
    println!("Part 1: {}", num_increasing);

    let mut prev = nums[0] + nums[1] + nums[2];
    let mut num_increasing = 0;
    for idx in 3..nums.len() {
        let curr = prev + nums[idx] - nums[idx - 3];
        if curr > prev {
            num_increasing += 1;
        }
        prev = curr;
    }
    println!("Part 2: {}", num_increasing);
}
