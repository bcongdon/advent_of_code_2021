use std::{
    collections::{BinaryHeap, HashMap},
    fmt::Display,
};

const NUM_ROOMS: usize = 4;
const NUM_HALLWAYS: usize = 7;

fn movement_weight(c: char) -> u64 {
    match c {
        'a' => 1,
        'b' => 10,
        'c' => 100,
        'd' => 1000,
        _ => panic!("invalid char"),
    }
}

fn hallway_dist(room_idx: usize, hallway_idx: usize) -> u64 {
    let exit_pos = (2 * room_idx) + 2;
    let hallway_pos = if hallway_idx > 1 {
        let rooms_skipped = hallway_idx - 1;
        hallway_idx + rooms_skipped
    } else {
        hallway_idx
    };
    return ((hallway_pos as i64) - (exit_pos as i64)).abs() as u64;
}

#[derive(Hash, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct State {
    rooms: [Vec<char>; NUM_ROOMS],
    hallway: [char; NUM_HALLWAYS],
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "State{{\n\thallway:{:?}\n\trooms:{:?}\n}}",
            self.hallway, self.rooms
        )
    }
}

impl State {
    fn completed(room_capacity: usize) -> State {
        State {
            rooms: [
                vec!['a'; room_capacity],
                vec!['b'; room_capacity],
                vec!['c'; room_capacity],
                vec!['d'; room_capacity],
            ],
            hallway: ['.'; NUM_HALLWAYS],
        }
    }

    fn neighbors(&self, max_room_capacity: usize) -> Vec<(State, u64)> {
        let mut neighbors = self.rooms_to_hallway(max_room_capacity);
        neighbors.append(&mut self.hallway_to_rooms(max_room_capacity));
        neighbors
    }

    fn rooms_to_hallway(&self, max_room_capacity: usize) -> Vec<(State, u64)> {
        let mut out = Vec::new();
        for room_idx in 0..NUM_ROOMS {
            let room_complete = self.rooms[room_idx]
                .iter()
                .all(|&c| (c as u8 - 'a' as u8) == room_idx as u8);
            if room_complete {
                continue;
            }

            let mut next_state = self.clone();
            let moving_amphipod = next_state.rooms[room_idx].pop().unwrap();
            let weight = movement_weight(moving_amphipod);
            let exit_dist = (max_room_capacity - next_state.rooms[room_idx].len()) as u64;
            let room_hallway_pos = room_idx + 2;
            // Check hallway spots to the left of the current room.
            for hallway_idx in (0..room_hallway_pos).rev() {
                let hallway_dist = hallway_dist(room_idx, hallway_idx);
                let cost = (exit_dist + hallway_dist) * weight;
                if next_state.hallway[hallway_idx] == '.' {
                    let mut next_state = next_state.clone();
                    next_state.hallway[hallway_idx] = moving_amphipod;
                    out.push((next_state, cost));
                } else {
                    break;
                }
            }
            // Check hallway spots to the right of the current room.
            for hallway_idx in room_hallway_pos..NUM_HALLWAYS {
                let hallway_dist = hallway_dist(room_idx, hallway_idx);
                let cost = (exit_dist + hallway_dist) * weight;
                if next_state.hallway[hallway_idx] == '.' {
                    let mut next_state = next_state.clone();
                    next_state.hallway[hallway_idx] = moving_amphipod;
                    out.push((next_state, cost));
                } else {
                    break;
                }
            }
        }

        out
    }

    fn hallway_to_rooms(&self, max_room_capacity: usize) -> Vec<(State, u64)> {
        let mut out = Vec::new();
        for hallway_idx in 0..NUM_HALLWAYS {
            let moving_amphipod = self.hallway[hallway_idx];
            if moving_amphipod == '.' {
                continue;
            }
            let dest_room_idx = (moving_amphipod as u8 - 'a' as u8) as usize;
            let room_needs_moveout = self.rooms[dest_room_idx]
                .iter()
                .any(|&c| (c as u8 - 'a' as u8) != dest_room_idx as u8);
            if room_needs_moveout {
                continue;
            }
            let (min_idx, max_idx) = if dest_room_idx + 2 > hallway_idx {
                (hallway_idx + 1, dest_room_idx + 1)
            } else {
                (dest_room_idx + 2, hallway_idx - 1)
            };
            let path_clear = (min_idx..=max_idx)
                .into_iter()
                .all(|idx| self.hallway[idx] == '.');
            if !path_clear {
                continue;
            }

            let hallway_dist = hallway_dist(dest_room_idx, hallway_idx);
            let entrance_dist = (max_room_capacity - self.rooms[dest_room_idx].len()) as u64;
            let cost = (hallway_dist as u64 + entrance_dist) * movement_weight(moving_amphipod);
            let mut next_state = self.clone();
            next_state.hallway[hallway_idx] = '.';
            next_state.rooms[dest_room_idx].push(moving_amphipod);
            out.push((next_state, cost));
        }
        out
    }
}

fn cost_to_rearrange(initial_state: State) -> u64 {
    let room_capacity = initial_state.rooms[0].len();
    let mut costs = HashMap::<State, i64>::new();
    let mut queue = BinaryHeap::new();
    costs.insert(initial_state.clone(), 0);
    queue.push((0_i64, initial_state));

    let completed = State::completed(room_capacity);
    while let Some((cost, curr)) = queue.pop() {
        if curr == completed {
            return -cost as u64;
        }
        let cost = -cost;
        if cost != costs[&curr] {
            continue;
        }
        for (neighbor, neighbor_cost) in curr.neighbors(room_capacity) {
            let new_cost = neighbor_cost as i64 + cost;
            if let Some(&prev_cost) = costs.get(&neighbor) {
                if prev_cost <= new_cost {
                    continue;
                }
            }
            costs.insert(neighbor.clone(), new_cost);
            queue.push((-new_cost, neighbor));
        }
    }

    panic!("No solution found")
}

fn main() {
    let initial_state = State {
        rooms: [
            vec!['c', 'b'],
            vec!['a', 'b'],
            vec!['a', 'd'],
            vec!['c', 'd'],
        ],
        hallway: ['.'; 7],
    };
    println!("Part 1: {}", cost_to_rearrange(initial_state));

    let initial_state = State {
        rooms: [
            vec!['c', 'd', 'd', 'b'],
            vec!['a', 'b', 'c', 'b'],
            vec!['a', 'a', 'b', 'd'],
            vec!['c', 'c', 'a', 'd'],
        ],
        hallway: ['.'; 7],
    };
    println!("Part 2: {}", cost_to_rearrange(initial_state));
}
