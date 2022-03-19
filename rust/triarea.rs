mod fast_input;
use fast_input::FastInput;

fn main() {
    let inp = FastInput::new();
    let (a, b): (f64, f64) = inp.next_tuple();
    println!("{}", (a * b) / 2.0);
}
