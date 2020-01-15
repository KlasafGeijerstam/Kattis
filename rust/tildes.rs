use std::io;
use std::io::prelude::*;
use std::io::BufWriter;

#[derive(Debug)]
struct UnionFind {
    data: Vec<usize>,
    rank: Vec<usize>,
    size: Vec<u32>
} 
impl UnionFind {
    fn new(size: u32) -> UnionFind {
        UnionFind {
            data: (0..size).map(|x| x as usize).collect(),
            rank: vec![0; size as usize],
            size: vec![1; size as usize]
        }
    }

    fn find(&mut self, i: usize) -> usize {
        let mut index = i;
        while index != self.data[index] {
            index = self.data[index];
        }

        let root = index;

        index = i;
        while index != self.data[index] {
            let tmp = self.data[index];
            self.data[index] = root;
            index = tmp
        }
        root
    }

    fn union(&mut self, i: usize, j: usize) {
        let p1 = self.find(i);
        let p2 = self.find(j);

        if p1 == p2 {
            return;
        }

        if self.rank[p1] > self.rank[p2] {
            self.data[p2] = p1; 
            self.size[p1] += self.size[p2];
        } else if self.rank[p1] < self.rank[p2] {
            self.data[p1] = p2;
            self.size[p2] += self.size[p1];
        } else {
            self.rank[p1] += 1;
            self.data[p2] = p1;  
            self.size[p1] += self.size[p2];
        }
    }

    fn size_of(&mut self, i: usize) -> u32 {
        let root = self.find(i);
        self.size[root]
    }
}

fn main() -> io::Result<()> {
    let l = read_ints()?;
    let mut find = UnionFind::new(l[0]);
    let mut line = String::new(); 
    let mut buffer = BufWriter::new(io::stdout());
    let sin = io::stdin();
    let mut ilock = sin.lock();
    while ilock.read_line(&mut line).unwrap() > 0 {
        let o: Vec<&str> = line.trim().split(" ").collect();
        if o.len() == 3 {
            let p1: usize = o[1].parse().unwrap();
            let p2: usize = o[2].parse().unwrap();
            find.union(p1 - 1, p2 - 1);
        } else {
            let p1: usize = o[1].parse().unwrap();
            writeln!(buffer, "{}", find.size_of(p1 - 1))?;
        }
        line.clear();
    }
    Ok(())
}

fn read_ints() -> Result<Vec<u32>, io::Error> {
    let mut k = String::new();
    io::stdin().read_line(&mut k)?;
    Ok(k.trim().split(" ").map(|x| x.parse().unwrap()).collect())
}
