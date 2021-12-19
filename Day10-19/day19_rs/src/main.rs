use itertools::iproduct;
use std::{collections::BTreeSet, ops::Add, ops::Sub};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Beacon(i32, i32, i32);

impl Add for Beacon {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Beacon(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Sub for Beacon {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Beacon(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

#[derive(Clone, Copy)]
enum Axis {
    X,
    Y,
    Z,
}

impl Beacon {
    fn rotate_around(&self, axis: Axis) -> Beacon {
        let (x, y, z) = (self.0, self.1, self.2);
        match axis {
            Axis::X => Beacon(x, z, -y),
            Axis::Y => Beacon(-z, y, x),
            Axis::Z => Beacon(y, -x, z),
        }
    }

    fn distance_to(&self, other: &Self) -> u32 {
        let (x, y, z) = (self.0, self.1, self.2);
        let (ox, oy, oz) = (other.0, other.1, other.2);
        let (dx, dy, dz) = ((x - ox).abs(), (y - oy).abs(), (z - oz).abs());
        (dx + dy + dz) as u32
    }
}

fn parse_beacon(s: &str) -> Beacon {
    let pieces = s
        .split(',')
        .map(|p| p.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    Beacon(pieces[0], pieces[1], pieces[2])
}

impl Beacon {}

fn main() {
    let scanners = include_str!("input.txt").split("\n\n");
    let mut scanners = scanners
        .into_iter()
        .map(|chunk| {
            chunk
                .trim()
                .lines()
                .skip(1)
                .map(parse_beacon)
                .collect::<BTreeSet<_>>()
        })
        .collect::<Vec<_>>();

    let mut aligned_scanners: BTreeSet<Beacon> = BTreeSet::new();
    aligned_scanners.insert(Beacon(0, 0, 0));

    let mut aligned_beacons = scanners.pop().unwrap();
    'outer: while let Some(mut curr_scanner) = scanners.pop() {
        for i in 0..6 {
            for _ in 0..4 {
                for &beacon in curr_scanner.iter() {
                    let ab_copy = aligned_beacons.iter();
                    for &b in ab_copy {
                        let delta = b - beacon;
                        let mut shift = curr_scanner
                            .iter()
                            .map(|b| *b + delta)
                            .collect::<BTreeSet<_>>();
                        let aligned = shift
                            .iter()
                            .filter(|b| aligned_beacons.contains(b))
                            .take(12)
                            .count();
                        if aligned >= 12 {
                            aligned_beacons.append(&mut shift);
                            aligned_scanners.insert(delta);
                            continue 'outer;
                        }
                    }
                }
                curr_scanner = curr_scanner
                    .iter()
                    .map(|b| b.rotate_around(Axis::X))
                    .collect();
            }
            let mut rotation_axes = vec![];
            if i < 3 {
                rotation_axes.push(Axis::Y)
            }
            if i > 2 && i < 5 {
                rotation_axes.push(Axis::Z);
            }
            if i == 4 {
                rotation_axes.push(Axis::Z);
            }
            for axis in rotation_axes.iter().rev() {
                curr_scanner = curr_scanner
                    .iter()
                    .map(|b| b.rotate_around(*axis))
                    .collect();
            }
        }
        scanners.insert(0, curr_scanner);
    }
    println!("Part 1: {}", aligned_beacons.len());
    let part2 = iproduct![aligned_scanners.iter(), aligned_scanners.iter()]
        .map(|(b1, b2)| b1.distance_to(b2))
        .max()
        .unwrap();
    println!("Part 2: {}", part2);
}
