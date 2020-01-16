use std::io;

fn main() -> io::Result<()> {
    let na = read_ints()?;
    let mut p = read_ints()?;
    p.sort();
    let mut won = 0;
    let mut s = na[1];
    for i in p {
        if s - (i + 1) > -1 {
            s -= i + 1;
            won += 1;
        } else {
            break;
        }
    }
    println!("{}", won);
    Ok(())
}

fn read_ints() -> Result<Vec<i32>, io::Error> {
    let mut k = String::new();
    io::stdin().read_line(&mut k)?;
    Ok(k.trim().split(" ").map(|x| x.parse().unwrap()).collect())
}
