mod fast_input;
use fast_input::FastInput;
use std::collections::HashSet;

fn main() {
    let inp = FastInput::new();
    let l = inp.next_line();
    let distinct: HashSet<_> = l.chars().collect();
    println!("{}", (l.len() == distinct.len()) as u32);
}

