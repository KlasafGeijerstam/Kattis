mod fast_input;
use fast_input::{Str, FastInput};
use std::collections::HashMap;
use std::io::{prelude::*, BufWriter, stdout};

fn main() {
    let inp = FastInput::new();
    let mut spies = vec![];
    let mut safes = vec![];
    for y in 0i32..inp.next() {
        for (x, c) in inp.next_line().chars().enumerate() {
            if c == 'H' {
                safes.push((x as i32, y));
            } else if c == 'S' {
                spies.push((x as i32, y)); 
            }
        }
    }
    
    let mut max = 0;
    for (sx, sy) in spies {
        let mut min = 10000;
        for (hx, hy) in &safes {
            min = i32::min(min, (sx - hx).abs() + (sy - hy).abs());
        }
        max = i32::max(max, min);
    }

    println!("{}", max);
}
