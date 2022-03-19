mod fast_input;
use fast_input::FastInput;

fn main() {
    let inp = FastInput::new();
    for l in inp.lines().step_by(2) {
        println!("{}", l);
    }
}
