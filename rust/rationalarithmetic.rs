mod fast_input;
mod rational;
use fast_input::{Str, FastInput};
use std::io::Result;
use rational::Rational;

fn main() -> Result<()> {
    type Line<'a> = (i64, i64, Str<'a>, i64, i64);
    let inp = FastInput::new();
    let n = inp.next(); 
    for _ in 0..n {
        let (n1, d1, op, n2, d2): Line = inp.next_quintuple();
        let r1 = Rational::new(n1, d1);
        let r2 = Rational::new(n2, d2);
        let ans = match *op {
            "+" => r1 + r2,
            "-" => r1 - r2,
            "*" => r1 * r2,
            _ => r1 / r2,
        };

        println!("{} / {}", ans.num, ans.den);
    }
    Ok(())
}

