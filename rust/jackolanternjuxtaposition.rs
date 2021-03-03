mod fast_input;
use fast_input::FastInput;

fn main() {
    let inp = FastInput::new();

    let sum = inp.next_as_iter::<u64>().product::<u64>();
    println!("{}", sum);
}
