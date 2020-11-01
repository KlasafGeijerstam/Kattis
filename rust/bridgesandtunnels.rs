mod fast_input;
use fast_input::{Str, FastInput};
mod union_find;
use union_find::UnionFind;
use std::collections::HashMap;
use std::io::{stdout, Write, BufWriter};

fn main() {
    let inp = FastInput::new();
    let n = inp.next(); 
    let mut bmap = HashMap::with_capacity(200_000);
    let mut size = vec![1; 200_000];
    let mut uf = UnionFind::new(200_000);
    
    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());

    for _ in 0..n {
        let (b1, b2) = inp.next_tuple::<Str, Str>();
        let l = bmap.len();
        let b1 = *bmap.entry(*b1).or_insert(l);
        let l = bmap.len();
        let b2 = *bmap.entry(*b2).or_insert(l);
        
        let b1root = uf.find(b1);
        let b2root = uf.find(b2);
        if b1root != b2root {
            let new_size = size[b1root] + size[b2root];
            size[b1root] = new_size;
            size[b2root] = new_size;
            uf.union(b1root, b2root);
        }
        let bsize = size[uf.find(b1)];
        writeln!(writer, "{}", bsize).ok();
    }
}

