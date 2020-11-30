mod fast_input;
mod long_inc_subseq;
use long_inc_subseq::*;

use fast_input::{FastInput};
use std::collections::HashSet;

fn main() {
    let inp = FastInput::new();
    let (n, m): (usize, usize) = inp.next_tuple();
    let mut people = vec![HashSet::new(); n];
    for _ in 0..m {
        let (i, j) = inp.next_tuple::<usize, usize>();
        people[i - 1].insert(j - 1);
    }
    let mut permutation = Vec::new(); 
    for i in 0..n {
        let mut before = Vec::new();
        for j in 0..i {
            if people[j].contains(&i) {
                before.push(j);
            }
        }

        if before.is_empty() {
            permutation.push(i); 
        } else {
            let mut min = std::usize::MAX;
            for b in &before {
                let p = permutation.iter()
                    .position(|x| x == b).unwrap();
                min = usize::min(min, p);
            }
            permutation.insert(min, i);
        }
    }
    permutation.reverse();
    println!("{}", longest_increasing_subsequence(&permutation));
    
}

