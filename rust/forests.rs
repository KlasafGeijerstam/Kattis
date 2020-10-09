mod inp;
use inp::Inp;
use std::collections::HashSet;

fn main() {
    let mut inp = Inp::new();
    let (p, _t) = inp.next_tuple();
    let mut peps = vec![0u128; p];
    while let Some((i, j)) = inp.get_next_tuple::<usize>() {
        peps[i - 1] |= 1u128 << (j - 1);
    }
    println!("{}", peps.iter().collect::<HashSet<&u128>>().len());
}

