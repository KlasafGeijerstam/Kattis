mod fast_input;
use fast_input::FastInput;

fn main() {
    let inp = FastInput::new();
    let s = inp.next_line();
    println!("{}", &s[s.find('a').unwrap()..]);
}