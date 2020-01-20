use std::io;
use std::io::prelude::*;
use std::io::{BufReader};
use std::collections::LinkedList;

fn main() -> io::Result<()> {
    let mut input = BufReader::new(io::stdin());
    let mut buf = String::new();
    input.read_line(&mut buf)?;
    let n = buf.trim().parse::<usize>().unwrap();
    let mut v: Vec<LinkedList<usize>> = Vec::with_capacity(n);
    let mut lines: Vec<String> = Vec::with_capacity(n);

    for _ in 0..n {
        buf.clear();
        input.read_line(&mut buf)?;
        lines.push(buf.trim().to_owned());
    }

    for j in 0..n {
        v.push(LinkedList::new());
        v[j].push_front(j);
    }
    
    let mut a = 0;
    for _ in 0..(n-1) {
        buf.clear();
        input.read_line(&mut buf)?;
        let mut i = buf.split_ascii_whitespace().map(|x| x.parse::<usize>().unwrap()); 
        a = i.next().unwrap() - 1;
        let b = i.next().unwrap() - 1;
        let mut x = std::mem::replace(&mut v[b], LinkedList::new());
        v[a].append(&mut x);
    }

    
    let mut output = String::with_capacity(1000001);
    for i in v[a].iter() {
        output.push_str(&lines[*i]); 
    }
    println!("{}", output);
    
    Ok(())
}
