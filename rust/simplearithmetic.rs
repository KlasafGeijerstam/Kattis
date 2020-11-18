mod fast_input;
use fast_input::{Str, FastInput};
use std::collections::HashMap;
use std::io::{prelude::*, BufWriter, stdout};
use std::iter::repeat;

fn main() {
    let inp = FastInput::new();
    let (a, b, c) = inp.next_triple::<u64, u64, u64>();
    let mut ab = a * b;
    if ab >= 10 * c {
        print!("{}", ab / ( 10 * c));
        ab %= 10 * c; 
    }
    println!("{}", (ab as f64 / c as f64));
}
