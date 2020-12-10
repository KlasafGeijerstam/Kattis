mod fast_input;
use fast_input::{Str, FastInput};
use std::collections::HashMap;
use std::io::{prelude::*, BufWriter, stdout};
use std::iter::repeat;

fn main() {
    let inp = FastInput::new();
    for i in 0.. {
        let n: u32 = inp.next();
        if n == 0 {
            break
        }

        if i != 0 {
            println!();
        }

        let mut min_l = 10000;
        let mut max_l = 0;
        for _ in 0..n {
            let mut s = String::with_capacity(1000); 
            let mut l = inp.next_line().split_ascii_whitespace();
            let mut dot = l.next().unwrap() == ".";
            let mut l = l.map(|x| x.parse::<usize>().unwrap());

            for i in l {
                let ch = if dot {
                    '.' 
                } else {
                    '#'
                };
                s.extend(repeat(ch).take(i));
                dot = !dot;
            }
            min_l = usize::min(min_l, s.len());
            max_l = usize::max(max_l, s.len());
            println!("{}", s);
        }
        if min_l != max_l {
            println!("Error decoding image");
        }
    }
}
