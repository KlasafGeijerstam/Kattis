mod inp;
use inp::Inp;
use std::collections::BinaryHeap;
use std::io::Result;

mod union_find;
use union_find::UnionFind;

use std::cmp::Ordering;
use std::io::{BufWriter, Write};

#[derive(Eq, Clone, Copy)]
struct Edge {
    pub from: usize,
    pub to: usize,
    pub weight: i32,
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.weight.partial_cmp(&self.weight)
    }
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        other.weight.cmp(&self.weight)
    }
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.weight == other.weight
    }
}

impl Edge {
    pub fn new(from: usize, to: usize, weight: i32) -> Self {
        Edge { from, to, weight }
    }
}

fn main() -> Result<()> {
    let mut inp = Inp::new();
    let stdout = std::io::stdout();
    let mut writer = BufWriter::new(stdout.lock());

    loop {
        let (n, m) = inp.next_tuple();

        if n + m == 0 {
            break;
        }

        let mut cost = 0;
        let mut tree = Vec::new();
        let mut edges = BinaryHeap::new();

        for _ in 0..m {
            let (u, v, w) = inp.next_triple::<i32>();
            if u < v {
                edges.push(Edge::new(u as usize, v as usize, w));
            } else {
                edges.push(Edge::new(v as usize, u as usize, w));
            }
        }


        let mut uf = UnionFind::new(n);        
    
        while let Some(e) = edges.pop() {
            if uf.find(e.from) != uf.find(e.to) {
                tree.push((e.from, e.to));

                cost += e.weight;

                uf.union(e.from, e.to);
            }
        }

        tree.sort();

        if tree.len() == n - 1 {
            writeln!(writer, "{}", cost)?;
            for (u, v) in tree {
                writeln!(writer, "{} {}", u, v)?;
            }
        } else {
            writeln!(writer, "Impossible")?;
        }
    }
    Ok(())
}
