mod fast_input;
use fast_input::FastInput;
use std::iter::repeat;

fn main() {
    let inp = FastInput::new();
    let n: usize = inp.next();
    let m: usize = inp.next();
    let mut rooms = vec![m / n; n];
    for i in 0..m % n {
        rooms[i] += 1;
    }
    rooms.iter().map(|&n| repeat('*').take(n).collect::<String>())
        .for_each(|l| println!("{}", l));
}
