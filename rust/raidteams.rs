mod inp;
use inp::Inp;
use std::cmp::Ordering;
use std::collections::{HashSet, BinaryHeap};
use std::rc::Rc;
use std::io::{stdout, BufWriter, prelude::*};


#[derive(Eq)]
struct Player(pub Rc<String>, pub u32);

impl PartialOrd for Player {
    fn partial_cmp(&self, o: &Self) -> Option<Ordering> {
        Some(self.cmp(o))     
    }
}

impl Ord for Player {
    fn cmp(&self, o: &Self) -> Ordering {
        self.1.cmp(&o.1).then(o.0.cmp(&self.0))
    }
}

impl PartialEq for Player {
    fn eq(&self, o: &Self) -> bool {
        self.1 == o.1 && self.0 == o.0
    }
}



fn main() {
    let mut inp = Inp::new();
    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());
    let n = inp.next();
    let mut available = HashSet::new();
    let mut s1 = BinaryHeap::new();
    let mut s2 = BinaryHeap::new();
    let mut s3 = BinaryHeap::new();
    
    for _ in 0..n {
        let mut l = inp.next_line().trim().split_ascii_whitespace();
        let n: Rc<String> = Rc::new(l.next().unwrap().into()); 
        available.insert(n.clone());
        let k1 = l.next().unwrap().parse().unwrap();
        let k2 = l.next().unwrap().parse().unwrap();
        let k3 = l.next().unwrap().parse().unwrap();
        s1.push(Player(n.clone(), k1));
        s2.push(Player(n.clone(), k2));
        s3.push(Player(n, k3));
    }

    loop {
        let mut peps = vec![]; 
        while let Some(p) = s1.pop() {
            if available.contains(&p.0) {
                available.remove(&p.0);
                peps.push(p.0);
                break
            }
        }
        while let Some(p) = s2.pop() {
            if available.contains(&p.0) {
                available.remove(&p.0);
                peps.push(p.0);
                break
            }
        }
        while let Some(p) = s3.pop() {
            if available.contains(&p.0) {
                available.remove(&p.0);
                peps.push(p.0);
                break
            }
        }
        
        if peps.len() == 3 {
            peps.sort();
            writeln!(writer, "{} {} {}", *peps[0], *peps[1], *peps[2]).unwrap();
        } else {
            break
        }
    }
}
