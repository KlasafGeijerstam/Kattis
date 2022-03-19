mod fast_input;
use fast_input::FastInput;

fn main() {
    let inp = FastInput::new();
    inp.next_line();
    let ans = inp.lines().fold(7, |volume, instr| match instr {
        "Skru op!" => (volume + 1).min(10),
        _ => (volume - 1).max(0)
    });

    println!("{}", ans);
}
