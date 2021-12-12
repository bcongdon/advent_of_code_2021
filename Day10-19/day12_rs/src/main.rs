use std::collections::HashMap;

fn num_paths<'a>(
    node: &'a str,
    edges: &HashMap<&'a str, Vec<&'a str>>,
    path: &mut Vec<&'a str>,
    allow_twice: bool,
) -> usize {
    if node == "end" {
        return 1;
    }

    let mut child_paths = 0;
    for &neighbor in &edges[node] {
        if neighbor == "start" {
            continue;
        }
        let is_small_cave = neighbor.to_lowercase() == neighbor;
        let already_visited_small_cave = is_small_cave && path.contains(&neighbor);
        if already_visited_small_cave && !allow_twice {
            continue;
        }

        path.push(neighbor);
        if allow_twice && already_visited_small_cave {
            child_paths += num_paths(neighbor, edges, path, false);
        } else {
            child_paths += num_paths(neighbor, edges, path, allow_twice);
        }
        path.pop();
    }
    child_paths
}

fn main() {
    let edges_raw = include_str!("input.txt").lines().map(|l| {
        let mut pieces = l.trim().split('-');
        (pieces.next().unwrap(), pieces.next().unwrap())
    });

    let mut edges = HashMap::<&str, Vec<&str>>::new();
    for (a, b) in edges_raw {
        edges.entry(a).or_default().push(b);
        edges.entry(b).or_default().push(a);
    }
    println!(
        "Part 1: {}",
        num_paths("start", &edges, &mut Vec::new(), false)
    );
    println!(
        "Part 2: {}",
        num_paths("start", &edges, &mut Vec::new(), true)
    );
}
