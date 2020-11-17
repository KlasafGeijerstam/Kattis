mod fast_input;
use fast_input::{Str, FastInput};
use std::iter::repeat;

fn main() {
    let inp = FastInput::new();
    let l = inp.next_line();
    println!("h{}y", repeat('e').take((l.len() - 2) * 2).collect::<String>());
}
