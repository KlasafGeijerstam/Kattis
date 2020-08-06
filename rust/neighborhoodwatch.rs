use std::io;
use std::io::prelude::*;
use std::str::from_utf8_unchecked;

fn main() -> io::Result<()> {
    let mut buffer = Vec::with_capacity(400000);
    std::io::stdin().lock().read_to_end(&mut buffer)?;
    let mut lines = buffer.split(|x| x == &b'\n');
    let (n, k) = unsafe {
        let mut ln = from_utf8_unchecked(lines.next().unwrap()).split(' ').map(|x| x.parse::<usize>().unwrap());
        (ln.next().unwrap(), ln.next().unwrap())
    };

    let mut count: u64 = 0;
    let mut safe = vec![false; n];
    
    unsafe {
        for _ in 0..k {
            safe[from_utf8_unchecked(lines.next().unwrap()).parse::<usize>().unwrap() - 1] = true;
        }
    }

    let mut last_safe = n;
    if !safe.is_empty() {
        for i in (0..n).rev() {
            if safe[i] {
                last_safe = i;
            }
            count += (n - last_safe) as u64;
        }
    }

    println!("{}", count);

    Ok(())
}
