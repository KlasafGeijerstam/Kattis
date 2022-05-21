mod fast_input;
use std::io::Write;
use fast_input::FastInput;
use std::io::{BufWriter, self};

#[derive(Default, Clone, Debug, Copy)]
struct Node {
    start: u32,
    end: u32,
    rank: u32,
}

impl Node {
    #[inline]
    const fn box_count(&self) -> u32 {
        (self.end - self.start) + 1
    }

    #[inline]
    fn is_within(&self, other: &Self) -> bool {
        self.start >= other.start && self.start <= other.end
    }
}

fn main() {
    let inp = FastInput::new();
    let n = inp.next();
    let mut nodes = vec![Node::default(); n]; 
    
    let mut references = vec![vec![]; n];
    let mut roots = vec![];
    
    for (i, value) in inp.next_as_iter::<usize>().enumerate() {
        if value > 0 {
            references[value - 1].push(i);
        } else {
            roots.push(i);
        }
    }
    
    let mut next_index = 0;
    for &root in &roots {
        next_index = process_tree(root, &mut nodes, &references, next_index, 0) + 1;
    }
    
    let stdout = io::stdout();
    let mut writer = BufWriter::new(stdout.lock());
    
    let mut queries = Vec::with_capacity(20);
    for _ in 0..inp.next() {
        queries.clear();
        queries.extend(inp.next_as_iter::<usize>()
            .skip(1)
            .map(|x| nodes[x - 1]));
        queries.sort_unstable_by_key(|n| n.rank);

        let mut sum = queries[0].box_count();
        for (i, &n) in queries.iter().enumerate().skip(1) {
            if queries[..i].iter().all(|x| !n.is_within(x)) {
                sum += n.box_count();
            }
        }
        writeln!(writer, "{}", sum).ok();
    }
}

fn process_tree(node: usize, nodes: &mut Vec<Node>, references: &Vec<Vec<usize>>, mut index: u32, rank: u32) -> u32 {
    nodes[node].start = index;
    nodes[node].rank = rank;
    for &child in &references[node] {
        index = process_tree(child, nodes, references, index + 1, rank + 1);
    }

    nodes[node].end = index;
    
    index
}

