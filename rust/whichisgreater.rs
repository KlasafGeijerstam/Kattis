mod fast_input;
use fast_input::FastInput;

fn main() {
    let inp = FastInput::new();
    let (a, b): (u32, u32) = inp.next_tuple();
    println!("{}", (a > b) as u32);
}
