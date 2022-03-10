mod fast_input;

use std::cmp::Reverse;
use crate::fast_input::Str;
use std::collections::BinaryHeap;

use fast_input::FastInput;

#[derive(Clone, Copy)]
struct Path {
    to: usize,
    length: u64,
    outside: bool,
}

fn main() {
    let inp = FastInput::new();
    let (n, m, p) = inp.next_triple();
    
    let mut paths = vec![Vec::<Path>::new(); n];
    
    for _ in 0..m {
        let (from, to, length, k): (usize, usize, u64, Str) = inp.next_quad();
        paths[from].push(Path {
            to,
            length,
            outside: *k == "O"
        });

        paths[to].push(Path {
            to: from,
            length,
            outside: *k == "O"
        });
    }

    let mut queue = BinaryHeap::with_capacity(800_000);
    'case: for _ in 0..p {
        queue.clear();
        let (from, to): (usize, usize) = inp.next_tuple();
        let mut costs = vec![(std::u64::MAX, std::u64::MAX); n];
        costs[from] = (0, 0);

        queue.push((Reverse(0), Reverse(0), from));
        
        while let Some((Reverse(cost_outside), Reverse(cost_inside), building)) = queue.pop() {
            if building == to {
                let total = cost_outside + cost_inside;
                println!("{} {}", cost_outside, total);
                continue 'case;
            }

            for path in &paths[building] {
                let outside = cost_outside + if path.outside { path.length } else { 0 };
                let inside = cost_inside + if !path.outside { path.length } else { 0 };

                if outside < costs[path.to].0 || outside == costs[path.to].0 && inside < costs[path.to].1 {
                    costs[path.to] = (outside, inside);
                    queue.push((Reverse(outside), Reverse(inside), path.to));
                }
            }
        }
        println!("IMPOSSIBLE");
    }
}
