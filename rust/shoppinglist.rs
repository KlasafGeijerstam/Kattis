mod fast_input;

use fast_input::FastInput;
use std::collections::{HashSet, HashMap};

use std::io::{stdout, BufWriter, Write};

fn main() {
    let inp = FastInput::new();
    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());
    let mut ingredients: HashMap<_, u16> = HashMap::new();
    let (n, _): (u16, u16) = inp.next_tuple();
    let mut list = HashSet::new();
    for _ in 0..n {
        list.extend(inp.next_split());
        list.drain().for_each(|ingredient| {
            *ingredients.entry(ingredient).or_default() += 1;
        });
    }
    let mut popular: Vec<_> = ingredients.drain().filter(|&(_, k)| k == n).map(|(item, _)| item).collect();
    popular.sort_unstable();
    writeln!(writer, "{}", popular.len()).ok(); 
    for item in popular {
        writeln!(writer, "{}", item).ok();
    }
}
