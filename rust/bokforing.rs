mod fast_input;
use fast_input::{FastInput};
use std::io::{Write, BufWriter, stdout};

fn main() {
    let stdout = stdout();
    let mut w = BufWriter::new(stdout.lock());
    let inp = FastInput::new();
    let (n, q) = inp.next_tuple();
    let mut time = 0; 
    let mut reset_time = 0;
    let mut reset_amount = 0;

    let mut people = vec![(0, 0); n];
    for _ in 0..q {
        time += 1;
        let mut i = inp.next_split();
        let instr = i.next().unwrap();
        if instr.starts_with("S") {
            let person: usize = i.next().unwrap().parse().unwrap();
            let amount = i.next().unwrap().parse().unwrap();
            people[person - 1] = (amount, time);
        } else if instr.starts_with("P") {
            let person: usize = i.next().unwrap().parse().unwrap();
            if people[person - 1].1 > reset_time {
                writeln!(w, "{}", people[person - 1].0).ok();
            } else {
                writeln!(w, "{}", reset_amount).ok();
            }
        } else {
            reset_amount = i.next().unwrap().parse().unwrap();
            reset_time = time;
        }

    }
}
