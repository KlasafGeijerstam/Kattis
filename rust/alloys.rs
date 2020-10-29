mod fast_input;
use fast_input::{Str, FastInput};

fn main() {
    let inp = FastInput::new();
    let n: f64 = inp.next();
    println!("{}", (f64::min(1.0, n) / 2.0).powf(2.0));
}
