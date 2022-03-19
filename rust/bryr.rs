mod fast_input;
use std::cmp::Reverse;
use fast_input::FastInput;
use std::collections::BinaryHeap;

fn main() {
    let inp = FastInput::new();
    let (n, m) = inp.next_tuple();
    let mut roads = vec![vec![]; n];
    for _ in 0..m {
        let (a, b, c): (usize, usize, u32) = inp.next_triple();
        roads[a - 1].push((b - 1, c));
        roads[b - 1].push((a - 1, c));
    }
    let mut distance = vec![std::u32::MAX; n];
    distance[n - 1] = 0;
    let mut nodes = BinaryHeap::new();
    nodes.push((Reverse(0), n - 1));
    while let Some((Reverse(cost), node)) = nodes.pop() {
        if node == 0 {
            break;
        }
        
        for &(other, c) in &roads[node] {
            if cost + c < distance[other] {
                distance[other] = cost + c;
                nodes.push((Reverse(cost + c), other));
            }
        }
    }

    println!("{}", distance[0]);
}
