mod fast_input;
use fast_input::{Str, FastInput};
use std::collections::HashMap;
use std::io::{prelude::*, BufWriter, stdout};

fn main() {
    let inp = FastInput::new();
    let mut friends: HashMap<&str, (&str, u32)> = HashMap::new();
    for _ in 0..inp.next() {
        let (name, like, date) = inp.next_triple::<Str, u32, Str>();
        friends.entry(*date)
            .and_modify(|e| {
                if e.1 < like {
                    e.1 = like; 
                    e.0 = *name;
                }
            })
            .or_insert((*name, like));
    }

    let stdout = stdout();

    let mut writer = BufWriter::new(stdout.lock());
    writeln!(writer, "{}", friends.len()).ok();
    let mut names: Vec<_> = friends.values().map(|(n, _)| n).collect();
    names.sort_unstable();
    for n in names {
        writeln!(writer, "{}", n).ok();
    }

}
