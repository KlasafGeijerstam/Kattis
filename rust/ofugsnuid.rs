mod fast_input;
use fast_input::{FastInput};
use std::io::{stdout, BufWriter, Write};

fn main() {
    let inp = FastInput::new();
    let n = inp.next();
    let nums: Vec<i32> = (0..n).map(|_| inp.next()).collect();
    let io = stdout();
    let mut writer = BufWriter::new(io.lock());
    for n in nums.iter().rev() {
        writeln!(writer, "{}", n).ok();
    }
}
