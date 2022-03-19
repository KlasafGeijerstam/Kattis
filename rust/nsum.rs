mod fast_input;
use fast_input::FastInput;

fn main() {
    let inp = FastInput::new();
    inp.next_line();
    println!("{}", inp.next_as_iter::<u32>().sum::<u32>());
}

