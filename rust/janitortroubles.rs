mod fast_input;
use fast_input::FastInput;

fn main() {
    let inp = FastInput::new();

    let (a, b, c, d) = inp.next_quad::<f64, f64, f64, f64>();
    let s = (a + b + c + d) / 2.0;
    println!("{}", ((s - a) * (s - b) * (s - c) * (s - d)).sqrt());
}
