mod fast_input;
use fast_input::FastInput;
use std::{collections::HashMap, io::{BufWriter, stdout, Write}};

fn main() {
    let inp = FastInput::new();
    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());
    let mut scopes = vec![HashMap::<&str, Option<&str>>::new()];
    let mut scope = HashMap::new();
    for line in inp.lines().skip(1) {
        if line == "{" {
            scopes.push(HashMap::new());
        } else if line == "}" {
            for (key, prev_value) in scopes.pop().unwrap().drain() {
                if let Some(value) = prev_value {
                    scope.insert(key, value);
                } else {
                    scope.remove(key);
                }
            }
        } else if line.starts_with("D") {
            let mut tokens = line.split_ascii_whitespace();
            tokens.next();
            let (var, def) = (tokens.next().unwrap(), tokens.next().unwrap());
            let current_scope = scopes.last_mut().unwrap();
            if current_scope.contains_key(&var) {
                writeln!(writer, "MULTIPLE DECLARATION").ok();
                break;
            }
            current_scope.insert(var, scope.insert(var, def));
        } else {
            let var = line.split_ascii_whitespace().skip(1).next().unwrap();
            if let Some(def) = scope.get(var) {
                writeln!(writer, "{}", def).ok();
            } else {
                writeln!(writer, "UNDECLARED").ok();
            }
        }
    }
}
