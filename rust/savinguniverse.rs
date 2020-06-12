use std::collections::HashSet;
use std::io::prelude::*;
use std::io::{self, StdinLock};

fn main() -> io::Result<()> {
    let mut buf = String::with_capacity(100);
    let mut vec = Vec::with_capacity(4);

    let stdin = io::stdin();
    let mut l = stdin.lock();
    read::<i32>(&mut l, &mut buf, &mut vec)?;
    let t = vec[0];

    for case in 0..t {
        read::<i32>(&mut l, &mut buf, &mut vec)?;
        let mut used: HashSet<String> = HashSet::new();
        let engine_count = vec[0] as usize;
        (0..vec[0])
            .for_each(|_| {
                buf.clear();
                l.read_line(&mut buf).unwrap();
            });

        read::<i32>(&mut l, &mut buf, &mut vec)?;
        let searches: Vec<String> = (0..vec[0])
            .map(|_| {
                buf.clear();
                l.read_line(&mut buf).unwrap();
                buf.clone()
            })
            .collect();
        let mut switches = 0;
        for i in searches {
            if !used.contains(&i) {
                if used.len() == engine_count -1 {
                    switches += 1;
                    used.clear();
                }
                used.insert(i.clone());
            }
        }
        println!("Case #{}: {}", case + 1, switches);
    }

    Ok(())
}

fn read<T>(stdin: &mut StdinLock, s: &mut String, v: &mut Vec<T>) -> io::Result<()>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    s.clear();
    v.clear();
    stdin.read_line(s)?;
    v.extend(
        s.trim()
            .split_ascii_whitespace()
            .map(|x| x.parse().unwrap()),
    );
    Ok(())
}
