mod fast_input;
use fast_input::FastInput;
use std::collections::HashMap;

fn main() {
    let inp = FastInput::new();
    let odd = inp
        .next_line()
        .chars()
        .fold(HashMap::<char, u32>::new(), |mut occ, c| {
            *occ.entry(c).or_default() += 1;
            occ
        })
        .values()
        .filter(|&&o| o & 1 == 1)
        .count() as i32;
    println!("{}", (odd - 1).max(0));
}
