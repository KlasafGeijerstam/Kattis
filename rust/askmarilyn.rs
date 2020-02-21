use std::io;

fn main() -> io::Result<()> {
    let mut z: u64 = 20170705;
    let a: u64 = 742938285;
    let m: u64 = 21700000;
    let mut k = String::new();

    for _ in 0..1000 {
        z = a * z % m;
        let ch =  ((z % 3) as u8 + 65) as char;
        println!("{}", ch);

        io::stdin().read_line(&mut k)?;
        let mut c = k.chars();
        let d = c.next().unwrap();
        let i = c.skip(1).next().unwrap();
        
        if i == '1' {
            println!("{}", d);
        } else {
            println!("{}", ((198 - ch as u8) - d as u8) as char);
        }
        io::stdin().read_line(&mut k)?;

        k.clear();
    }

    Ok(())
}
