use itertools::iproduct;

fn check_if_lands_in_target(
    mut vx: i32,
    mut vy: i32,
    tx: (i32, i32),
    ty: (i32, i32),
) -> (bool, i32) {
    let (mut x, mut y, mut max_y) = (0, 0, i32::MIN);
    while x <= tx.1 && y >= ty.0 {
        if tx.0 <= x && x <= tx.1 && ty.0 <= y && y <= ty.1 {
            return (true, max_y);
        }
        x += vx;
        y += vy;
        max_y = max_y.max(y);
        if vx > 0 {
            vx -= 1;
        } else if vx < 0 {
            vx -= 1;
        }
        vy -= 1;
    }
    (false, i32::MIN)
}

fn main() {
    let target_x = (25, 67);
    let target_y = (-260, -200);

    let (mut part1, mut part2) = (i32::MIN, 0);
    for (vx, vy) in iproduct![
        (0..=target_x.1).into_iter(),
        (target_y.0..-target_y.0).into_iter()
    ] {
        if let (true, max_y) = check_if_lands_in_target(vx, vy, target_x, target_y) {
            part1 = part1.max(max_y);
            part2 += 1;
        }
    }
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
