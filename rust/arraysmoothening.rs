mod fast_input;
use fast_input::FastInput;
use std::collections::{BinaryHeap, HashMap};

fn main() {
    let inp = FastInput::new();
    let (_, k): (u32, u32) = inp.next_tuple();
    let mut occ = HashMap::<u32, u32>::new();
    for v in inp.next_as_iter() {
        *occ.entry(v).or_default() += 1;
    }
    let mut values: BinaryHeap<_> = occ.into_values().collect();
    for _ in 0..k {
        if let Some(v) = values.pop() {
            if v > 1 {
                values.push(v - 1);
            }
        }
    }
    println!("{}", values.pop().unwrap_or(0));
}
