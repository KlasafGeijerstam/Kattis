mod fast_input;
use fast_input::{Str, FastInput};
use std::collections::HashMap;
use std::io::{prelude::*, BufWriter, stdout};

fn main() {
    let inp = FastInput::new();
    let mut map = HashMap::new();
    map.insert('B', 1);
    map.insert('F', 1);
    map.insert('P', 1);
    map.insert('V', 1);

    map.insert('C', 2);
    map.insert('G', 2);
    map.insert('J', 2);
    map.insert('K', 2);
    map.insert('Q', 2);
    map.insert('S', 2);
    map.insert('X', 2);
    map.insert('Z', 2);

    map.insert('D', 3);
    map.insert('T', 3);

    map.insert('L', 4);

    map.insert('M', 5);
    map.insert('N', 5);

    map.insert('R', 6);
    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());
    while inp.has_next_line() {
        let mut line: Vec<_> = inp.next_line()
            .chars()
            .collect();
        line.dedup_by(|a, b| map.contains_key(a) && map.contains_key(b) && map[a] == map[b]);

        let line: Vec<_> = line.iter().filter(|c| map.contains_key(&c))
            .map(|c| map[&c])
            .collect();

        for c in line {
            write!(writer, "{}", c).ok();
        }

        writeln!(writer).ok();
    }
}
