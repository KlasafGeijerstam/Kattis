mod fast_input;
use fast_input::FastInput;

fn main() {
    let inp = FastInput::new();
    let ans = (0..inp.next())
        .map(|_| inp.next::<i32>())
        .fold(1, |acc, cur| acc + cur - 1);
    println!("{}", ans);
}
