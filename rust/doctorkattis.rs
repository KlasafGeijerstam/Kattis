mod inp;
use inp::Inp;
use std::collections::{BinaryHeap, HashMap};
use std::cmp::{Ord, Ordering};



use std::io::Result;
use std::io::prelude::*;
use std::io::BufWriter;

#[derive(Debug)]
struct Cat {
    id: u32,
    name: String,
    severity: u16,
    time: u32,
}

impl Clone for Cat {
    fn clone(&self) -> Self {
        Cat {
            name: self.name.clone(),
            ..*self
        }
    }
}

impl Cat {
    fn new(id: u32, name: String, severity: u16) -> Self {
        Cat {
            id,
            name,
            severity,
            time: id,
        }
    }

    fn update(&mut self, new_id: u32, amount: u16) {
        self.id = new_id;
        self.severity += amount;
    }

    fn invalidate(&mut self) {
        self.id = core::u32::MAX;
    }
}

impl PartialEq for Cat {
    fn eq(&self, other: &Self) -> bool {
        self.severity == other.severity
    }
}

impl Eq for Cat {}

impl Ord for Cat {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.severity > other.severity {
            Ordering::Greater
        } else if other.severity > self.severity {
            Ordering::Less
        } else if self.time < other.time {
            Ordering::Greater
        } else if self.time > other.time {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }
}

impl PartialOrd for Cat {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() -> Result<()> {
    let mut inp = Inp::new(); 
    let stdout = std::io::stdout();
    let mut writer = BufWriter::new(stdout.lock());
    let n = inp.next();
    let mut cats = HashMap::new();
    let mut cat_prio = BinaryHeap::new();
    let mut cat_counter = 0;

    'query: for _ in 0..n {
        let mut itr = inp.next_as_iter_raw();
        match itr.next().unwrap().parse().unwrap() {
            0 => {
                let name = itr.next().unwrap();  
                let severity = itr.next().unwrap().parse().unwrap();  
                let cat = Cat::new(cat_counter, name.into(), severity);
                cat_counter += 1;
                cat_prio.push(cat.clone());
                cats.insert(name.to_owned(), cat);
            },
            1 => {
                let cat = itr.next().unwrap();  
                let severity = itr.next().unwrap().parse().unwrap();  
                if severity == 0 {
                    continue;
                }
                cats.get_mut(cat).unwrap().update(cat_counter, severity);
                cat_counter += 1;
                cat_prio.push(cats[cat].clone());
            },
            2 => {
                let cat = itr.next().unwrap();  
                cats.get_mut(cat).unwrap().invalidate();
            },
            _ => {
                while let Some(c) = cat_prio.peek() {
                    if c.id == cats[&c.name].id {
                        writeln!(writer, "{}", c.name)?;
                        continue 'query; 
                    }
                    cat_prio.pop();
                }
                writeln!(writer, "The clinic is empty")?;
            },
        }
    }
    writer.flush()?;
    Ok(())
}

