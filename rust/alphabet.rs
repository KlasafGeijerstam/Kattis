mod inp;
mod long_inc_subseq;
use long_inc_subseq::longest_increasing_subsequence;

use inp::Inp;

fn main() {
    let mut inp = Inp::new();
    println!("{}", 26 - longest_increasing_subsequence(&inp.next_line().trim().as_bytes()));
}
