use std::io;

fn main() -> io::Result<()> {
    let l = read_ints()?;
    if l[0] == 1 {
        println!("1");
        return Ok(());
    }
    let mut p: Vec<(i32, i32)> = read_ints()?
        .iter()
        .zip(2..l[0] + 1)
        .map(|x| (x.0 + 0, x.1))
        .collect();
    p.sort_by_key(|x| x.0);
    let mut x = String::from("1");
    for i in p {
        x.push(' ');
        x.push_str(&i.1.to_string());
    }
    println!("{}", x);
    Ok(())
}

fn read_ints() -> Result<Vec<i32>, io::Error> {
    let mut k = String::new();
    io::stdin().read_line(&mut k)?;
    Ok(k.trim().split(" ").map(|x| x.parse().unwrap()).collect())
}
