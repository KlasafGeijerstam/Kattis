mod fast_input;
use fast_input::FastInput;

fn main() {
    let inp = FastInput::new();
    let ans: u32 = inp.next_as_iter::<u32>().sum();
    println!("{}", ans);
}
