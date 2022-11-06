mod fast_input;

use fast_input::{FastInput, FastParse};
use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct Candidate {
    has_idol: bool,
    id: usize,
    distances: Vec<i32>,
}

impl Candidate {
    fn new(id: usize, has_idol: bool) -> Self {
        Self {
            has_idol,
            id,
            distances: vec![std::i32::MAX; 10_000],
        }
    }
}

#[derive(Debug)]
struct Cave {
    has_idol: bool,
    edges: Vec<(usize, i32)>,
}

fn max_idols(current_cave: usize, caves: &[Cave], used: &mut [bool; 9], air: i32, idols: u8) -> u8 {
    let mut best = 0;
    if current_cave == 0 {
        best = idols;
    }

    for &(to, cost) in caves[current_cave].edges.iter() {
        if !used[to] && to != current_cave && air >= cost {
            used[to] = true;
            let next_idols = idols + if caves[to].has_idol { 1 } else { 0 };
            best = best.max(max_idols(to, caves, used, air - cost, next_idols));
            used[to] = false;
        }
    }

    best
}

fn main() {
    let inp = FastInput::new();

    let mut tunnels: Vec<Vec<(usize, i32)>> = vec![Vec::new(); 10_000];
    let mut queue = BinaryHeap::with_capacity(10_000);

    for _ in 0..inp.next_parsed() {
        let (_, m): (u32, u32) = inp.next();

        for _ in 0..m {
            let (a, b, l): (usize, usize, i32) = inp.next();
            tunnels[a].push((b, l));
            tunnels[b].push((a, l));
        }

        let _ = inp.next_parsed::<usize>();

        let mut caves: Vec<_> = inp
            .next_as_iter()
            .map(|p| Candidate::new(p, true))
            .collect();

        if caves.iter().find(|c| c.id == 0).is_none() {
            caves.push(Candidate::new(0, false));
        }

        let start_has_idol = if let Some(cave) = caves.iter_mut().find(|c| c.id == 0) {
            if cave.has_idol {
                cave.has_idol = false;
                true
            } else {
                false
            }
        } else {
            false
        };

        caves.sort_by_key(|c| c.id);

        let air = inp.next_parsed();

        let caves_len = caves.len();
        for start_index in 0..caves_len {
            let start_id = caves[start_index].id;
            for end_index in (start_index + 1)..caves_len {
                let end_id = caves[end_index].id;

                caves[start_index].distances[start_id] = 0;

                queue.push((Reverse(0), start_id));

                while let Some((Reverse(used_air), node)) = queue.pop() {
                    if used_air > caves[start_index].distances[node] {
                        continue;
                    }
                    caves[start_index].distances[node] = used_air;

                    if node == end_id {
                        let cost = caves[start_index].distances[end_id];
                        caves[end_index].distances[start_id] = cost;
                        break;
                    }

                    for &(to, cost) in tunnels[node].iter() {
                        if used_air + cost <= caves[start_index].distances[to] {
                            caves[start_index].distances[to] = used_air + cost;
                            queue.push((Reverse(used_air + cost), to));
                        }
                    }
                }

                queue.clear();
            }
        }

        let mut caves: Vec<_> = caves
            .iter()
            .map(|cave| Cave {
                has_idol: cave.has_idol,
                edges: caves
                    .iter()
                    .enumerate()
                    .map(|(i, c)| (i, cave.distances[c.id]))
                    .collect(),
            })
            .collect();

        println!(
            "{}",
            max_idols(
                0,
                &mut caves,
                &mut [false; 9],
                air,
                if start_has_idol { 1 } else { 0 }
            )
        );

        tunnels.iter_mut().for_each(Vec::clear);
    }
}
