fn main() {
    let input = include_str!("input.txt");

    let lines = input.lines().collect::<Vec<_>>();
    let binary_len = lines[0].len();

    let ones = ones_frequencies(&lines);
    let mut gamma = 0;
    let mut epsilon = 0;
    for i in 0..binary_len {
        let pow = 2_i32.pow((binary_len - i - 1) as u32);
        if ones[i] > lines.len() / 2 {
            gamma += pow;
        } else {
            epsilon += pow;
        }
        assert!(ones[i] != lines.len() / 2);
    }
    println!("Part 1: {}", gamma * epsilon);

    let mut oxygen_candidates = lines.iter().copied().collect::<Vec<_>>();
    let mut co2_candidates = lines.iter().copied().collect::<Vec<_>>();
    for i in 0..binary_len {
        if oxygen_candidates.len() > 1 {
            let wanted = match most_common_binary_digit(&oxygen_candidates, i) {
                Some(1) | None => '1',
                Some(0) => '0',
                Some(_) => unreachable!(),
            };
            oxygen_candidates = oxygen_candidates
                .into_iter()
                .filter(|l| l.chars().nth(i).unwrap() == wanted)
                .collect();
        }
        if co2_candidates.len() > 1 {
            let wanted = match most_common_binary_digit(&co2_candidates, i) {
                Some(1) | None => '0',
                Some(0) => '1',
                Some(_) => unreachable!(),
            };
            co2_candidates = co2_candidates
                .into_iter()
                .filter(|l| l.chars().nth(i).unwrap() == wanted)
                .collect();
        }
    }
    let oxygen = binary_to_decimal(oxygen_candidates[0]);
    let co2 = binary_to_decimal(co2_candidates[0]);
    println!("Part 2: {}", oxygen * co2);
}

fn binary_to_decimal(binary: &str) -> i32 {
    let mut result = 0;
    let len = binary.len();
    for (idx, c) in binary.chars().enumerate() {
        result += 2_i32.pow((len - idx - 1) as u32) * if c == '1' { 1 } else { 0 };
    }
    result
}

fn ones_frequencies(nums: &[&str]) -> Vec<usize> {
    let mut ones = vec![0; nums.len()];
    for n in nums {
        for (i, c) in n.chars().enumerate() {
            if c == '1' {
                ones[i] += 1;
            }
        }
    }
    return ones;
}

fn most_common_binary_digit(nums: &[&str], idx: usize) -> Option<usize> {
    let ones_count = nums
        .iter()
        .filter(|n| n.chars().nth(idx).unwrap() == '1')
        .count();
    if nums.len() % 2 == 0 && ones_count == nums.len() / 2 {
        None
    } else if ones_count > nums.len() / 2 {
        Some(1)
    } else {
        Some(0)
    }
}
