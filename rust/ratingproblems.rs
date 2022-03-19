mod fast_input;
use fast_input::FastInput;

fn main() {
    let inp = FastInput::new();
    let (n, k): (f64, f64) = inp.next_tuple();
    let rating: f64 = inp.lines().map(|l| l.parse::<f64>().unwrap()).sum();
    println!("{} {}", (rating + -3.0 * (n - k)) / n, (rating + 3.0 * (n - k)) / n);
}
