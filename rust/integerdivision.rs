use std::io;
use std::collections::HashMap;

fn main() -> io::Result<()> {
    let p = read_ints()?;
    let nums = read_ints()?;
    let mut a: HashMap<i32, (i32, u64)> = HashMap::with_capacity(p[0] as usize);
    for n in nums {
        let mut e = a.entry(n / p[1]).or_insert((0, 0));
        e.1 = e.1 + e.0 as u64;
        e.0 += 1;
    }
    println!("{}", a.values().fold(0, |acc, cur| acc + cur.1));
    Ok(())
}

fn read_ints() -> Result<Vec<i32>, io::Error> {
    let mut k = String::new();
    io::stdin().read_line(&mut k)?;
    Ok(k.trim().split(" ").map(|x| x.parse().unwrap()).collect())
}
