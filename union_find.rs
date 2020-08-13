pub struct UnionFind {
    parent: Vec::<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    pub fn new(size: usize) -> Self {

        UnionFind {
            parent: (0..size).collect(),
            rank: vec![0; size],
        }
    }

    pub fn find(&mut self, mut item: usize) -> usize {
        let mut root = item;

        while root != self.parent[root] {
            root = self.parent[root];
        }

        while item != root {
            let parent = self.parent[item];   
            self.parent[item] = root;
            item = parent;
        }

        root
    }

    pub fn union(&mut self, left: usize, right: usize) {
        let rl = self.find(left);
        let rr = self.find(right);
        
        if rl == rr {
            return;
        }

        if self.rank[rl] < self.rank[rr] {
            self.parent[rl] = rr;
        } else {
            self.parent[rr] = rl;
        }
    
    }
}
