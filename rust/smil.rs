mod fast_input;
use fast_input::FastInput;
use States::*;

enum States {
    No,
    Eyes(usize),
    Nose(usize),
    Mouth(usize),
}

impl States {
    fn next(&self, c: char, index: usize) -> Self {
        match (self, c) {
            (No, ':') | (No, ';') => Eyes(index),
            (Eyes(i), '-') => Nose(*i),
            (Eyes(i), ')') => Mouth(*i),
            (Eyes(_), ':') | (Eyes(_), ';') => Eyes(index),
            (Nose(i), ')') => Mouth(*i),
            _ => No
        }
    }
}

fn main() {
    let inp = FastInput::new();
    inp.next_line().chars().enumerate().fold(No, |state, (i, c)| {
        let new_state = state.next(c, i);
        if let Mouth(start) = new_state {
            println!("{}", start);
            No
        } else {
            new_state
        }
    });
}
