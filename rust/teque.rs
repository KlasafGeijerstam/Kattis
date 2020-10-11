mod inp;
mod teque_impl;
use teque_impl::Teque;

use inp::Inp;
use std::io::{BufWriter, Result};
use std::io::prelude::*;

fn main() -> Result<()> {
    let mut inp = Inp::new();
    let mut teque = Teque::new();
    let n = inp.next();
    let stdout = std::io::stdout();
    let mut w = BufWriter::new(stdout.lock());
    for _ in 0..n {
        let mut l = inp.next_line().trim().split_ascii_whitespace();
        let c = l.next().unwrap();
        let n = l.next().unwrap().parse().unwrap();
        if c.len() < 5 {
            writeln!(w, "{}", teque.get(n as usize))?;
        } else if c.as_bytes()[5] == 'b' as u8 {
            teque.push_back(n);
        } else if c.as_bytes()[5] == 'f' as u8 {
            teque.push_front(n);
        } else {
            teque.push_middle(n);
        }
    }

    Ok(())
}
