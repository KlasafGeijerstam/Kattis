mod fast_input;
use fast_input::FastInput;

fn main() {
    let inp = FastInput::new();
    println!("{}", inp.next_line().starts_with("555") as u32);
}
