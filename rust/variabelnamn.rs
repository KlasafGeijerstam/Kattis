mod fast_input;
use fast_input::FastInput;
use std::io::{Write, BufWriter, stdout};
use std::collections::{HashMap, HashSet};

fn main() -> std::io::Result<()> {
    let inp = FastInput::new();
    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());

    let mut taken = HashSet::new();
    let mut last_stop = HashMap::new();

    for _ in 0..inp.next() {
        let v = inp.next(); 
        let start = *last_stop.get(&v).unwrap_or(&v);
        for i in (start..).step_by(v) {
            if !taken.contains(&i) {
                writeln!(writer, "{}", i)?;
                taken.insert(i);
                last_stop.insert(v, i);
                break
            }
        }
    }

    Ok(())
}
